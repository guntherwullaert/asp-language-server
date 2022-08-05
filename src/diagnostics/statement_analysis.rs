use std::{collections::HashSet, hash::Hash};

use dashmap::DashMap;
use log::info;
use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::{Node, TreeCursor};

use crate::{document::DocumentData, diagnostics::tree_utils::SpecialLiteralSemantics};

#[cfg(test)]
use crate::test_utils::create_test_document;

use super::{
    diagnostic_codes::DiagnosticsCode, diagnostic_run_data::DiagnosticsRunData, tree_utils::{retrace, EncodingSemantics, do_simple_query},
};

/**
 * Walk through the parse tree and analyze the statements
 */
pub fn statement_analysis(diagnostic_data: &mut DiagnosticsRunData, document: &DocumentData) {
    let mut cursor = document.tree.walk();

    //Look through the tree to find statements, then anylize those statements
    let mut reached_root = false;
    while !reached_root {
        let node = cursor.node();

        //If we reached the error limit stop analyzing further
        if diagnostic_data.current_number_of_problems >= diagnostic_data.maximum_number_of_problems
        {
            return;
        };

        if node.kind() == "statement" {
            /*let empty = &mut Vec::new();
            let (variables, scopes) = get_variables_under_scope(&cursor);
            let (unsafe_vars, safe_vars) =
                check_nodes_for_safety_under_scope(node, variables, empty, &document.source);

            throw_unsafe_error_for_vars(unsafe_vars, diagnostic_data);

            // Repeat process for other scopes
            //TODO: can there be a scope in a scope ?
            for scope in scopes {
                let safe_variables = &mut safe_vars.clone();
                let (scoped_variables, _) = get_variables_under_scope(&scope);
                let (scoped_unsafe_vars, _) = check_nodes_for_safety_under_scope(
                    scope.node(),
                    scoped_variables,
                    safe_variables,
                    &document.source,
                );

                throw_unsafe_error_for_vars(scoped_unsafe_vars, diagnostic_data);
            }

            diagnostic_data.create_linter_diagnostic(
                node.range(),
                DiagnosticSeverity::INFORMATION,
                DiagnosticsCode::UnsafeVariable.into_i32(),
                format!("'{:?}' are safe", document.semantics.get_vars_for_node(&node.id())),
            );*/
            check_safety_of_statement(&node, &document.semantics, diagnostic_data, document.source.as_bytes());
        }

        if cursor.goto_first_child() {
            continue;
        }

        if cursor.goto_next_sibling() {
            continue;
        }

        (cursor, reached_root) = retrace(cursor);
    }
}

/**
 * Calculates the safe set for a set of dependencies
 */
fn calculate_safe_set(dependencies: &mut Vec<(HashSet<String>, HashSet<String>)>, global_vars: &HashSet<String>, global: bool) -> (HashSet<String>, HashSet<String>){
    let mut dep = dependencies.clone();
    let mut safe_set : HashSet<String> = HashSet::new();
    let mut prev_length = 0;
    let mut vars_in_dependency : HashSet<String> = HashSet::new();

    // First collect all variables contained in dep
    for (provide, depend) in &dep {
        vars_in_dependency = vars_in_dependency.union(provide).cloned().collect::<HashSet<String>>().union(depend).cloned().collect::<HashSet<String>>();
    }

    //If we are not in a global context, change dependencies according to vars \ G
    if !global {
        dep = get_dependencies_only_occuring_in_set(&dep, vars_in_dependency.difference(global_vars).cloned().collect());
        info!("Variables only occuring in local context: {:?}", vars_in_dependency.difference(global_vars).cloned().collect::<HashSet<String>>());
    }

    loop {
        info!("Starting new round of safety checking with dep: {:?}", dep);

        // Have a mutable reference for closure
        let safe_set_ref = &mut safe_set;

        // Go through the dependencies list and find any elements we have all dependencies for
        dep.retain(|(provide, depend)| {
            // If all dependencies are in our safe set, then the dependency requirements are met
            if depend.is_subset(safe_set_ref) {
                info!("Found a tuple that has all the dependencies provided: ({:?}, {:?}) with safety ({:?})", provide, depend, safe_set_ref);
                // Everything that is provided is thus also safe
                safe_set_ref.extend(provide.iter().cloned());

                // Remove this dependency from the dependencies list
                return false;
            }
            else {
                info!("Passing tuple ({:?}, {:?}) as not all dependencies are provided: {:?}", provide, depend, safe_set_ref);
            }
            
            true
        });

        // Stop checking once we cannot find anything that we can use
        if dep.len() == prev_length  { break; }
        prev_length = dep.len()
    }

    (safe_set, vars_in_dependency)
}

fn get_dependencies_only_occuring_in_set(dependencies: & Vec<(HashSet<String>, HashSet<String>)>, set: HashSet<String>) -> Vec<(HashSet<String>, HashSet<String>)>{
    let mut new_dependencies = Vec::new();

    for (provide, depend) in dependencies {
        let pt = provide.intersection(&set).cloned().collect::<HashSet<String>>();
        let dt = depend.intersection(&set).cloned().collect::<HashSet<String>>();
        if !pt.is_empty() || !dt.is_empty() {
            new_dependencies.push((pt, dt));
        }
    }

    new_dependencies
}

/**
 * Check if a statement is safe
 */
fn check_safety_of_statement(node : &Node, semantics: &EncodingSemantics, diagnostics: &mut DiagnosticsRunData, source: &[u8]) {
    let mut dep = semantics.get_dependency_for_node(&node.id());

    info!("Starting safety check of statement with dependencies: {:?}", dep);

    // Find all global variables
    let global_vars = semantics.get_global_vars_for_node(&node.id());
    info!("Found global variables: {:?}", global_vars);

    let (global_safe_set, vars_in_dependency) = calculate_safe_set(&mut get_dependencies_only_occuring_in_set(&dep, global_vars.clone()), &global_vars, true);

    let mut local_unsafe_sets: Vec<(SpecialLiteralSemantics, HashSet<String>)> = Vec::new();

    //Calculate for local contexts
    for literal in semantics.get_special_literals_for_node(&node.id()) {
        info!("Checking safety for literal: {:?}", literal);
        let (local_safe_set, local_vars_in_dependency) = calculate_safe_set(&mut literal.local_dependency.clone(), &global_vars, false);

        let unsafe_vars : HashSet<String> = local_vars_in_dependency.difference(&local_safe_set).cloned().collect();
        local_unsafe_sets.push((literal, unsafe_vars.difference(&vars_in_dependency).cloned().collect()));
        info!("Found this unsafe set: {:?}", local_unsafe_sets.last().unwrap().1);
    }

    let mut unsafe_set: HashSet<String> = vars_in_dependency.difference(&global_safe_set).cloned().collect();

    for (literal, set) in local_unsafe_sets {
        let variable_locations = literal.variable_locations;
        for unsafe_var in set.iter() {
            for (range, name) in &variable_locations {
                if unsafe_var == name {
                    diagnostics.create_linter_diagnostic(*range, DiagnosticSeverity::ERROR, DiagnosticsCode::UnsafeVariable.into_i32(), format!("'{}' is unsafe", unsafe_var))
                }
            }
        }
    }
    
    info!("Found this global unsafe set: {:?}", unsafe_set);
    let variable_locations = do_simple_query("(VARIABLE) @name", node, source);
    for unsafe_var in unsafe_set.iter() {
        for (range, name, _) in &variable_locations {
            if unsafe_var == name {
                diagnostics.create_linter_diagnostic(*range, DiagnosticSeverity::ERROR, DiagnosticsCode::UnsafeVariable.into_i32(), format!("'{}' is unsafe", unsafe_var))
            }
        }
    }

}

/**
 * Looks through the tree and see which variables are found under this scope.
 * Also returns extra scopes it found in this scope
 */
fn get_variables_under_scope<'a>(cur: &'a TreeCursor) -> (Vec<Node<'a>>, Vec<TreeCursor<'a>>) {
    let mut variables = Vec::new();
    let mut scopes = Vec::new();
    let mut cursor = cur.clone();

    let root = cursor.node();
    let mut reached_root = false;

    //Jump into the scope
    cursor.goto_first_child();

    while !reached_root {
        let node = cursor.node();

        if node.kind() == root.kind() || node.kind() == "statement" {
            //We have explored everything
            return (variables, scopes);
        } else if node.kind() == "VARIABLE" {
            //We have found a variable add it to the list
            variables.push(node);
        } else if node.kind() == "disjunction"
            || node.kind() == "conjunction"
            || node.kind() == "bodyaggregate"
        {
            //We entered a different scope, add it to the list of scopes to be check later
            scopes.push(cursor.clone());

            //Don't explore this element any further
            (cursor, reached_root) = retrace(cursor);
            continue;
        }

        if cursor.goto_first_child() {
            continue;
        }

        if node.next_sibling().is_none()
            && node.parent().is_some()
            && node.parent().unwrap().kind() == root.kind()
        {
            //We have explored everything
            return (variables, scopes);
        }

        if cursor.goto_next_sibling() {
            continue;
        }

        (cursor, reached_root) = retrace(cursor);
    }
    (variables, scopes)
}

/**
 * Checks if every variable found is safe in this scope
 */
fn check_nodes_for_safety_under_scope<'a>(
    scope: Node,
    to_check_variables: Vec<Node>,
    known_safe_variables: &'a mut Vec<&'a str>,
    source: &'a str,
) -> (DashMap<&'a str, Vec<tree_sitter::Range>>, Vec<&'a str>) {
    let mut safe_variables = Vec::new();
    safe_variables.append(known_safe_variables);
    let unsafe_variables = DashMap::new();

    //Go through each variable from the bottom,
    for variable in to_check_variables {
        let mut inspected_node = variable;
        let mut reached_atom_once = false;
        while inspected_node != scope {
            // goto parent if it exists
            match inspected_node.parent() {
                Some(parent) => inspected_node = parent,
                None => break,
            }

            if inspected_node == scope {
                break;
            }
            // if these variables are in the body add them to the safe list
            // We also need to have reached atom once, otherwise we are only in a equation
            else if inspected_node.kind() == "bodydot" && reached_atom_once {
                safe_variables.push(variable.utf8_text(source.as_bytes()).unwrap());
                //diagnostic_data.create_linter_diagnostic(variable.range(), DiagnosticSeverity::INFORMATION, 0, "body dot safed variable".to_string());
                break;
            }
            // if we reach this cannot lead to safety
            else if inspected_node.kind() == "atom" {
                reached_atom_once = true;
                match inspected_node.prev_sibling() {
                    Some(prev) => {
                        if prev.kind() == "NOT" {
                            break;
                        }
                    }
                    None => {
                        //If we are in a conjunction we set variables to safe
                        if scope.kind() == "conjunction" {
                            safe_variables.push(variable.utf8_text(source.as_bytes()).unwrap());
                            //diagnostic_data.create_linter_diagnostic(variable.range(), DiagnosticSeverity::INFORMATION, 0, "conjunction safed variable".to_string());
                            break;
                        }
                    }
                }
            }
            // If we have an assignment before an aggegrate that value becomes safe
            else if inspected_node.kind() == "term" {
                match inspected_node.parent() {
                    Some(parent) => {
                        if parent.kind() == "lubodyaggregate" {
                            safe_variables.push(variable.utf8_text(source.as_bytes()).unwrap());
                            break;
                        }
                    }
                    None => {}
                }
            }
            //If we see a COLON before us, then we are a safe variable for this context
            match inspected_node.prev_sibling() {
                Some(before) => {
                    if before.kind() == "COLON" {
                        safe_variables.push(variable.utf8_text(source.as_bytes()).unwrap());
                        //diagnostic_data.create_linter_diagnostic(variable.range(), DiagnosticSeverity::INFORMATION, 0, "COLON safed variable".to_string());
                        break;
                    }
                }
                None => {}
            }
        }

        let name = variable.utf8_text(source.as_bytes()).unwrap();
        if !unsafe_variables.contains_key(name) {
            unsafe_variables.insert(name, vec![variable.range()]);
        } else {
            unsafe_variables
                .get_mut(name)
                .unwrap()
                .push(variable.range());
        }
    }

    for safe in &safe_variables {
        unsafe_variables.remove(safe);
    }

    (unsafe_variables, safe_variables)
}

/**
 * creates a diagnostic error for each of the unsafe variables
 */
fn throw_unsafe_error_for_vars(
    vars: DashMap<&str, Vec<tree_sitter::Range>>,
    diagnostic_data: &mut DiagnosticsRunData,
) {
    for var in &vars {
        for range in var.value() {
            diagnostic_data.create_linter_diagnostic(
                *range,
                DiagnosticSeverity::ERROR,
                DiagnosticsCode::UnsafeVariable.into_i32(),
                format!("'{}' is unsafe", var.key()),
            );
        }
    }
}

#[test]
fn no_variables_should_be_detected_as_safe() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(&mut diags, &create_test_document("a :- b.".to_string()));

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_no_body() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(&mut diags, &create_test_document("a(X).".to_string()));

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn unsafe_variables_should_be_detected_no_variables_in_body() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(&mut diags, &create_test_document("a(X) :- b.".to_string()));

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn unsafe_variables_should_not_be_detected_if_variable_in_body() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(X) :- b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn safeness_should_be_blocked_by_not() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a :- not b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn safeness_should_be_blocked_by_multiple_nots() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a :- not not b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn safeness_should_be_working_if_one_of_the_body_atoms_provides_it() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(X) :- not b(X), b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn safeness_should_not_be_blocked_by_not_in_head() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("not a(X) :- b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn safeness_should_work_with_integrity_constraints() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(":- b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafeness_should_work_with_integrity_constraints() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(":- b(X), c(X+Y).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn unsafe_variables_should_be_detected_in_choice_rule_with_no_body() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(&mut diags, &create_test_document("{a(X)}.".to_string()));

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn safeness_correctly_detected_in_choice_rule() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("{a(X) : b(X)}.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_in_choice_rule() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("{a(Y) : b(X)}.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn body_can_safe_variables_in_choice_rule() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("{a(X)} :- b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_in_conjunctions() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("{a(X)} :- a : b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 2);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn safeness_should_be_detected_in_conjunctions_with_pure_variables() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a :- a(Y) : b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_in_conjunctions_with_not() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a :- not a(Y) : b(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn unsafe_variables_should_be_detected_in_show() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("#show X : a.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn safeness_should_be_detected_in_show() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("#show X : a(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_with_aggregates() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(X) :- N = #count{X : b(X)}.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 3);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn unsafe_variables_should_be_detected_with_aggregates_and_disjunction() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(N), c(X) :- N = #count{X : b(X)}.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn safeness_should_be_detected_with_aggregates() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a :- N = #count{X : b(X)}.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_within_aggregates() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a :- N = #count{X : b}.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn unsafe_variables_should_be_detected_with_disjunction() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(E) : b(X) :- a.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnsafeVariable.into_i32())
    );
}

#[test]
fn safeness_should_be_detected_with_disjunction() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(X) : b(X) :- a.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_with_comparison() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(Y) :- X=Y.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 3);
}

#[test]
fn safe_variables_should_be_detected_with_comparison_indirectly() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(Y) :- a(X), X=Y.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}


#[test]
fn unsafe_variables_should_be_detected_with_multiple_statements_correctly() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(
            "a(X) :- b(Y). c(X) :- d(X). a(Y), b(Z) :- a(X), X=Y, Y=Z. c(X,Y) :- a(Z, Y)."
                .to_string(),
        ),
    );

    assert_eq!(diags.total_diagnostics.len(), 2);
}

#[test]
fn safe_variables_should_be_detected_with_pools() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(
            "a(X) :- a(X;X)."
                .to_string(),
        ),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_with_pools() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(
            "a(X) :- a(X;Y)."
                .to_string(),
        ),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_with_aritmethics() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(
            "a(X,Y) :- a(X+Y, X)."
                .to_string(),
        ),
    );

    assert_eq!(diags.total_diagnostics.len(), 2);
}

#[test]
fn constant_should_safe_equation() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(
            "a(X) :- a(X+1)."
                .to_string(),
        ),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn constant_cannot_safe_multiplication_if_zero() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(
            "a(X) :- a(X*0)."
                .to_string(),
        ),
    );

    assert_eq!(diags.total_diagnostics.len(), 2);
}

#[test]
fn negated_not_equals_should_be_handled_as_equals() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(
            "a(X) :- a(Y), not Y != X."
                .to_string(),
        ),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}
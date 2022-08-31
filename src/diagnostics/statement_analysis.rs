use std::{collections::HashSet, hash::Hash};

use dashmap::DashMap;
use log::info;
use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::{Node, TreeCursor};

use crate::{document::DocumentData, semantics::{encoding_semantic::EncodingSemantics, special_literal_semantic::SpecialLiteralSemantics}};

//#[cfg(test)]
//use crate::test_utils::create_test_document;

use super::{
    diagnostic_codes::DiagnosticsCode, diagnostic_run_data::DiagnosticsRunData, tree_utils::retrace,
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
            check_safety_of_statement(&node, &document.semantics, diagnostic_data);
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
    }

    loop {
        // Have a mutable reference for closure
        let safe_set_ref = &mut safe_set;

        // Go through the dependencies list and find any elements we have all dependencies for
        dep.retain(|(provide, depend)| {
            // If all dependencies are in our safe set, then the dependency requirements are met
            if depend.is_subset(safe_set_ref) {
                // Everything that is provided is thus also safe
                safe_set_ref.extend(provide.iter().cloned());

                // Remove this dependency from the dependencies list
                return false;
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
fn check_safety_of_statement(node : &Node, semantics: &EncodingSemantics, diagnostics: &mut DiagnosticsRunData) {
    let statement_semantics = semantics.get_statement_semantics_for_node(node.id());
    let mut dep = statement_semantics.dependencies;

    // Find all global variables
    let global_vars = statement_semantics.global_vars;
    
    info!("Checking Safety of statement with dependency set: {:?} and global variables: {:?} and vars: {:?}", &dep, &global_vars, statement_semantics.vars);

    let (global_safe_set, vars_in_dependency) = calculate_safe_set(&mut get_dependencies_only_occuring_in_set(&dep, global_vars.clone()), &global_vars, true);

    let mut local_unsafe_sets: Vec<(SpecialLiteralSemantics, HashSet<String>)> = Vec::new();

    //Calculate for local contexts
    for literal in statement_semantics.special_literals {
        let (local_safe_set, local_vars_in_dependency) = calculate_safe_set(&mut literal.local_dependency.clone(), &global_vars, false);

        let unsafe_vars : HashSet<String> = local_vars_in_dependency.difference(&local_safe_set).cloned().collect();
        local_unsafe_sets.push((literal, unsafe_vars.difference(&vars_in_dependency).cloned().collect()));
    }

    let mut unsafe_set: HashSet<String> = vars_in_dependency.difference(&global_safe_set).cloned().collect();
    let variable_locations = statement_semantics.vars_locations;

    for (_, set) in local_unsafe_sets {
        for unsafe_var in set.iter() {
            let locations = variable_locations.get(unsafe_var);
            if let Some(l) = locations {
                for location in l {
                    diagnostics.create_linter_diagnostic(*location, DiagnosticSeverity::ERROR, DiagnosticsCode::UnsafeVariable.into_i32(), format!("'{}' is unsafe", unsafe_var))
                }
            } else {
                info!("Could not find variable that should be in variable list: {}", unsafe_var);
            }
        }
    }

    //TODO: Merge these 2 into a function
    for unsafe_var in unsafe_set.iter() {
        let locations = variable_locations.get(unsafe_var);
        if let Some(l) = locations {
            for location in l {
                diagnostics.create_linter_diagnostic(*location, DiagnosticSeverity::ERROR, DiagnosticsCode::UnsafeVariable.into_i32(), format!("'{}' is unsafe", unsafe_var))
            }
        } else {
            info!("Could not find variable that should be in variable list: {}", unsafe_var);
        }
    }

}

/*

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
*/
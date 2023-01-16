use log::info;
use std::collections::HashSet;
use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::{Node, Query, QueryCursor};

use crate::{document::DocumentData, semantics::special_literal_semantic::SpecialLiteralSemantics};

#[cfg(test)]
use crate::test_utils::create_test_document;

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
            check_safety_of_statement(&node, &document, diagnostic_data);
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
fn calculate_safe_set(
    dependencies: &mut Vec<(HashSet<String>, HashSet<String>)>,
    global_vars: &HashSet<String>,
    global: bool,
) -> (HashSet<String>, HashSet<String>) {
    let mut dep = dependencies.clone();
    let mut safe_set: HashSet<String> = HashSet::new();
    let mut prev_length = 0;
    let mut vars_in_dependency: HashSet<String> = HashSet::new();

    // First collect all variables contained in dep
    for (provide, depend) in &dep {
        vars_in_dependency = vars_in_dependency
            .union(provide)
            .cloned()
            .collect::<HashSet<String>>()
            .union(depend)
            .cloned()
            .collect::<HashSet<String>>();
    }

    //If we are not in a global context, change dependencies according to vars \ G
    if !global {
        dep = get_dependencies_only_occuring_in_set(
            &dep,
            vars_in_dependency
                .difference(global_vars)
                .cloned()
                .collect(),
        );
    }

    loop {
        // Have a mutable reference for closure
        let safe_set_ref = &mut safe_set;

        info!("Starting loop with safe set {:?}", safe_set_ref);

        // Go through the dependencies list and find any elements we have all dependencies for
        dep.retain(|(provide, depend)| {
            // If all dependencies are in our safe set, then the dependency requirements are met
            if depend.is_subset(safe_set_ref) {
                info!("Using dependency: ({:?},{:?})", provide, depend);

                // Everything that is provided is thus also safe
                safe_set_ref.extend(provide.iter().cloned());

                // Remove this dependency from the dependencies list
                return false;
            }
            true
        });

        // Stop checking once we cannot find anything that we can use
        if dep.len() == prev_length {
            break;
        }
        prev_length = dep.len()
    }

    (safe_set, vars_in_dependency)
}

fn get_dependencies_only_occuring_in_set(
    dependencies: &Vec<(HashSet<String>, HashSet<String>)>,
    set: HashSet<String>,
) -> Vec<(HashSet<String>, HashSet<String>)> {
    let mut new_dependencies = Vec::new();

    for (provide, depend) in dependencies {
        let pt = provide
            .intersection(&set)
            .cloned()
            .collect::<HashSet<String>>();
        let dt = depend
            .intersection(&set)
            .cloned()
            .collect::<HashSet<String>>();
        if !pt.is_empty() || !dt.is_empty() {
            new_dependencies.push((pt, dt));
        }
    }

    new_dependencies
}

/**
 * Find all variables occuring in a part of the encoding
 */
fn get_variables_in_statement<'a>(
    node: &tree_sitter::Node<'a>,
    source: &'a [u8],
) -> std::vec::Vec<(tree_sitter::Range, &'a str, tree_sitter::Node<'a>)> {
    let mut query_cursor = QueryCursor::new();
    let query = Query::new(tree_sitter_clingo::language(), "(VARIABLE) @name").unwrap();

    let matches = query_cursor.matches(&query, *node, source);
    let mut output = Vec::new();

    for each_match in matches {
        for capture in each_match.captures.iter() {
            let range = capture.node.range();
            let name = capture.node.utf8_text(source).unwrap();

            output.push((range, name, capture.node));
        }
    }

    output
}

/**
 * Check if a statement is safe
 */
fn check_safety_of_statement(
    node: &Node,
    document: &DocumentData,
    diagnostics: &mut DiagnosticsRunData,
) {
    let statement_semantics = document
        .semantics
        .get_statement_semantics_for_node(node.id());
    let dep = statement_semantics.dependencies;

    // Find all global variables
    let global_vars = statement_semantics.global_vars;

    info!("Checking Safety of statement with dependency set: {:?} and global variables: {:?} and vars: {:?}", &dep, &global_vars, statement_semantics.vars);

    let (global_safe_set, vars_in_dependency) = calculate_safe_set(
        &mut get_dependencies_only_occuring_in_set(&dep, global_vars.clone()),
        &global_vars,
        true,
    );

    let mut local_unsafe_sets: Vec<(SpecialLiteralSemantics, HashSet<String>)> = Vec::new();

    //Calculate for local contexts
    for literal in statement_semantics.special_literals {
        info!("Calculating for local context: {:?}", literal);
        let (local_safe_set, local_vars_in_dependency) =
            calculate_safe_set(&mut literal.local_dependency.clone(), &global_vars, false);

        let unsafe_vars: HashSet<String> = local_vars_in_dependency
            .difference(&local_safe_set)
            .cloned()
            .collect();
        local_unsafe_sets.push((
            literal,
            unsafe_vars
                .difference(&vars_in_dependency)
                .cloned()
                .collect(),
        ));
    }

    let unsafe_set: HashSet<String> = vars_in_dependency
        .difference(&global_safe_set)
        .cloned()
        .collect();

    //Due to the fact that the variable locations could have changed in terms of byte range, we look for the variables again
    let source = document.get_bytes();
    let variable_locations = get_variables_in_statement(node, &source);
    let mut unsafe_vars = unsafe_set.clone();

    info!("{:?}", variable_locations);
    info!("local unsafe set: {:?}", local_unsafe_sets);
    info!("unsafe set: {:?}", unsafe_set);

    //First combine the lists of all unsafe variables in the statements
    for (_, set) in local_unsafe_sets {
        unsafe_vars = unsafe_vars.union(&set).cloned().collect();
    }

    //Next we create a diagnostic for every variable we find in the variable_locations list that occurs in the unsafe_vars list
    for (location, var, _) in variable_locations {
        if unsafe_vars.contains(var) {
            diagnostics.create_linter_diagnostic(
                location,
                DiagnosticSeverity::ERROR,
                DiagnosticsCode::UnsafeVariable.into_i32(),
                format!("'{}' is unsafe", var),
            )
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

    statement_analysis(&mut diags, &create_test_document(":- b(X).".to_string()));

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

    assert_eq!(diags.total_diagnostics.len(), 4);

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

    assert_eq!(diags.total_diagnostics.len(), 5);

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
        &create_test_document("a(X) :- a(X;X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_with_pools() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(X) :- a(X;Y).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 2);
}

#[test]
fn unsafe_variables_should_be_detected_with_aritmethics() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(X,Y) :- a(X+Y, X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 2);
}

#[test]
fn constant_should_safe_equation() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(X) :- a(X+1).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn constant_cannot_safe_multiplication_if_zero() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(X) :- a(X*0).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 2);
}

#[test]
fn negated_not_equals_should_be_handled_as_equals() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("a(X) :- a(Y), not Y != X.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_in_weak_constraint() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(":~ a(X). [Y]".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);
}

#[test]
fn safeness_should_be_detected_in_weak_constraint() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(":~ a(X). [X]".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_in_optimization() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("#minimize{Y@1,X:hotel(X)}.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 1);
}

#[test]
fn safeness_should_be_detected_in_optimization() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("#minimize{Y@1,X:hotel(X), star(X, Y)}.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

#[test]
fn unsafe_variables_should_be_detected_for_aggregate_in_head() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("#sum{X : b(X)}.".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 2);
}

#[test]
fn safeness_should_be_detected_for_aggregate_in_head() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document("#sum{X : b(X)} :- test(X).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 0);
}

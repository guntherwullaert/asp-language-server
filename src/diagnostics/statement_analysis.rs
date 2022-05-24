use dashmap::DashMap;
use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::{Node, TreeCursor};

use crate::{document::DocumentData};

#[cfg(test)]
use crate::test_utils::create_test_document;

use super::{diagnostic_run_data::DiagnosticsRunData, diagnostic_codes::DiagnosticsCode, tree_utils::retrace};

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
            let empty = &mut Vec::new();
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
fn safeness_should_be_detected_in_conjunctions() {
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
        &create_test_document(":- X1=X2, Y0=Y1..Y2, Z0=(Z1;Z2).".to_string()),
    );

    assert_eq!(diags.total_diagnostics.len(), 8);
}

#[test]
fn unsafe_variables_should_be_detected_with_multiple_statements_correctly() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();

    statement_analysis(
        &mut diags,
        &create_test_document(
            "a(X) :- N = #count{X : b(X)}.
    a(N), c(X) :- N = #count{X : b(X)}.
    :- X1=X2, Y0=Y1..Y2, Z0=(Z1;Z2)."
                .to_string(),
        ),
    );

    assert_eq!(diags.total_diagnostics.len(), 10);
}

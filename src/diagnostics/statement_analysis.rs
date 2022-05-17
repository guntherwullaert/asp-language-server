use dashmap::DashMap;
use tower_lsp::{lsp_types::{DiagnosticSeverity, MessageType}, Client};

use crate::{
    document::DocumentData,
    treeutils::{create_simple_query, retrace},
};

use super::{diagnostic_run_data::DiagnosticsRunData, error_codes::*};

/**
 * Walk through the parse tree and analyze the statements
 */
pub fn statement_analysis(
    diagnostic_data: &mut DiagnosticsRunData,
    document: &DocumentData,
) {
    let mut cursor = document.tree.walk();

    let mut reached_root = false;
    while !reached_root {
        let node = cursor.node();

        if diagnostic_data.current_number_of_problems >= diagnostic_data.maximum_number_of_problems
        {
            return;
        };

        if node.kind() == "statement" {
            //Check if a variable is unsafe
            let all_variables = DashMap::new();
            let mut safe_variables: Vec<&str> = Vec::new();

            //Create a query to search for the variables in the statement
            let all_captures = create_simple_query(
                r#"
                ((VARIABLE) @variable)
                "#,
                node,
                document.source.as_bytes(),
            );

            //For each occurence check if the variable is safe
            for capture in all_captures {
                let (range, name, capture_node) = capture;
                if !all_variables.contains_key(name) {
                    let ranges = vec![range];
                    all_variables.insert(name, ranges);
                } else {
                    all_variables.get_mut(name).unwrap().push(range)
                }

                //traverse up the tree and see if this variable is contained by something that is safe
                let mut reached = false;
                let mut inspected_node = capture_node;
                while !reached {
                    match inspected_node.parent() {
                        Some(parent) => inspected_node = parent,
                        None => {
                            diagnostic_data.create_linter_diagnostic(
                                range,
                                DiagnosticSeverity::ERROR,
                                0,
                                "(statement analysis) reached root without, reaching a known node!".to_string(),
                            );
                            break;
                        }
                    }

                    if inspected_node.kind() == "statement" {
                        reached = true;
                        diagnostic_data.create_linter_diagnostic(
                            range,
                            DiagnosticSeverity::INFORMATION,
                            0,
                            format!("'{}' reached statement", name),
                        );
                    }
                    if inspected_node.kind() == "bodydot" {
                        safe_variables.push(name);
                        reached = true;
                        diagnostic_data.create_linter_diagnostic(
                            range,
                            DiagnosticSeverity::INFORMATION,
                            0,
                            format!("'{}' reached body, added to safe", name),
                        );
                    }
                    if inspected_node.kind() == "atom" {
                        match inspected_node.prev_sibling() {
                            Some(prev) => {
                                if prev.kind() == "NOT" {
                                    reached = true;
                                    diagnostic_data.create_linter_diagnostic(
                                        range,
                                        DiagnosticSeverity::INFORMATION,
                                        0,
                                        format!("'{}' reached not", name),
                                    );
                                }
                            }
                            None => continue,
                        }
                    }
                }
            }

            //Remove every entry in all_variables that is in the safe variable list
            for safe in safe_variables {
                all_variables.remove(safe);
            }

            //Send an error to the client for each variable that is unsafe
            for var in all_variables {
                for range in var.1 {
                    diagnostic_data.create_linter_diagnostic(
                        range,
                        DiagnosticSeverity::ERROR,
                        UNSAFE_VARIABLE,
                        format!("'{}' is unsafe", var.0),
                    );
                }
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

use tower_lsp::{lsp_types::DiagnosticSeverity, Client};

use crate::{
    document::DocumentData,
    treeutils::{humanize_token, retrace},
};

use super::{diagnostic_run_data::DiagnosticsRunData, error_codes::*};

/**
* Search for errors in the parse tree.
*/
pub fn search_for_tree_error(
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

        if node.is_error() {
            let next = node.prev_sibling();
            if next.is_some() && next.unwrap().kind() == "statement" {
                //Found an error which is preceeded by an statement, most likely a . is missing
                diagnostic_data.create_tree_sitter_diagnostic(
                    node.range(),
                    DiagnosticSeverity::ERROR,
                    EXPECTED_DOT_PARSE_ERROR,
                    format!(
                        "syntax error while parsing value: '{}', expected: '.'",
                        node.utf8_text(document.source.as_bytes()).unwrap()
                    ),
                );

                //Don't go deeper into the error node
                (cursor, reached_root) = retrace(cursor);
                continue;
            }

            //If we reach here, we do not have a guess why the error occured
            diagnostic_data.create_tree_sitter_diagnostic(
                node.range(),
                DiagnosticSeverity::ERROR,
                UNKNOWN_PARSE_ERROR,
                format!(
                    "syntax error while parsing value: '{}'",
                    node.utf8_text(document.source.as_bytes()).unwrap()
                ),
            );
        } else if node.is_missing() {
            //If node is missing, tell the user what we expected
            diagnostic_data.create_tree_sitter_diagnostic(
                node.range(),
                DiagnosticSeverity::ERROR,
                EXPECTED_MISSING_PARSE_ERROR,
                format!(
                    "syntax error while parsing, expected: '{}'",
                    humanize_token(&node.kind().to_string())
                ),
            );
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

use tower_lsp::{
    lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range},
    Client,
};
use tree_sitter::Tree;

use crate::{document::DocumentData, treeutils::humanize_token};

/**
 * ERROR CODES TREE-SITTER
 */
const UNKNOWN_PARSE_ERROR: i32 = 1000;
const EXPECTED_DOT_PARSE_ERROR: i32 = 1001;
const EXPECTED_MISSING_PARSE_ERROR: i32 = 1002;

#[derive(Debug)]
pub struct DiagnosticsAnalyzer {
    maximum_number_of_problems: u32,
    current_number_of_problems: u32,
}

impl DiagnosticsAnalyzer {
    /**
     * Run the selected diagnostics on the parse tree
     */
    pub async fn run(&mut self, document: &DocumentData, client: &Client) {
        //Reset the current number of problems found to zero
        self.current_number_of_problems = 0;

        let mut total_diagnostics = Vec::new();

        total_diagnostics.append(&mut self.search_for_tree_error(&document.tree, &document.source));

        //Once done send all diagnostic info to the client
        client
            .publish_diagnostics(
                document.uri.clone(),
                total_diagnostics,
                Some(document.version),
            )
            .await;
    }

    /**
     * Search for errors in the parse tree.
     */
    fn search_for_tree_error(
        &self,
        tree: &Tree,
        source: &str,
    ) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        let mut diagnostics = Vec::new();

        let mut cursor = tree.walk();

        let mut reached_root = false;
        while !reached_root {
            let node = cursor.node();

            if self.current_number_of_problems >= self.maximum_number_of_problems {
                return diagnostics;
            };

            if node.is_error() {
                let next = node.prev_sibling();
                if next.is_some() && next.unwrap().kind() == "statement" {
                    //Found an error which is preceeded by an statement, most likely a . is missing
                    diagnostics.push(self.create_tree_sitter_diagnostic(
                        node.range(),
                        DiagnosticSeverity::ERROR,
                        EXPECTED_DOT_PARSE_ERROR,
                        format!(
                            "syntax error while parsing value: '{}', expected: '.'",
                            node.utf8_text(source.as_bytes()).unwrap()
                        ),
                    ));

                    //Don't go deeper into the error node
                    let mut retracing = true;
                    while retracing {
                        if !cursor.goto_parent() {
                            retracing = false;
                            reached_root = true;
                        }

                        if cursor.goto_next_sibling() {
                            retracing = false;
                        }
                    }
                    continue;
                }

                //If we reach here, we do not have a guess why the error occured
                diagnostics.push(self.create_tree_sitter_diagnostic(
                    node.range(),
                    DiagnosticSeverity::ERROR,
                    UNKNOWN_PARSE_ERROR,
                    format!(
                        "syntax error while parsing value: '{}'",
                        node.utf8_text(source.as_bytes()).unwrap()
                    ),
                ));
            } else if node.is_missing() {
                //If node is missing, tell the user what we expected
                diagnostics.push(self.create_tree_sitter_diagnostic(
                    node.range(),
                    DiagnosticSeverity::ERROR,
                    EXPECTED_MISSING_PARSE_ERROR,
                    format!(
                        "syntax error while parsing, expected: '{}'",
                        humanize_token(&node.kind().to_string())
                    ),
                ));
            }

            if cursor.goto_first_child() {
                continue;
            }

            if cursor.goto_next_sibling() {
                continue;
            }

            let mut retracing = true;
            while retracing {
                if !cursor.goto_parent() {
                    retracing = false;
                    reached_root = true;
                }

                if cursor.goto_next_sibling() {
                    retracing = false;
                }
            }
        }

        diagnostics
    }

    pub fn new(maximum_number_of_problems: u32) -> DiagnosticsAnalyzer {
        DiagnosticsAnalyzer {
            current_number_of_problems: 0,
            maximum_number_of_problems,
        }
    }

    /**
     * Create a diagnostic message from tree-sitter
     */
    fn create_tree_sitter_diagnostic(
        &self,
        range: tree_sitter::Range,
        severity: DiagnosticSeverity,
        code_number: i32,
        message: String,
    ) -> Diagnostic {
        self.create_diagnostic(
            range,
            severity,
            code_number,
            "tree-sitter".to_string(),
            message,
        )
    }

    /**
     * Create a generic diagnostic message
     */
    fn create_diagnostic(
        &self,
        range: tree_sitter::Range,
        severity: DiagnosticSeverity,
        code_number: i32,
        source: String,
        message: String,
    ) -> Diagnostic {
        Diagnostic::new_with_code_number(
            Range::new(
                Position::new(
                    range.start_point.row.try_into().unwrap(),
                    range.start_point.column.try_into().unwrap(),
                ),
                Position::new(
                    range.end_point.row.try_into().unwrap(),
                    range.end_point.column.try_into().unwrap(),
                ),
            ),
            severity,
            code_number,
            Some(source),
            message,
        )
    }
}
use dashmap::DashMap;
use tower_lsp::{
    lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range},
    Client,
};
use tree_sitter::{Query, QueryCursor, Tree};

use crate::{document::DocumentData, treeutils::humanize_token, treeutils::retrace};

/**
 * ERROR CODES TREE-SITTER
 */
const UNKNOWN_PARSE_ERROR: i32 = 1000;
const EXPECTED_DOT_PARSE_ERROR: i32 = 1001;
const EXPECTED_MISSING_PARSE_ERROR: i32 = 1002;

/**
 * ERROR CODES ANALYSIS
 */
const UNSAFE_VARIABLE: i32 = 2000;

#[derive(Debug)]
pub struct DiagnosticsAnalyzer {
    maximum_number_of_problems: u32,
    current_number_of_problems: u32,

    //A list of diagnostics to be send to the user
    total_diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsAnalyzer {
    /**
     * Run the selected diagnostics on the parse tree
     */
    pub async fn run(&mut self, document: &DocumentData, client: &Client) {
        //Reset the current number of problems found to zero
        self.current_number_of_problems = 0;

        self.total_diagnostics = Vec::new();

        //self.total_diagnostics.append(&mut self.search_for_tree_error(&document.tree, &document.source));
        self.search_for_tree_error(&document.tree, &document.source);

        self.statement_analysis(&document.tree, &document.source);

        //Once done send all diagnostic info to the client
        client
            .publish_diagnostics(
                document.uri.clone(),
                self.total_diagnostics.clone(),
                Some(document.version),
            )
            .await;
    }

    /**
     * Search for errors in the parse tree.
     */
    fn search_for_tree_error(&mut self, tree: &Tree, source: &str) {
        let mut cursor = tree.walk();

        let mut reached_root = false;
        while !reached_root {
            let node = cursor.node();

            if self.current_number_of_problems >= self.maximum_number_of_problems {
                return;
            };

            if node.is_error() {
                let next = node.prev_sibling();
                if next.is_some() && next.unwrap().kind() == "statement" {
                    //Found an error which is preceeded by an statement, most likely a . is missing
                    self.create_tree_sitter_diagnostic(
                        node.range(),
                        DiagnosticSeverity::ERROR,
                        EXPECTED_DOT_PARSE_ERROR,
                        format!(
                            "syntax error while parsing value: '{}', expected: '.'",
                            node.utf8_text(source.as_bytes()).unwrap()
                        ),
                    );

                    //Don't go deeper into the error node
                    (cursor, reached_root) = retrace(cursor);
                    continue;
                }

                //If we reach here, we do not have a guess why the error occured
                self.create_tree_sitter_diagnostic(
                    node.range(),
                    DiagnosticSeverity::ERROR,
                    UNKNOWN_PARSE_ERROR,
                    format!(
                        "syntax error while parsing value: '{}'",
                        node.utf8_text(source.as_bytes()).unwrap()
                    ),
                );
            } else if node.is_missing() {
                //If node is missing, tell the user what we expected
                self.create_tree_sitter_diagnostic(
                    node.range(),
                    DiagnosticSeverity::ERROR,
                    EXPECTED_MISSING_PARSE_ERROR,
                    format!(
                        "syntax error while parsing, expected: '{}'",
                        humanize_token(&node.kind().to_string())
                    ),
                );
            } else {
                // Debug statement to inspect tree
                // self.create_tree_sitter_diagnostic(
                //    node.range(),
                //    DiagnosticSeverity::INFORMATION,
                //    -1,
                //    (&node.kind().to_string()).to_string(),
                //);
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
     * Walk through the parse tree and anylise the statements
     */
    fn statement_analysis(&mut self, tree: &Tree, source: &str) {
        let mut cursor = tree.walk();

        let mut reached_root = false;
        while !reached_root {
            let node = cursor.node();

            if self.current_number_of_problems >= self.maximum_number_of_problems {
                return;
            };

            if node.kind() == "statement" {
                //Check if a variable is unsafe
                let all_variables = DashMap::new();
                let mut safe_variables: Vec<&str> = Vec::new();

                //Create a query to search for the variables in the statement
                let mut query_cursor = QueryCursor::new();
                let query = Query::new(
                    tree_sitter_clingo::language(),
                    r#"
                    ((VARIABLE) @variable)
                    "#,
                )
                .unwrap();

                let all_matches = query_cursor.matches(&query, node, source.as_bytes());

                //For each occurence check if the variable is safe
                for each_match in all_matches {
                    for capture in each_match.captures.iter() {
                        let range = capture.node.range();
                        let name = capture.node.utf8_text(source.as_bytes()).unwrap();
                        if !all_variables.contains_key(name) {
                            all_variables.insert(name, range);
                        }

                        //traverse up the tree and see if this variable is contained by something that is safe
                        let mut reached = false;
                        let mut inspected_node = capture.node;
                        while !reached {
                            inspected_node = inspected_node.parent().unwrap();
                            if inspected_node.kind() == "statement" {
                                reached = true;
                            }
                            if inspected_node.kind() == "bodydot" {
                                safe_variables.push(name);
                                reached = true;
                            }
                            if inspected_node.kind() == "atom" {
                                match inspected_node.prev_sibling() {
                                    Some(prev) => {
                                        if prev.kind() == "NOT" {
                                            reached = true;
                                        }
                                    }
                                    None => continue,
                                }
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
                    self.create_linter_diagnostic(
                        var.1,
                        DiagnosticSeverity::ERROR,
                        UNSAFE_VARIABLE,
                        format!("'{}' is unsafe", var.0),
                    );
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

    pub fn new(maximum_number_of_problems: u32) -> DiagnosticsAnalyzer {
        DiagnosticsAnalyzer {
            current_number_of_problems: 0,
            maximum_number_of_problems,
            total_diagnostics: Vec::new(),
        }
    }

    /**
     * Create a diagnostic message from clinlint
     */
    fn create_linter_diagnostic(
        &mut self,
        range: tree_sitter::Range,
        severity: DiagnosticSeverity,
        code_number: i32,
        message: String,
    ) {
        self.create_diagnostic(
            range,
            severity,
            code_number,
            "clinlint".to_string(),
            message,
        )
    }

    /**
     * Create a diagnostic message from tree-sitter
     */
    fn create_tree_sitter_diagnostic(
        &mut self,
        range: tree_sitter::Range,
        severity: DiagnosticSeverity,
        code_number: i32,
        message: String,
    ) {
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
        &mut self,
        range: tree_sitter::Range,
        severity: DiagnosticSeverity,
        code_number: i32,
        source: String,
        message: String,
    ) {
        self.total_diagnostics
            .push(Diagnostic::new_with_code_number(
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
            ));
        self.current_number_of_problems += 1;
    }
}

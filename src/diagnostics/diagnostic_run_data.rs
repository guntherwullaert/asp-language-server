use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

/**
 * A object that contains all the diagnostic data which was found
 */
#[derive(Debug)]
pub struct DiagnosticsRunData {
    pub maximum_number_of_problems: u32,
    pub current_number_of_problems: u32,

    //A list of diagnostics to be send to the user
    pub total_diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsRunData {
    /**
     * Create a diagnostic message from clinlint
     */
    pub fn create_linter_diagnostic(
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
    pub fn create_tree_sitter_diagnostic(
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

    #[cfg(test)]
    pub fn create_test_diagnostics() -> DiagnosticsRunData {
        DiagnosticsRunData {
            maximum_number_of_problems: 100,
            current_number_of_problems: 0,
            total_diagnostics: Vec::new(),
        }
    }
}

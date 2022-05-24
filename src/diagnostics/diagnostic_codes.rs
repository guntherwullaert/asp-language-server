/**
 * DIAGNOSTICS CODES TREE-SITTER
 */
pub enum DiagnosticsCode {
    /**
     * ERROR CODES TREE SITTER
     */
    // Error nodes
    UnknownParseState = 1000,
    ExpectedDot = 1001,
    // Missing nodes
    ExpectedMissingToken = 1101,

    /**
     * ERROR CODES ANALYSIS
     */
    UnsafeVariable = 2000,
    UndefinedOperation = 2001,
}

impl DiagnosticsCode {
    pub fn into_i32(self) -> i32{
        self as i32
    }
}
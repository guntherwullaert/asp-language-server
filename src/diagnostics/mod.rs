use crate::diagnostics::statement_analysis::statement_analysis;
use crate::document::DocumentData;

use self::{diagnostic_run_data::DiagnosticsRunData, tree_error_analysis::search_for_tree_error};

mod diagnostic_codes;
mod diagnostic_run_data;
mod statement_analysis;
mod tree_error_analysis;
pub mod tree_utils;

/**
 * Run the selected diagnostics on the parse tree
 */
pub fn run_diagnostics(
    document: DocumentData,
    maximum_number_of_problems: u32,
) -> Vec<tower_lsp::lsp_types::Diagnostic> {
    //Setup the diagnostics run data object to be used for this diagnostics run
    let mut diagnostic_data = DiagnosticsRunData {
        maximum_number_of_problems,
        current_number_of_problems: 0,
        total_diagnostics: Vec::new(),
    };

    search_for_tree_error(&mut diagnostic_data, &document);

    statement_analysis(&mut diagnostic_data, &document);

    diagnostic_data.total_diagnostics
}

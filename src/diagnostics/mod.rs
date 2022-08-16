use std::time::Duration;

use log::info;
use tower_lsp::Client;

use crate::document::DocumentData;

use self::{
    diagnostic_run_data::DiagnosticsRunData,
    tree_error_analysis::search_for_tree_error,
};

mod diagnostic_codes;
mod diagnostic_run_data;
mod expression_analysis;
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

    //Analyze the tree and get all the semantic data out of it
    //document.semantics = analyze_tree(&document.tree, &document.source);

    info!("Started Diagnostics");

    //search_for_tree_error(&mut diagnostic_data, document);

    info!("Finished Search");
    
    //analyze_expressions(&mut diagnostic_data, document);

    //statement_analysis(&mut diagnostic_data, document);

    //Once done send all diagnostic info to the client
    /*let err = tokio::time::timeout(Duration::from_millis(1000), client
    .publish_diagnostics(
        document.uri.clone(),
        diagnostic_data.total_diagnostics.clone(),
        Some(document.version),
    )).await;

    if err.is_err() {
        info!("Could not send diagnostics in under a second");
    }
    */

    diagnostic_data.total_diagnostics
}

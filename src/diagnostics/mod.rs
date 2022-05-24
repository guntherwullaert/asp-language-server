use tower_lsp::Client;

use crate::{document::DocumentData};

use self::{
    diagnostic_run_data::DiagnosticsRunData, statement_analysis::statement_analysis,
    tree_error_analysis::search_for_tree_error, tree_utils::analyze_tree, expression_analysis::analyze_expressions,
};

mod tree_utils;
mod diagnostic_run_data;
mod diagnostic_codes;
mod statement_analysis;
mod tree_error_analysis;
mod expression_analysis;

/**
 * Run the selected diagnostics on the parse tree
 */
pub async fn run_diagnostics(
    client: &Client,
    document: &DocumentData,
    maximum_number_of_problems: u32,
) {
    //Setup the diagnostics run data object to be used for this diagnostics run
    let mut diagnostic_data = DiagnosticsRunData {
        maximum_number_of_problems,
        current_number_of_problems: 0,
        total_diagnostics: Vec::new(),
    };

    //Analyze the tree and get all the semantic data out of it
    let semantics = analyze_tree(&document.tree);

    search_for_tree_error(&mut diagnostic_data, document, &semantics);

    analyze_expressions(&mut diagnostic_data, document, &semantics);

    statement_analysis(&mut diagnostic_data, document);

    //Once done send all diagnostic info to the client
    client
        .publish_diagnostics(
            document.uri.clone(),
            diagnostic_data.total_diagnostics.clone(),
            Some(document.version),
        )
        .await;
}
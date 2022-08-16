use tower_lsp::lsp_types::DiagnosticSeverity;
/*
use crate::document::DocumentData;

use super::{
    diagnostic_codes::DiagnosticsCode,
    diagnostic_run_data::DiagnosticsRunData,
    tree_utils::{TermSemantics, TermType},
};

/**
 * See if the semantics of the expressions in the encoding make sense
 */
pub fn analyze_expressions(diagnostic_data: &mut DiagnosticsRunData, document: &DocumentData) {
    // Check if there was a undefined operation
    let mut unknown_terms: Vec<TermSemantics> = Vec::new();
    for term in &document.semantics.terms {
        if term.value().kind == TermType::Unknown {
            let mut subset_of_other_range = false;
            for t in &unknown_terms {
                // If this unknown term is included in this term
                if term.range.start_byte > t.range.start_byte
                    && term.range.end_byte < t.range.end_byte
                {
                    subset_of_other_range = true;
                }
            }
            if !subset_of_other_range {
                unknown_terms.push(term.value().clone());
            }
        }
    }

    for term in &unknown_terms {
        for another_term in &unknown_terms {
            if !(term.range.start_byte > another_term.range.start_byte
                && term.range.end_byte < another_term.range.end_byte)
            {
                diagnostic_data.create_linter_diagnostic(
                    term.range,
                    DiagnosticSeverity::WARNING,
                    DiagnosticsCode::UndefinedOperation.into_i32(),
                    "Operation is undefined".to_string(),
                );
            }
        }
    }
}
*/
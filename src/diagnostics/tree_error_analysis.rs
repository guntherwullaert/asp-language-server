use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::{Range, Node};

use crate::{
    document::DocumentData,
};

use super::{diagnostic_run_data::DiagnosticsRunData, diagnostic_codes::DiagnosticsCode, tree_utils::{humanize_token, EncodingSemantics}};

pub struct ErrorSemantic {
    range: Range,
    prev_sibling_type: String
}

impl ErrorSemantic {
    pub fn new(node: &Node) -> ErrorSemantic {
        ErrorSemantic {
            range: node.range(),
            prev_sibling_type: node.prev_sibling().map_or_else(|| "", |n| n.kind()).to_string()
        }
    }
}

pub struct MissingSemantic {
    range: Range,
    missing: String 
}

impl MissingSemantic {
    pub fn new(range: Range, missing: &str) -> MissingSemantic {
        MissingSemantic {
            range,
            missing: missing.to_string()
        }
    }
}

/**
* Search for errors in the parse tree.
*/
pub fn search_for_tree_error(diagnostic_data: &mut DiagnosticsRunData, document: &DocumentData, semantics: &EncodingSemantics) {

    //Go through the errors found in the document
    for error in &semantics.errors {
        if error.prev_sibling_type == "statement" {
            //Found an error which is preceeded by an statement, most likely a . is missing
            diagnostic_data.create_tree_sitter_diagnostic(
                error.range,
                DiagnosticSeverity::ERROR,
                DiagnosticsCode::ExpectedDot.into_i32(),
                format!(
                    "syntax error while parsing value: '{}', expected: '.'",
                    Some(&document.source[error.range.start_byte..error.range.end_byte]).unwrap()
                ),
            );

            continue;
        }
        //If we reach here, we do not have a guess why the error occured
        diagnostic_data.create_tree_sitter_diagnostic(
            error.range,
            DiagnosticSeverity::ERROR,
            DiagnosticsCode::UnknownParseState.into_i32(),
            format!(
                "syntax error while parsing value: '{}'",
                Some(&document.source[error.range.start_byte..error.range.end_byte]).unwrap()
            ),
        );
    }

    for missing in &semantics.missing {
        //If node is missing, tell the user what we expected
        diagnostic_data.create_tree_sitter_diagnostic(
            missing.range,
            DiagnosticSeverity::ERROR,
            DiagnosticsCode::ExpectedMissingToken.into_i32(),
            format!(
                "syntax error while parsing, expected: '{}'",
                humanize_token(&missing.missing)
            ),
        );
    }
}

#[cfg(test)]
use crate::{
    test_utils::create_test_document,
    diagnostics::analyze_tree
};

#[test]
fn unknown_character_should_throw_unknown_parser_error() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();
    let doc = create_test_document("a b.".to_string());

    search_for_tree_error(&mut diags, &doc, &analyze_tree(&doc.tree));

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::UnknownParseState.into_i32())
    );
}

#[test]
fn if_parser_expects_dot_throw_dot_parser_error() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();
    let doc = create_test_document("a. d c :- a.".to_string());

    search_for_tree_error(&mut diags, &doc, &analyze_tree(&doc.tree));

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::ExpectedDot.into_i32())
    );
}

#[test]
fn if_parser_misses_token_throw_missing_token() {
    let mut diags = DiagnosticsRunData::create_test_diagnostics();
    let doc = create_test_document("a(b.".to_string());

    search_for_tree_error(&mut diags, &doc, &analyze_tree(&doc.tree));

    assert_eq!(diags.total_diagnostics.len(), 1);

    assert_eq!(
        format!(
            "{:?}",
            diags
                .total_diagnostics
                .get(0)
                .unwrap()
                .code
                .clone()
                .unwrap()
        ),
        format!("Number({})", DiagnosticsCode::ExpectedMissingToken.into_i32())
    );
}

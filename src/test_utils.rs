use std::str::FromStr;

use crate::{document::DocumentData};

use ropey::Rope;
use tower_lsp::lsp_types::Url;
use tree_sitter::Parser;

pub fn create_test_document(source: String) -> DocumentData {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_clingo::language())
        .expect("Error loading clingo grammar");

    let tree = parser.parse(source.clone(), None).unwrap();

    let mut doc = DocumentData::new(Url::from_str("file://test.lp").unwrap(), tree, Rope::from_str(&source), 1);
    doc.generate_semantics(None);
    doc
}

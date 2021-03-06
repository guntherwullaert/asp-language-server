use std::str::FromStr;

use tower_lsp::lsp_types::Url;
use tree_sitter::Parser;

use crate::document::DocumentData;

pub fn create_test_document(source: String) -> DocumentData {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_clingo::language())
        .expect("Error loading clingo grammar");

    let tree = parser.parse(source.clone(), None).unwrap();

    DocumentData::new(Url::from_str("file://test.lp").unwrap(), tree, source, 1)
}

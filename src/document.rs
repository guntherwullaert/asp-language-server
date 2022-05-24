use tower_lsp::lsp_types::Url;
use tree_sitter::Tree;

use crate::diagnostics::tree_utils::EncodingSemantics;

#[derive(Debug, Clone)]
pub struct DocumentData {
    pub uri: Url,
    pub tree: Tree,
    pub source: String,
    pub version: i32,
    pub semantics: EncodingSemantics,
}
impl DocumentData {
    pub fn new(uri: Url, tree: Tree, source: String, version: i32) -> DocumentData {
        DocumentData {
            uri,
            tree,
            source,
            version,
            semantics: EncodingSemantics::new(),
        }
    }
}

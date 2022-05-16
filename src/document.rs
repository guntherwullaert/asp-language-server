use tower_lsp::lsp_types::Url;
use tree_sitter::Tree;

#[derive(Debug, Clone)]
pub struct DocumentData {
    pub uri: Url,
    pub tree: Tree,
    pub source: String,
    pub version: i32,
}
impl DocumentData {
    pub fn new(uri: Url, tree: Tree, source: String, version: i32) -> DocumentData{
        return DocumentData { uri, tree, source, version }
    }
}
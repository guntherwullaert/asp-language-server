use tree_sitter::{Range, Node};

/**
 * Contains any semantic information around an error
 */
#[derive(Clone, Debug)]
pub struct ErrorSemantic {
    /**
     * Where the error occured in the source code
     */
    pub range: Range,

    /**
     * What kind of sibling was in front of the error
     */
    pub prev_sibling_type: String,
}

impl ErrorSemantic {

    /**
     * Create a new ErrorSemantic struct based on a node
     */
    pub fn new(node: &Node) -> ErrorSemantic {
        ErrorSemantic {
            range: node.range(),
            prev_sibling_type: node
                .prev_sibling()
                .map_or_else(|| "", |n| n.kind())
                .to_string(),
        }
    }
}
use tree_sitter::Range;

/**
 * Holds all the information around missing tokens in the abstract syntax tree
 */
#[derive(Clone, Debug)]
pub struct MissingSemantic {
    /**
     * The range of where the missing tokens occurred
     */
    pub range: Range,

    /**
     * What token is missing
     */
    pub missing: String,
}

impl MissingSemantic {
    pub fn new(range: Range, missing: &str) -> MissingSemantic {
        MissingSemantic {
            range,
            missing: missing.to_string(),
        }
    }
}
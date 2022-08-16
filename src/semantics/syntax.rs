use super::{error_semantic::ErrorSemantic, encoding_semantic::Semantics, missing_semantic::MissingSemantic};

/**
 * Syntax searches for any errors in the abstract syntax tree or looks for any missing nodes like brackets that have not been closed
 */
#[derive(Clone, Debug)]
pub struct Syntax {
    errors: Vec<ErrorSemantic>,
    missing: Vec<MissingSemantic>,
}

impl Syntax {
    pub fn new() -> Syntax {
        Syntax {
            errors: Vec::new(),
            missing: Vec::new()
        }
    }

    pub fn get_errors(&self) -> Vec<ErrorSemantic> {
        self.errors.clone()
    }

    pub fn get_missing(&self) -> Vec<MissingSemantic> {
        self.missing.clone()
    }
}

impl Semantics for Syntax {
    fn on_node(node: tree_sitter::Node, document: &mut crate::document::DocumentData) {
        if node.is_error() {
            //TODO: If this error supersedes an error we encountered before, remove that error ? 
            // If the node contains an error we add it to the list of errors
            document.semantics.syntax.errors.push(ErrorSemantic::new(&node));
        } else if node.is_missing() {
            // Save where something is missing and what is missing
            document.semantics.syntax.missing.push(MissingSemantic::new(node.range(), node.kind()));
        }
    }

    fn startup(document: &mut crate::document::DocumentData) {
        document.semantics.syntax.errors = Vec::new();
        document.semantics.syntax.missing = Vec::new();
    }
}
use tower_lsp::lsp_types::{Location, Position};
use tree_sitter::Point;

use crate::{
    document::DocumentData, semantics::predicate_occurence_semantics::PredicateOccurenceLocation,
};

use super::get_occurences_for_predicate;

/**
 * Check and find the definition for an predicate at this position
 */
pub fn check_goto_definition(document: &DocumentData, position: Position) -> Option<Vec<Location>> {
    || -> Option<Vec<Location>> {
        let node = document.tree.root_node().descendant_for_point_range(
            Point {
                row: position.line as usize,
                column: (position.character) as usize,
            },
            Point {
                row: position.line as usize,
                column: (position.character) as usize,
            },
        );

        let ret =
            get_occurences_for_predicate(document, node, vec![PredicateOccurenceLocation::Head]);

        Some(ret)
    }()
}

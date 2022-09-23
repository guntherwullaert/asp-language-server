use tower_lsp::lsp_types::{lsif::Document, Location, Position, Range, GotoDefinitionResponse};
use tree_sitter::Point;

use crate::{document::DocumentData, semantics::predicate_occurence_semantics::PredicateOccurenceLocation};

use super::get_occurences_for_predicate;

/**
 * Check and find the definition for an predicate at this position
 */
pub fn check_goto_definition(document: &DocumentData, position: Position) -> Option<Vec<Location>> {
    || -> Option<Vec<Location>> {
        //TODO: Keep track if analysis has been done yet
        //let semantics = analyze_tree(&document.tree, &document.source);

        //TODO: Have a function to get the node instead of a duplicate
        let mut node = document.tree.root_node().descendant_for_point_range(
            Point { row: position.line as usize, column: (position.character) as usize }, 
            Point { row: position.line as usize, column: (position.character) as usize }
        );

        let mut ret = get_occurences_for_predicate(document, node, vec![PredicateOccurenceLocation::Head]);

        Some(ret)
    }()
}
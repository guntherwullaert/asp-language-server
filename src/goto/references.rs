use log::info;
use tower_lsp::lsp_types::{Position, Location};
use tree_sitter::Point;

use crate::{document::DocumentData, semantics::predicate_occurence_semantics::PredicateOccurenceLocation};

use super::get_occurences_for_predicate;

/**
 * Check and find the references to an predicate at this position
 */
pub fn check_goto_references(document: &DocumentData, position: Position) -> Option<Vec<Location>> {
    || -> Option<Vec<Location>> {
        //TODO: Keep track if analysis has been done yet
        //let semantics = analyze_tree(&document.tree, &document.source);

        //TODO: Have a function to get the node instead of a duplicate
        let mut node = document.tree.root_node().descendant_for_point_range(
            Point { row: position.line as usize, column: (position.character) as usize }, 
            Point { row: position.line as usize, column: (position.character) as usize }
        );

        info!("Predicates: {:?}", document.semantics.predicate_semantics.predicates);

        let ret = get_occurences_for_predicate(document, node, vec![PredicateOccurenceLocation::Body, PredicateOccurenceLocation::Condition]);

        Some(ret)
    }()
}
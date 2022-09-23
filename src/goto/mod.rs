use tower_lsp::lsp_types::{Range, Position, Location};
use tree_sitter::Node;

use crate::{semantics::predicate_occurence_semantics::PredicateOccurenceLocation, document::DocumentData};

pub mod definition;
pub mod references;

/**
 * Obtain the occurences for a specific predicate
 * identifier: The identifier of this predicate
 * arity: The arity of this predicate
 * locations: Which location the predicate needs to be to be counted as an occurence
 */
pub fn get_occurences_for_predicate(document: &DocumentData, starting_node: Option<Node>, locations: Vec<PredicateOccurenceLocation>) -> Vec<Location>{
    let mut node = starting_node;
    let mut ret = Vec::new();
    while node.is_some() {
        // If we have an predicate with an identifier
        if (node.unwrap().kind() == "atom" || node.unwrap().kind() == "term") && node.unwrap().child_count() >= 3 && node.unwrap().child(0).unwrap().kind() == "identifier" {
            //TODO: Maybe create a function for this ?!?
            let node_identifier = document.get_source_for_range(node.unwrap().child(0).unwrap().range());
            let node_arity = document.semantics.predicate_semantics.get_predicates_arity_for_node(&node.unwrap().child(2).unwrap().id()) + 1;

            for ((identifier, arity), occurences) in document.semantics.predicate_semantics.predicates.clone() {
                // Find if this is the correct identifier and arity 
                if identifier == node_identifier && arity == node_arity {
                    // Return all occurences that are in the head
                    for occurence in occurences {
                        if locations.contains(&occurence.location) {
                            let range = Range::new(
                                Position { line: occurence.range.start_point.row as u32, character: occurence.range.start_point.column as u32}, 
                                Position { line: occurence.range.end_point.row as u32, character: occurence.range.end_point.column as u32}
                            );

                            ret.push(Location::new(document.uri.clone(), range));
                        }
                    }

                    break;
                }
            }
            break;
        }
        node = node.unwrap().parent();
    }
    ret
}
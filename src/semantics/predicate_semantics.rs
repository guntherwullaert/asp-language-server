use std::collections::HashSet;

use dashmap::DashMap;
use im_rc::HashMap;
use log::info;

use super::{predicate_occurence_semantics::{PredicateOccurenceSemantics, PredicateOccurenceLocation}, encoding_semantic::{EncodingSemantics, Semantics}, statement_semantic::StatementSemantics};

/**
 * Predicate Semantics infers information what and where predicates occur in the encoding
 */
#[derive(Clone, Debug)]
pub struct PredicateSemantics {
    pub predicates: DashMap<(String, usize), HashSet<PredicateOccurenceSemantics>>,
    pub predicates_arity: DashMap<usize, usize>
}

impl PredicateSemantics {
    pub fn new() -> PredicateSemantics {
        PredicateSemantics {
            predicates: DashMap::new(),
            predicates_arity: DashMap::new()
        }
    }

    /**
     * Update predicates for a node, if there is no semantics object for that node it creates one
     */
    pub fn insert_predicate_for_node(semantics: &EncodingSemantics, identifier: String, arity: usize, new_value: PredicateOccurenceSemantics) {
        if semantics.predicate_semantics.predicates.contains_key(&(identifier.clone(), arity)) {
            semantics.predicate_semantics.predicates.get_mut(&(identifier, arity)).unwrap().insert(new_value);
            return;
        }

        let mut hash = HashSet::new();
        hash.insert(new_value);
        semantics.predicate_semantics.predicates.insert((identifier, arity), hash);
    }

    /**
     * Returns the amount of termvecs in this part of the encoding
     */
    pub fn get_predicates_arity_for_node(&self, node: &usize) -> usize {
        if self.predicates_arity.contains_key(node) {
            return self.predicates_arity.get(node).unwrap().value().clone();
        }
        0
    }
}

//TODO: Split this to finding the arity only when changing
impl Semantics for PredicateSemantics {
    fn on_node(node: tree_sitter::Node, document: &mut crate::document::DocumentData) {
        //Find all predicates with their arity
        match node.kind() {
            "atom" | "term" => {
                if node.child_count() >= 3 && node.child(0).unwrap().kind() == "identifier" {
                    let identifier = document.get_source_for_range(node.child(0).unwrap().range());
                    let arity = document.semantics.predicate_semantics.get_predicates_arity_for_node(&node.child(2).unwrap().id()) + 1;

                    let mut location = PredicateOccurenceLocation::Head;
                    let mut parent = node.parent();
                    while parent.is_some() {
                        match parent.unwrap().kind() {
                            "bodydot" => location = PredicateOccurenceLocation::Body,
                            "optcondition" => location = PredicateOccurenceLocation::Condition,
                            _ => {}
                        }
                        parent = parent.unwrap().parent();
                    }

                    Self::insert_predicate_for_node(&document.semantics, identifier.clone(), arity, PredicateOccurenceSemantics {
                        node_id: node.id(),
                        range: node.range(),
                        location
                    });
                }
            }
            "termvec" | "argvec" => {
                let mut arity = 0;

                for child in node.children(&mut node.walk()) {
                    arity += document.semantics.predicate_semantics.get_predicates_arity_for_node(&child.id());
                }
                document.semantics.predicate_semantics.predicates_arity.insert(node.id(), arity);
            }
            "COMMA" => {
                document.semantics.predicate_semantics.predicates_arity.insert(node.id(), 1);
            }
            _ => {}
        }
    }

    fn startup(document: &mut crate::document::DocumentData) {
        document.semantics.predicate_semantics.predicates = DashMap::new();
    }
}
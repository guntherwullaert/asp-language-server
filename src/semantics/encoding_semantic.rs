use dashmap::{DashMap, DashSet};
use im_rc::HashSet;
use log::info;
use rust_lapper::Lapper;
use serde::__private::doc;
use tree_sitter::{Node, Range};

use crate::document::DocumentData;

use super::{error_semantic::{ErrorSemantic}, syntax::Syntax, statement_semantic::{StatementSemantics, self}, term_semantic::TermSemantic, predicate_semantics::PredicateSemantics};

/**
 * Encoding semantics are all the information needed about the program that then can be used by the other parts of the LSP
 */
#[derive(Clone, Debug)]
pub struct EncodingSemantics {
    pub syntax: Syntax,
    pub predicate_semantics: PredicateSemantics,
    pub statement_semantics: DashMap<usize, StatementSemantics>,
    pub old_node_ids_encountered: DashSet<usize>,
    pub node_ids_encountered: DashSet<usize>
}

impl EncodingSemantics {
    pub fn new() -> EncodingSemantics {
        EncodingSemantics { 
            syntax: Syntax::new(),
            predicate_semantics: PredicateSemantics::new(),
            statement_semantics: DashMap::new(),
            old_node_ids_encountered: DashSet::new(),
            node_ids_encountered: DashSet::new()
        }
    }

    /**
     * This can be used if any cleanup of previous iterations needs to be done to the document and is called just before analysis starts
     */
    pub fn startup(document: &mut DocumentData) {
        document.semantics.node_ids_encountered = document.semantics.old_node_ids_encountered.clone();
        document.semantics.old_node_ids_encountered = DashSet::new();

        Syntax::startup(document);
    }

    /**
     * On discovering a node this function gets called and all the analyzers need to decide what this means now
     */
    pub fn on_node(node: Node, document: &mut DocumentData, changed_ranges: &Option<Lapper<usize, usize>>) {
        document.semantics.node_ids_encountered.remove(&node.id());

        // Check if node is affected by the changes
        // This is quite expensive to find out
        // We sadly have to check if the key is in use, because sometimes node id's are changed that are not in the changed nodes list
        if let Some(ranges) = changed_ranges {
            if ranges.find(node.range().start_byte, node.range().end_byte).any(|_| true) || !document.semantics.statement_semantics.contains_key(&node.id()) {
                EncodingSemantics::checks_on_only_affected_area(node, document);
            }
            /*for (start_byte, end_byte) in ranges {
                if node.range().start_byte < *end_byte && node.range().end_byte > *start_byte {
                    // Perform checks that only care about the affected area
                    EncodingSemantics::checks_on_only_affected_area(node, document);
                    break;
                }
            }*/
        } else {
            // For first check we check everything
            EncodingSemantics::checks_on_only_affected_area(node, document);
        }

        // Perform any checks that need to be done regardless of changes
        EncodingSemantics::checks_that_always_need_to_happen(node, document);
    }

    /**
     * This will be called any time an affected area by changes has changed
     */
    fn checks_on_only_affected_area(node: Node, document: &mut DocumentData) {
        TermSemantic::on_node(node, document);
        StatementSemantics::on_node(node, document);
    }

    /**
     * This will be called everytime we check the document for semantics
     */
    fn checks_that_always_need_to_happen(node: Node, document: &mut DocumentData) {
        PredicateSemantics::on_node(node, document);
        Syntax::on_node(node, document);
    }

    /**
     * Cleanup any node ids that do not excist anymore
     */
    pub fn cleanup(document: &mut DocumentData) {
        for id in document.semantics.node_ids_encountered.iter() {
            //If an ID has not been removed, we can free that memory from other node lists
            document.semantics.statement_semantics.remove(&id);
        }

        // Afterwards add all used nodes to the old list
        document.semantics.old_node_ids_encountered = DashSet::with_capacity(document.semantics.statement_semantics.len());
        for refmulti in document.semantics.statement_semantics.iter() {
            document.semantics.old_node_ids_encountered.insert(*refmulti.key());
        }
    }

    /**
     * Get a clone of the statement semantics for a specific node, if none where found a new StatementSemantics object will be generated
     */
    pub fn get_statement_semantics_for_node(&self, node_id: usize) -> StatementSemantics {
        if self.statement_semantics.contains_key(&node_id) {
            return self.statement_semantics.get(&node_id).unwrap().value().clone();
        }

        StatementSemantics::new()
    }
}

/**
 * Each of the semantic analyzers need to implement the on_node function that will be called on each node
 */
pub trait Semantics {
    /**
     * On discovering a node this function gets called and all the analyzers need to decide what this means now
     */
    fn on_node(node: Node, document: &mut DocumentData);

    /**
     * This can be used if any cleanup of previous iterations needs to be done to the document and is called just before analysis starts
     */
    fn startup(document: &mut DocumentData) {

    }
}
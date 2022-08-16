use std::collections::HashSet;

use super::{error_semantic::ErrorSemantic, encoding_semantic::{Semantics, EncodingSemantics}, missing_semantic::MissingSemantic};

/**
 * Statement Semantics infers information from the abstract syntax tree about statements and their parts.
 * Many of these fields are later used in the safety analysis
 */
#[derive(Clone, Debug)]
pub struct StatementSemantics {
    /**
     * Which variables are contained in this part of the encoding
     */
    pub vars: HashSet<String>,
}

impl StatementSemantics {
    pub fn new() -> StatementSemantics {
        StatementSemantics {
            vars: HashSet::new(),
        }
    }

    /**
     * Get statement semantics for a specific node, if none where found a new StatementSemantics object will be generated
     */
    pub fn get_statement_semantics_for_node(semantics: &EncodingSemantics, node_id: usize) -> StatementSemantics {
        if semantics.statement_semantics.contains_key(&node_id) {
            return semantics.statement_semantics.get(&node_id).unwrap().value().clone();
        }

        StatementSemantics::new()
    }

    /**
     * Update a vars for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_vars_for_node(semantics: &EncodingSemantics, node_id: usize, new_value: HashSet<String>) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics.statement_semantics.get_mut(&node_id).unwrap().vars = new_value;
            return;
        }

        semantics.statement_semantics.insert(node_id, StatementSemantics { vars: new_value });
    }

    /**
     * Check if a variable occurs in this node, if not we pass on the variables in our children. 
     */
    fn check_for_variables(node: tree_sitter::Node, document: &mut crate::document::DocumentData) {
        match node.kind() {
            "VARIABLE" => {
                let mut set = HashSet::new();
                set.insert(document.get_source_for_range(node.range()));
                StatementSemantics::update_vars_for_node(&document.semantics, node.id(), set);
            }
            "source_file" => {} // Ignore any fields above statements
            _ => {
                let mut vars_in_children = HashSet::new();
                for child in node.children(&mut node.walk()) {
                    vars_in_children.extend(StatementSemantics::get_statement_semantics_for_node(&document.semantics, child.id()).vars);
                }
                StatementSemantics::update_vars_for_node(&document.semantics, node.id(), vars_in_children);
            }
        }
    }
}

impl Semantics for StatementSemantics {
    fn on_node(node: tree_sitter::Node, document: &mut crate::document::DocumentData) {
        StatementSemantics::check_for_variables(node, document);
    }
}
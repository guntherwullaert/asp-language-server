use std::collections::HashSet;

use ropey::RopeSlice;
use tree_sitter::Node;

use crate::document::DocumentData;

use super::{error_semantic::ErrorSemantic, encoding_semantic::{Semantics, EncodingSemantics}, missing_semantic::MissingSemantic, term_semantic::{TermSemantic, TermType}};

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

    /**
     * Which variables are provided by this part of the encoding
     */
    pub provide: HashSet<String>,

    /**
     * For a term in a statement this struct contains the information needed to understand what this term is
     */
    pub term: TermSemantic,
}

impl StatementSemantics {
    pub fn new() -> StatementSemantics {
        StatementSemantics {
            vars: HashSet::new(),
            provide: HashSet::new(),
            term: TermSemantic::new(),
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
     * Update vars for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_vars_for_node(semantics: &EncodingSemantics, node_id: usize, new_value: HashSet<String>) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics.statement_semantics.get_mut(&node_id).unwrap().vars = new_value;
            return;
        }

        semantics.statement_semantics.insert(node_id, StatementSemantics { vars: new_value, provide: HashSet::new(), term: TermSemantic::new() });
    }

    /**
     * Update provide for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_provide_for_node(semantics: &EncodingSemantics, node_id: usize, new_value: HashSet<String>) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics.statement_semantics.get_mut(&node_id).unwrap().provide = new_value;
            return;
        }

        semantics.statement_semantics.insert(node_id, StatementSemantics { provide: new_value, vars: HashSet::new(), term: TermSemantic::new() });
    }

    /**
     * Check if a variable occurs in this node, if not we pass on the variables in our children. 
     */
    fn check_for_variables(node: Node, document: &mut DocumentData) {
        match node.kind() {
            "VARIABLE" => {
                let mut set = HashSet::new();
                set.insert(document.get_source_for_range(node.range()));
                Self::update_vars_for_node(&document.semantics, node.id(), set);
            }
            "source_file" => {} // Ignore any fields above statements
            _ => {
                let mut vars_in_children = HashSet::new();
                for child in node.children(&mut node.walk()) {
                    vars_in_children.extend(Self::get_statement_semantics_for_node(&document.semantics, child.id()).vars);
                }
                Self::update_vars_for_node(&document.semantics, node.id(), vars_in_children);
            }
        }
    }

    /**
     * Returns true if we could evaluate a value for this term (only possible if term is a constant) 
     */
    pub fn is_evaluable(node: usize, document: &mut DocumentData) -> bool {
        let semantic = Self::get_statement_semantics_for_node(&document.semantics, node);

        if semantic.term.kind == TermType::Constant {
            return true;
        }
        false
    }

    /**
     * Combine every provide in the children of node and set this as the provide for this node
     */
    fn pass_on_provide_from_children(node: Node, document: &mut DocumentData) {
        let mut provide_in_children = HashSet::new();
        for child in node.children(&mut node.walk()) {
            provide_in_children.extend(Self::get_statement_semantics_for_node(&document.semantics, child.id()).provide);
        }
        Self::update_provide_for_node(&document.semantics, node.id(), provide_in_children);
    }

    /**
     * Check what variables a part of a statement provides
     */
    fn check_provide(node: Node, document: &mut DocumentData) {
        match node.kind() {
            "NUMBER" | "identifier" => {}, //IGNORE if required an emptyset will be returned by default
            "VARIABLE" => {
                //Return a set containing this variable
                let mut set = HashSet::new();
                set.insert(document.get_source_for_range(node.range()));
                Self::update_provide_for_node(&document.semantics, node.id(), set);
            }
            "term" => {
                // If we only have 1 child we pass on the provide for the child
                if node.child_count() == 1 {
                    Self::pass_on_provide_from_children(node, document);
                } else if node.child_count() >= 3 {
                    let left_child = node.child(0).unwrap();
                    let operator = node.child(1).unwrap();
                    let right_child = node.child(2).unwrap();
                    match operator.kind() {
                        "LPAREN" => {
                            // We have an term of form f(t) pass on the provide value of the child
                            Self::pass_on_provide_from_children(node, document);
                        }
                        "ADD" | "SUB" => {
                            // We have an term of form a \star b
                            if Self::is_evaluable(left_child.id(), document) {
                                // If the left term is a constant then we pass on the provide value of the right child
                                Self::update_provide_for_node(&document.semantics, node.id(), Self::get_statement_semantics_for_node(&document.semantics, right_child.id()).provide);
                            } else if Self::is_evaluable(right_child.id(), document) {
                                // If the right term is a constant then we pass on the provide value of the left child
                                Self::update_provide_for_node(&document.semantics, node.id(), Self::get_statement_semantics_for_node(&document.semantics, left_child.id()).provide);
                            }
                        }
                        "MUL" => {
                            // We have an term of form a * b
                            if Self::is_evaluable(left_child.id(), document) && !Self::get_statement_semantics_for_node(&document.semantics, left_child.id()).term.value.contains(&0) {
                                // If the left term is a constant then we pass on the provide value of the right child
                                semantics.provide.insert(node.id(), semantics.get_provide_for_node(&right_child.id()));
                            } else if semantics.is_evaluable(&right_child.id()) && !semantics.get_evaluation_for_term(&right_child.id()).contains(&0) {
                                // If the right term is a constant then we pass on the provide value of the left child
                                semantics.provide.insert(node.id(), semantics.get_provide_for_node(&left_child.id()));
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

impl Semantics for StatementSemantics {
    fn on_node(node: Node, document: &mut DocumentData) {
        StatementSemantics::check_for_variables(node, document);
        StatementSemantics::check_provide(node, document);
    }
}
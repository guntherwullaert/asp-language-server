use std::collections::HashSet;

use log::info;
use tree_sitter::{Range, Node, Point};

use crate::document::DocumentData;

use super::{statement_semantic::StatementSemantics, encoding_semantic::Semantics};

/**
 * What type of operation this term is
 */
#[derive(Clone, Debug)]
pub enum TermOperator {
    None,
    Add,
    Sub,
    Mul,
    Div,
    Dots,
}

/**
 * What type of term we have her
 */
#[derive(Clone, Debug, PartialEq)]
pub enum TermType {
    Unknown,
    Identifier,
    Constant,
    Variable,
}

/**
 * For a term in a statement this struct contains the information needed to understand what this term is
 */
#[derive(Clone, Debug)]
pub struct TermSemantic {
    pub operator: TermOperator,
    pub kind: TermType,
    pub value: HashSet<usize>,
    pub range: Range,
}

impl TermSemantic {
    pub fn new() -> TermSemantic {
        TermSemantic { 
            operator: TermOperator::None, 
            kind: TermType::Unknown, 
            value: HashSet::new(), 
            range: Range { start_byte: 0, end_byte: 0, start_point: Point { row: 0, column: 0 }, end_point: Point { row: 0, column: 0 } }
        }
    }

    pub fn from_node(node: Node, document: &mut DocumentData) -> TermSemantic {
        let mut kind = TermType::Unknown;
        let mut operator = TermOperator::None;
        let mut value = HashSet::new();

        match node.kind() {
            "dec" | "NUMBER" => {
                kind = TermType::Constant;
                value.insert(document.get_source_for_range(node.range()).parse::<usize>().unwrap_or_default());
            },
            "VARIABLE" => kind = TermType::Variable,
            "identifier" => kind = TermType::Identifier,
            "term" => {
                // We have a term, find out based on the children what type of term we have
                if node.child_count() == 1 {
                    //If we only have one child we pass on the values of that child
                    let child = document.semantics.get_statement_semantics_for_node(node.child(0).unwrap().id()).term;

                    kind = child.kind.clone();
                    value = child.value.clone();
                } else if node.child_count() > 2 {
                    match node.child(1).unwrap().kind() {
                        //If we have more than 2 children and the second child is an operation
                        "ADD" => operator = TermOperator::Add,
                        "SUB" => operator = TermOperator::Sub,
                        "MUL" => operator = TermOperator::Mul,
                        "SLASH" => operator = TermOperator::Div,
                        "DOTS" => operator = TermOperator::Dots,
                        //Or we have a identfier then we create a new term object
                        "LPAREN" => {
                            kind = TermType::Identifier;
                            return TermSemantic {
                                operator,
                                kind,
                                value,
                                range: node.range(),
                            };
                        }
                        _ => {}
                    }
                    //We check what the children are
                    let left_child = document.semantics.get_statement_semantics_for_node(node.child(0).unwrap().id());
                    let right_child = document.semantics.get_statement_semantics_for_node(node.child(2).unwrap().id());

                    if left_child.term.kind == TermType::Constant && right_child.term.kind == TermType::Constant {
                        //If both children are constant the resulting term is constant and we can evaluate the value of the constant
                        kind = TermType::Constant;
                        value = TermSemantic::evaluate(&left_child.term, &right_child.term, &operator);
                    } else if (left_child.term.kind == TermType::Variable && right_child.term.kind == TermType::Constant) || (left_child.term.kind == TermType::Constant && right_child.term.kind == TermType::Variable) || (left_child.term.kind == TermType::Variable && right_child.term.kind == TermType::Variable) {
                        kind = TermType::Variable
                    } else {
                        kind = TermType::Unknown
                    }
                }
            }
            _ => {}
        }

        TermSemantic {
            operator,
            kind,
            value,
            range: node.range(),
        }
    }

    pub fn evaluate(a : &TermSemantic, b : &TermSemantic, op : &TermOperator) -> HashSet<usize> {
        let mut result_set = HashSet::new();

        match op {
            TermOperator::Add => {
                for s_i in a.value.clone() {
                    for s_j in b.value.clone() {
                        result_set.insert(s_i + s_j);
                    }
                }
            },
            TermOperator::Sub => {
                for s_i in a.value.clone() {
                    for s_j in b.value.clone() {
                        result_set.insert(s_i - s_j);
                    }
                }
            },
            TermOperator::Mul => {
                for s_i in a.value.clone() {
                    for s_j in b.value.clone() {
                        result_set.insert(s_i * s_j);
                    }
                }
            },
            TermOperator::Div => {
                for s_i in a.value.clone() {
                    for s_j in b.value.clone() {
                        if s_j != 0 {
                            result_set.insert(s_i / s_j);
                        }
                    }
                }
            },
            _ => {}
        }

        result_set
    }

    /**
     * Negates an comparison operator provided and returns the new operator as a string
     */
    pub fn negate_comparison_operator(operator: &str) -> &str {
        match operator {
            "NEQ" => "EQ",
            "EQ" => "NEQ",
            "LT" => "GEQ",
            "GT" => "LEQ",
            "LEQ" => "GT",
            "GEQ" => "LT",
            _ => ""
        }
    }
}

impl Semantics for TermSemantic {
    fn on_node(node: Node, document: &mut DocumentData) {
        match node.kind() {
            "dec" | "NUMBER" | "term" | "VARIABLE" | "identifier" => {
                let term = TermSemantic::from_node(node, document);
                StatementSemantics::update_term_for_node(&document.semantics, node.id(), term);
            }
            _ => {}
        }
    }
}
use std::collections::HashSet;

use crate::document::DocumentData;
use tree_sitter::Node;

/**
 * What type a literal is
 */
#[derive(Clone, Debug)]
pub enum LiteralType {
    Normal,
    Conjunction,
    AggregateElement,
    Disjunction,
}

/**
 * Special Literal semantics contain all the information needed around a conditional literal or aggregate
 */
#[derive(Clone, Debug)]
pub struct SpecialLiteralSemantics {
    pub id: usize,
    pub kind: LiteralType,
    pub local_dependency: Vec<(HashSet<String>, HashSet<String>)>,
}

impl SpecialLiteralSemantics {
    pub fn new(node: &Node, document: &DocumentData) -> SpecialLiteralSemantics {
        let mut local_dependency: Vec<(HashSet<String>, HashSet<String>)> = Vec::new();

        match node.kind() {
            "conjunction" | "disjunction" => {
                if node.child_count() == 3 {
                    let l0 = node.child(0).unwrap();
                    let condition = node.child(2).unwrap();

                    local_dependency.push((
                        HashSet::new(),
                        document
                            .semantics
                            .get_statement_semantics_for_node(l0.id())
                            .vars,
                    ));
                    local_dependency.extend(
                        document
                            .semantics
                            .get_statement_semantics_for_node(condition.id())
                            .dependencies,
                    );
                }
            }
            "bodyaggrelem" => {
                if node.child_count() >= 2 {
                    let terms = node.child(0).unwrap();
                    let condition = node.child(1).unwrap();

                    local_dependency.push((
                        HashSet::new(),
                        document
                            .semantics
                            .get_statement_semantics_for_node(terms.id())
                            .vars,
                    ));
                    local_dependency.extend(
                        document
                            .semantics
                            .get_statement_semantics_for_node(condition.id())
                            .dependencies,
                    );
                }
            }
            _ => {}
        }

        SpecialLiteralSemantics {
            id: node.id(),
            kind: match node.kind() {
                "conjunction" => LiteralType::Conjunction,
                "bodyaggrelem" => LiteralType::AggregateElement,
                "altheadaggrelemvec" => LiteralType::AggregateElement,
                "disjunction" => LiteralType::Disjunction,
                _ => LiteralType::Normal,
            },
            local_dependency,
        }
    }

    pub fn new_with_dep(
        node: &Node,
        local_dependency: Vec<(HashSet<String>, HashSet<String>)>,
    ) -> SpecialLiteralSemantics {
        SpecialLiteralSemantics {
            id: node.id(),
            kind: match node.kind() {
                "conjunction" => LiteralType::Conjunction,
                "bodyaggrelem" => LiteralType::AggregateElement,
                "altheadaggrelemvec" => LiteralType::AggregateElement,
                "disjunction" => LiteralType::Disjunction,
                _ => LiteralType::Normal,
            },
            local_dependency,
        }
    }
}

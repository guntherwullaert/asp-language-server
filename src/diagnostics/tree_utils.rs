use dashmap::DashMap;
use tree_sitter::{Query, QueryCursor, TreeCursor, Node, Tree, Range};

use super::tree_error_analysis::{MissingSemantic, ErrorSemantic};

/**
 * Convert a token value into a human readable string
 */
pub fn humanize_token(token: &str) -> &str {
    match token {
        "RPAREN" => ")",
        "LPAREN" => "(",
        "RBRACE" => "{",
        "LBRACE" => "}",
        "DOT" => ".",
        _ => token,
    }
}

/**
 * Retrace back to where we can continue walking
 */
pub fn retrace(mut cursor: TreeCursor) -> (TreeCursor, bool) {
    let mut retracing = true;
    let mut reached_root = false;
    while retracing {
        if !cursor.goto_parent() {
            retracing = false;
            reached_root = true;
        }

        if cursor.goto_next_sibling() {
            retracing = false;
        }
    }
    (cursor, reached_root)
}

/**
 * Do a simple query on a part of the parse tree and return the captures
 */
#[allow(dead_code)]
pub fn do_simple_query<'a>(
    query_string: &'a str,
    node: tree_sitter::Node<'a>,
    source: &'a [u8],
) -> std::vec::Vec<(tree_sitter::Range, &'a str, tree_sitter::Node<'a>)> {
    let mut query_cursor = QueryCursor::new();
    let query = Query::new(tree_sitter_clingo::language(), query_string).unwrap();

    let matches = query_cursor.matches(&query, node, source);
    let mut output = Vec::new();

    for each_match in matches {
        for capture in each_match.captures.iter() {
            let range = capture.node.range();
            let name = capture.node.utf8_text(source).unwrap();

            output.push((range, name, capture.node));
        }
    }

    output
}

/**
 * Encoding semantics are all the information needed about the program that then can be used by the other parts of the LSP
 */
pub struct EncodingSemantics {
    pub errors: Vec<ErrorSemantic>,
    pub missing: Vec<MissingSemantic>,
    pub terms: DashMap<usize, TermSemantics>
}

impl EncodingSemantics {
    pub fn new() -> EncodingSemantics {
        EncodingSemantics {
            errors: Vec::new(),
            missing: Vec::new(),
            terms: DashMap::new()
        }
    }
}

#[derive(Clone)]
pub struct TermSemantics {
    pub operator: TermOperator,
    pub kind: TermType,
    pub range: Range,
}

impl TermSemantics {
    pub fn new (node : &Node, terms: &DashMap<usize, TermSemantics>) -> TermSemantics {
        let mut kind = TermType::Unknown;
        let mut operator = TermOperator::None;

        match node.kind() {
            "dec" | "NUMBER" => {
                kind = TermType::Constant
            }
            "VARIABLE" => {
                kind = TermType::Variable
            }
            "identifier" => {
                kind = TermType::Identifier
            }
            "term" => {
                if node.child_count() == 1 && terms.contains_key(&node.child(0).unwrap().id()){
                    kind = terms.get(&node.child(0).unwrap().id()).unwrap().kind.clone();
                }
                else if node.child_count() > 2 {
                    match node.child(1).unwrap().kind() {
                        "ADD" => operator = TermOperator::Add,
                        "SUB" => operator = TermOperator::Sub,
                        "MUL" => operator = TermOperator::Mul,
                        "SLASH" => operator = TermOperator::Slash,
                        "DOTS" => operator = TermOperator::Dots,
                        "LPAREN" => {
                            kind = TermType::Identifier;
                            return TermSemantics { operator, kind, range: node.range() };
                        },
                        _ => {}
                    }
                    let left_child = node.child(0).unwrap();
                    let right_child = node.child(2).unwrap();
                    if terms.contains_key(&left_child.id()) && terms.contains_key(&right_child.id()) {
                        let left_child_sem = terms.get(&left_child.id()).unwrap();
                        let right_child_sem = terms.get(&right_child.id()).unwrap();
    
                        if left_child_sem.kind == TermType::Constant && right_child_sem.kind == TermType::Constant {
                            kind = TermType::Constant
                        }
                        else if (left_child_sem.kind == TermType::Variable && right_child_sem.kind == TermType::Constant) || (left_child_sem.kind == TermType::Constant && right_child_sem.kind == TermType::Variable) {
                            kind = TermType::Variable
                        }
                        else {
                            kind = TermType::Unknown
                        }
                    }
                }
            }
            _ => {}
        }
        
        TermSemantics { operator, kind, range: node.range() }
    }
}

#[derive(Clone)]
pub enum TermOperator {
    None,
    Add,
    Sub,
    Mul,
    Slash,
    Dots
}

#[derive(Clone, PartialEq)]
pub enum TermType {
    Unknown,
    Identifier,
    Constant,
    Variable
}

/**
 * Go through the tree post order and populate an encoding semantics object
 */
pub fn analyze_tree(tree: &Tree) -> EncodingSemantics {
    let mut semantics = EncodingSemantics::new();
    let mut cursor = tree.walk();
    
    let mut reached_root = false;
    while !reached_root {

        if cursor.goto_first_child() {
            continue;
        }

        let node = cursor.node();

        if cursor.goto_next_sibling() {
            on_node(&node, &mut semantics);
            continue;
        }

        loop {

            on_node(&cursor.node(), &mut semantics);

            if !cursor.goto_parent() {
                reached_root = true;
                break;
            }

            let node = cursor.node();

            if cursor.goto_next_sibling() {
                on_node(&node, &mut semantics);
                break;
            }
        };
    }

    semantics
}

pub fn on_node(node: &Node, semantics: &mut EncodingSemantics) {
    if node.is_error() {
        // Save where there is an error
        semantics.errors.push(ErrorSemantic::new(node));
    } else if node.is_missing() {
        // Save where something is missing and what is missing
        semantics.missing.push(MissingSemantic::new(node.range(), node.kind()));
    }

    match node.kind() {
        "dec" | "NUMBER" | "term" | "VARIABLE" | "identifier" => {
            semantics.terms.insert(node.id(), TermSemantics::new(node, &semantics.terms));
        }
        "ADD" | "SUB" | "MUL" | "SLASH" | "DOTS" => {
            // If this expression is of the kind where an identifier + ... is used, give a warning to the user that this operation is undefined
            // if node.prev_sibling().map_or_else(|| false, |prev| prev.child_count() > 0 && prev.child(0).map_or_else(|| false, |prev_child| prev_child.kind() == "identifier")) {
            //    
            // }
        }
        _ => {}
    }
}
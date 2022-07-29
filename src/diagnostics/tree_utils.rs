use std::{collections::HashSet};

use dashmap::DashMap;
use log::info;
use tree_sitter::{Node, Query, QueryCursor, Range, Tree, TreeCursor};

use super::tree_error_analysis::{ErrorSemantic, MissingSemantic};

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
    node: &tree_sitter::Node<'a>,
    source: &'a [u8],
) -> std::vec::Vec<(tree_sitter::Range, &'a str, tree_sitter::Node<'a>)> {
    let mut query_cursor = QueryCursor::new();
    let query = Query::new(tree_sitter_clingo::language(), query_string).unwrap();

    let matches = query_cursor.matches(&query, *node, source);
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
 * What type a literal is
 */
#[derive(Clone, Debug)]
pub enum LiteralType {
    Normal,
    Conjunction,
    AggregateElement
}

/**
 * Special Literal semantics contain all the information needed around a conditional literal or aggregate
 */
#[derive(Clone, Debug)]
pub struct SpecialLiteralSemantics {
    pub id: usize,
    pub kind: LiteralType,
    pub local_dependency: Vec<(HashSet<String>, HashSet<String>)>,
    pub variable_locations: Vec<(Range, String)>
}

impl SpecialLiteralSemantics {
    pub fn new(node: &Node, semantics: &EncodingSemantics, source: &[u8]) -> SpecialLiteralSemantics {
        let mut local_dependency: Vec<(HashSet<String>, HashSet<String>)> = Vec::new();

        match node.kind() {
            "conjunction" =>  {
                if node.child_count() == 3 {
                    let l0 = node.child(0).unwrap();
                    let condition = node.child(2).unwrap();
                    
                    local_dependency.push((HashSet::new(), semantics.get_vars_for_node(&l0.id())));
                    local_dependency.extend(semantics.get_dependency_for_node(&condition.id()));
                }
            },
            "bodyaggrelem" => {
                if node.child_count() >= 2 {
                    let terms = node.child(0).unwrap();
                    let condition = node.child(1).unwrap();
                    
                    local_dependency.push((HashSet::new(), semantics.get_vars_for_node(&terms.id())));
                    local_dependency.extend(semantics.get_dependency_for_node(&condition.id()));

                    info!("Body Aggregate Element terms: {:?}, condition: {:?}", semantics.get_vars_for_node(&terms.id()), semantics.get_dependency_for_node(&condition.id()));
                    info!("Body Aggregate Element litvec: {:?}", semantics.get_dependency_for_node(&condition.child(1).unwrap().id()));
                }
            }
            _ => {}
        }
        
        //Find local variable locations
        let local_vars = do_simple_query("(VARIABLE) @name", node, source);
        let mut variable_locations = Vec::new();
        for (range, string, _) in local_vars {
            variable_locations.push((range, string.to_string()));
        }

        SpecialLiteralSemantics {
            id: node.id(),
            kind: match node.kind() {
                "conjunction" => LiteralType::Conjunction,
                "bodyaggrelem" => LiteralType::AggregateElement,
                _ => LiteralType::Normal
            },
            local_dependency,
            variable_locations 
        }
    }
}

/**
 * Encoding semantics are all the information needed about the program that then can be used by the other parts of the LSP
 */
#[derive(Clone, Debug)]
pub struct EncodingSemantics{
    pub errors: Vec<ErrorSemantic>,
    pub missing: Vec<MissingSemantic>,
    pub terms: DashMap<usize, TermSemantics>,
    pub vars: DashMap<usize, HashSet<String>>,
    pub global_vars: DashMap<usize, HashSet<String>>,
    pub provide: DashMap<usize, HashSet<String>>,
    pub depend: DashMap<usize, HashSet<String>>,
    pub dependency: DashMap<usize, Vec<(HashSet<String>, HashSet<String>)>>,
    pub special_literals: DashMap<usize, Vec<SpecialLiteralSemantics>>,
}

impl EncodingSemantics {
    pub fn new() -> EncodingSemantics {
        EncodingSemantics {
            errors: Vec::new(),
            missing: Vec::new(),
            terms: DashMap::new(),
            vars: DashMap::new(),
            global_vars: DashMap::new(),
            provide: DashMap::new(),
            depend: DashMap::new(),
            dependency: DashMap::new(),
            special_literals: DashMap::new()
        }
    }

    /**
     * Returns all variables occuring in that part of the encoding
     */
    pub fn get_vars_for_node(&self, node: &usize) -> HashSet<String> {
        if self.vars.contains_key(node) {
            return self.vars.get(node).unwrap().value().clone();
        }
        HashSet::new()
    }

    /**
     * Returns all global variables occuring in that part of the encoding
     */
    pub fn get_global_vars_for_node(&self, node: &usize) -> HashSet<String> {
        if self.global_vars.contains_key(node) {
            return self.global_vars.get(node).unwrap().value().clone();
        }
        HashSet::new()
    }

    /**
     * Returns every variable that part of the encoding provides
     */
    pub fn get_provide_for_node(&self, node: &usize) -> HashSet<String> {
        if self.provide.contains_key(node) {
            return self.provide.get(node).unwrap().value().clone();
        }
        HashSet::new()
    }

    /**
     * Returns every variable that is pure in part of the encoding 
     */
    pub fn get_depend_for_node(&self, node: &usize) -> HashSet<String> {
        if self.depend.contains_key(node) {
            return self.depend.get(node).unwrap().value().clone();
        }
        HashSet::new()
    }

    /**
     * Returns every dependency that part of the encoding has
     */
    pub fn get_dependency_for_node(&self, node: &usize) -> Vec<(HashSet<String>,HashSet<String>)> {
        if self.dependency.contains_key(node) {
            return self.dependency.get(node).unwrap().value().clone();
        }
        Vec::new()
    }

    /**
     * Returns every special literal that part of the encoding has
     */
    pub fn get_special_literals_for_node(&self, node: &usize) -> Vec<SpecialLiteralSemantics> {
        if self.special_literals.contains_key(node) {
            return self.special_literals.get(node).unwrap().value().clone();
        }
        Vec::new()
    }

    /**
     * Returns true if we could evaluate a value for this term (only possible if term is a constant) 
     */
    pub fn is_evaluable(&self, node: &usize) -> bool {
        if self.terms.contains_key(node) {
            let sem = self.terms.get(node).unwrap();
            if sem.kind == TermType::Constant {
                return true;
            }
        }
        false
    }

    /**
     * Returns the value for this constant term
     * Empty if it does not exist or if it is not a constant
     */
    pub fn get_evaluation_for_term(&self, node: &usize) -> HashSet<i64> {
        if self.terms.contains_key(node) && self.is_evaluable(node){
            let sem = self.terms.get(node).unwrap();

            return sem.value.clone();
        }
        HashSet::new()
    }


}

#[derive(Clone, Debug)]
pub struct TermSemantics {
    pub operator: TermOperator,
    pub kind: TermType,
    pub value: HashSet<i64>,
    pub range: Range,
}

/**
 * contains all information attached to a term
 */
impl TermSemantics {
    pub fn new(node: &Node, terms: &DashMap<usize, TermSemantics>, source: &[u8]) -> TermSemantics {
        let mut kind = TermType::Unknown;
        let mut operator = TermOperator::None;
        let mut value = HashSet::new();

        match node.kind() {
            "dec" | "NUMBER" => {
                kind = TermType::Constant;
                value.insert(node.utf8_text(source).unwrap().parse::<i64>().unwrap_or_default());
            },
            "VARIABLE" => kind = TermType::Variable,
            "identifier" => kind = TermType::Identifier,
            "term" => {
                if node.child_count() == 1 && terms.contains_key(&node.child(0).unwrap().id()) {
                    let child = terms
                    .get(&node.child(0).unwrap().id())
                    .unwrap();
                    
                    kind = child.kind.clone();
                    value = child.value.clone();
                } else if node.child_count() > 2 {
                    match node.child(1).unwrap().kind() {
                        "ADD" => operator = TermOperator::Add,
                        "SUB" => operator = TermOperator::Sub,
                        "MUL" => operator = TermOperator::Mul,
                        "SLASH" => operator = TermOperator::Div,
                        "DOTS" => operator = TermOperator::Dots,
                        "LPAREN" => {
                            kind = TermType::Identifier;
                            return TermSemantics {
                                operator,
                                kind,
                                value,
                                range: node.range(),
                            };
                        }
                        _ => {}
                    }
                    let left_child = node.child(0).unwrap();
                    let right_child = node.child(2).unwrap();
                    if terms.contains_key(&left_child.id()) && terms.contains_key(&right_child.id())
                    {
                        let left_child_sem = terms.get(&left_child.id()).unwrap();
                        let right_child_sem = terms.get(&right_child.id()).unwrap();

                        if left_child_sem.kind == TermType::Constant
                            && right_child_sem.kind == TermType::Constant
                        {
                            kind = TermType::Constant;
                            value = TermSemantics::evaluate(left_child_sem.value(), right_child_sem.value(), &operator) 
                        } else if (left_child_sem.kind == TermType::Variable
                            && right_child_sem.kind == TermType::Constant)
                            || (left_child_sem.kind == TermType::Constant
                                && right_child_sem.kind == TermType::Variable)
                        {
                            kind = TermType::Variable
                        } else {
                            kind = TermType::Unknown
                        }
                    }
                }
            }
            _ => {}
        }

        TermSemantics {
            operator,
            kind,
            value,
            range: node.range(),
        }
    }

    pub fn evaluate(a : &TermSemantics, b : &TermSemantics, op : &TermOperator) -> HashSet<i64> {
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
                            result_set.insert(s_i - s_j);
                        }
                    }
                }
            },
            _ => {}
        }

        result_set
    }
}


#[derive(Clone, Debug)]
pub enum TermOperator {
    None,
    Add,
    Sub,
    Mul,
    Div,
    Dots,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TermType {
    Unknown,
    Identifier,
    Constant,
    Variable,
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

/**
 * Go through the tree post order and populate an encoding semantics object
 */
pub fn analyze_tree<'a>(tree: &Tree, source: &str) -> EncodingSemantics {
    let mut semantics = EncodingSemantics::new();
    let mut cursor = tree.walk();

    let mut reached_root = false;
    while !reached_root {
        if cursor.goto_first_child() {
            continue;
        }

        let node = cursor.node();

        if cursor.goto_next_sibling() {
            on_node(&node, &mut semantics, source);
            continue;
        }

        loop {
            on_node(&cursor.node(), &mut semantics, source);

            if !cursor.goto_parent() {
                reached_root = true;
                break;
            }

            let node = cursor.node();

            if cursor.goto_next_sibling() {
                on_node(&node, &mut semantics, source);
                break;
            }
        }
    }

    semantics
}

pub fn on_node(node: &Node, semantics: &mut EncodingSemantics, source: &str) {
    if node.is_error() {
        // Save where there is an error
        semantics.errors.push(ErrorSemantic::new(node));
    } else if node.is_missing() {
        // Save where something is missing and what is missing
        semantics
            .missing
            .push(MissingSemantic::new(node.range(), node.kind()));
    }

    //Find all variables
    match node.kind() {
        "VARIABLE" => {
            let mut set = HashSet::new();
            set.insert(node.utf8_text(source.as_bytes()).unwrap_or("").to_string());
            semantics.vars.insert(node.id(), set);
        }
        _ => {
            let mut vars_in_children = HashSet::new();
            for child in node.children(&mut node.walk()) {
                vars_in_children.extend(semantics.get_vars_for_node(&child.id()).iter().cloned());
            }
            semantics.vars.insert(node.id(), vars_in_children.clone());
        }
    }

    //Perform provide function
    match node.kind() {
        "NUMBER" | "identifier" => {
            //Return an empty set
            semantics.provide.insert(node.id(), HashSet::new());
        }
        "VARIABLE" => {
            //Return a set containing this variable
            let mut set = HashSet::new();
            set.insert(node.utf8_text(source.as_bytes()).unwrap_or("").to_string());
            semantics.provide.insert(node.id(), set);
        }
        "term" => {
            // If we only have 1 child we pass on the provide for the child
            if node.child_count() == 1 {
                let child = node.child(0).unwrap();
                semantics.provide.insert(node.id(), semantics.get_provide_for_node(&child.id()));
            } else if node.child_count() >= 3 {
                let left_child = node.child(0).unwrap();
                let operator = node.child(1).unwrap();
                let right_child = node.child(2).unwrap();
                match operator.kind() {
                    "LPAREN" => {
                        // We have an term of form f(t) pass on the provide value of the child
                        semantics.provide.insert(node.id(), semantics.get_provide_for_node(&right_child.id()));
                    }
                    "ADD" | "SUB" => {
                        // We have an term of form a \star b
                        if semantics.is_evaluable(&left_child.id()) {
                            // If the left term is a constant then we pass on the provide value of the right child
                            semantics.provide.insert(node.id(), semantics.get_provide_for_node(&right_child.id()));
                        } else if semantics.is_evaluable(&right_child.id()) {
                            // If the right term is a constant then we pass on the provide value of the left child
                            semantics.provide.insert(node.id(), semantics.get_provide_for_node(&left_child.id()));
                        }
                    }
                    "MUL" => {
                        // We have an term of form a * b
                        info!("Multiplication {:?} -- provide left: {:?}, right: {:?} -- value left: {:?} ({:?}), right: {:?} ({:?})", node.utf8_text(source.as_bytes()), &semantics.get_provide_for_node(&left_child.id()), &semantics.get_provide_for_node(&right_child.id()), &semantics.get_evaluation_for_term(&left_child.id()), semantics.is_evaluable(&left_child.id()), &semantics.get_evaluation_for_term(&right_child.id()), semantics.is_evaluable(&right_child.id()));

                        if semantics.is_evaluable(&left_child.id()) && !semantics.get_evaluation_for_term(&left_child.id()).contains(&0) {
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
        "termvec" => {
            let mut provide = HashSet::new();

            // Pass on all the provides of every child
            for child in (0 .. node.child_count()) {
                let child = node.child(child).unwrap();

                if child.kind() != "COMMA" {
                    provide.extend(semantics.get_provide_for_node(&child.id()).iter().cloned());
                }
                
            }

            semantics.provide.insert(node.id(), provide);
        }
        "argvec" => {
            // If we only have 1 child we pass on the provide for the child
            if node.child_count() == 1 {
                let child = node.child(0).unwrap();
                semantics.provide.insert(node.id(), semantics.get_provide_for_node(&child.id()));
            } else if node.child_count() == 3{
                // If we have a argvec with a semicolon we have a pool
                let semicolon = node.child(1).unwrap();
                if semicolon.kind() == "SEM" {
                    let left_child = node.child(0).unwrap();
                    let right_child = node.child(2).unwrap();

                    // For a Pool we take the difference between the two provides of the children
                    let left_provide = semantics.get_provide_for_node(&left_child.id());
                    let right_provide = semantics.get_provide_for_node(&right_child.id());

                    let provide = left_provide.intersection(&right_provide).cloned().collect();

                    semantics.provide.insert(node.id(), provide);
                }
            }
        }
        _ => {}
    }

    //Collect all depend variables for terms
    match node.kind() {
        "termvec" => {
            let mut depend = HashSet::new();

            // Pass on all the depend of every child
            for child in (0 .. node.child_count()) {
                let child = node.child(child).unwrap();

                if child.kind() != "COMMA" {
                    depend.extend(semantics.get_depend_for_node(&child.id()).iter().cloned());
                }
                
            }

            //Remove any variables who are provided
            depend = depend.difference(&semantics.get_provide_for_node(&node.id())).cloned().collect();

            semantics.depend.insert(node.id(), depend);
        }
        "argvec" => {
            // If we only have 1 child we pass on the depend for the child
            if node.child_count() == 1 {
                let child = node.child(0).unwrap();
                semantics.depend.insert(node.id(), semantics.get_depend_for_node(&child.id()).clone());

                info!("Argvec {:?} -- depend: {:?}", node.utf8_text(source.as_bytes()), semantics.get_depend_for_node(&child.id()).clone());
            } else if node.child_count() == 3{
                // If we have a argvec with a semicolon we have a pool
                let semicolon = node.child(1).unwrap();
                if semicolon.kind() == "SEM" {
                    let left_child = node.child(0).unwrap();
                    let right_child = node.child(2).unwrap();

                    // For a Pool we take the difference between the two depends of the children
                    let left_depend = semantics.get_depend_for_node(&left_child.id());
                    let right_depend = semantics.get_depend_for_node(&right_child.id());

                    let mut depend : HashSet<String> = left_depend.union(&right_depend).cloned().collect();

                    //Remove any variables who are provided
                    depend = depend.difference(&semantics.get_provide_for_node(&node.id())).cloned().collect();

                    semantics.depend.insert(node.id(), depend);
                }
            }
        }
        "term" => {
            // If we only have 1 child we pass on the depend for the child
            if node.child_count() == 1 {
                let child = node.child(0).unwrap();
                semantics.depend.insert(node.id(), semantics.get_depend_for_node(&child.id()));
            } else if node.child_count() >= 3 {
                let left_child = node.child(0).unwrap();
                let operator = node.child(1).unwrap();
                let right_child = node.child(2).unwrap();
                match operator.kind() {
                    "LPAREN" => {
                        // We have an term of form f(t) pass on the depend value of the child
                        semantics.depend.insert(node.id(), semantics.get_depend_for_node(&right_child.id()));
                    }
                    _ => {
                        // We have a term of form a * b, this means we have to return vars(e) \ pt(e)
                        let mut depend = semantics.get_vars_for_node(&node.id());
                        depend = depend.difference(&semantics.get_provide_for_node(&node.id())).cloned().collect();

                        info!("Term {:?} -- depend: {:?}", node.utf8_text(source.as_bytes()), &depend);

                        semantics.depend.insert(node.id(), depend);
                    }
                }
            }
        }
        _ => {
            // Pass on all the dependencies
            let mut depend = HashSet::new();

            // Pass on all the depend of every child
            for child in 0 .. node.child_count() {
                let child = node.child(child).unwrap();

                if child.kind() != "COMMA" {
                    depend.extend(semantics.get_depend_for_node(&child.id()).iter().cloned());
                }
                
            }

            semantics.depend.insert(node.id(), depend);
        }
    }

    //Perform dependency function
    match node.kind() {
        "statement" => {
            if node.child_count() >= 1 {
                //If we have one child do the dependency function for a rule (\emptyset, vars(head))
                let head = node.child(0).unwrap();
                let mut dependencies = vec![(HashSet::new(), semantics.get_vars_for_node(&head.id()))];

                // If there is a body then we add the dependencies from each literal
                if node.child_count() >= 3 {
                    let body = node.child(2).unwrap();
                    if body.kind() == "bodydot" {
                        dependencies.extend(semantics.dependency.get(&body.id()).unwrap().iter().cloned())
                    }
                }
                
                semantics.dependency.insert(node.id(), dependencies);
            }
            else {
                semantics.dependency.insert(node.id(), Vec::new());
            }
        }
        "atom" => {
            if node.child_count() >=3 {
                //If we have three children do the dependency function for an atom (pt(t), \emptyset), (\emptyset, dt(t))
                let argument = node.child(2).unwrap();
                let provide = semantics.get_provide_for_node(&argument.id());
                let depend = semantics.get_depend_for_node(&argument.id());
                let dependencies = vec![(provide, HashSet::new()), (HashSet::new(), depend)];
                semantics.dependency.insert(node.id(), dependencies);
            } else {
                semantics.dependency.insert(node.id(), Vec::new());
            }
        }
        "literal" => {
            // If we have a literal then pass on all variables as global
            semantics.global_vars.insert(node.id(), semantics.get_vars_for_node(&node.id()));

            if node.child_count() == 1 {
                //If we have one child pass on the dependency from the atom
                let atom = node.child(0).unwrap();
                semantics.dependency.insert(node.id(), semantics.get_dependency_for_node(&atom.id()));
            }
            else if node.child_count() == 2 {
                //We have a not in front of the atom
                let not = node.child(0).unwrap();
                let atom = node.child(1).unwrap();
                let mut dep = Vec::new();

                if not.kind() == "NOT" {
                    dep.push((HashSet::new(), semantics.get_vars_for_node(&atom.id())));

                    semantics.dependency.insert(node.id(), dep);
                }
            }
            else if node.child_count() >= 3{
                let mut left_child = node.child(0).unwrap();
                let mut operator = node.child(1).unwrap();
                let mut right_child = node.child(2).unwrap();
                if node.child_count() == 4 && left_child.kind() == "NOT" {
                    left_child = node.child(1).unwrap();
                    operator = node.child(2).unwrap();
                    right_child = node.child(3).unwrap();
                }

                match operator.kind() {
                    "cmp" => {
                        
                        // We have an comparison literal, set all variables to a dependency
                        let mut dep = Vec::new();
                        let vars: HashSet<String> = semantics.get_vars_for_node(&left_child.id()).union(&semantics.get_vars_for_node(&right_child.id())).cloned().collect();
                        let mut comparison = operator.child(0).unwrap_or(operator).kind();
                        if(node.child_count() == 4) {
                            comparison = negate_comparison_operator(comparison);
                        }

                        // If the comparison is not an assignment
                        if comparison != "EQ" {
                            dep.push((HashSet::new(), vars));
                        } else {
                            // (pt(t1), vars(t2))
                            dep.push((semantics.get_provide_for_node(&left_child.id()), semantics.get_vars_for_node(&right_child.id())));
                            // (pt(t2), vars(t1))
                            dep.push((semantics.get_provide_for_node(&right_child.id()), semantics.get_vars_for_node(&left_child.id())));
                            // (∅, dt(t1) ∪ dt(t2))
                            dep.push((HashSet::new(), semantics.get_depend_for_node(&left_child.id()).union(&semantics.get_depend_for_node(&right_child.id())).cloned().collect()));
                        }

                        semantics.dependency.insert(node.id(), dep);
                    }
                    _ => {
                        semantics.dependency.insert(node.id(), Vec::new());
                    }
                }
            }
        }
        "conjunction" => {
            // We have a conditional literal
            let vars = semantics.get_vars_for_node(&node.id());

            if node.child_count() >= 3 {
                let local_literal = node.child(0).unwrap();
                let condition = node.child(2).unwrap();

                let dep = vec![(HashSet::new(), vars)];
                semantics.dependency.insert(node.id(), dep);

                // for the global context we return all variables that are not in the local context
                semantics.global_vars.insert(node.id(), semantics.get_vars_for_node(&local_literal.id()).difference(&semantics.get_vars_for_node(&condition.id())).cloned().collect());
            }
        }
        "bodycomma" => {
            // If we only have 2 children we pass on the dependency for the first children (ignore the last child which is a comma)
            if node.child_count() >= 2 {
                let mut dependencies : Vec<(HashSet<String>, HashSet<String>)> = Vec::new();

                for child_id in 0 .. node.child_count() {
                    let literal = node.child(child_id).unwrap();

                    match literal.kind() {
                        "literal" | "bodycomma"| "bodydot" | "conjunction" | "lubodyaggregate" => {
                            dependencies.append(&mut semantics.get_dependency_for_node(&literal.id()));
                        }
                        _ => {}
                    }
                }

                semantics.dependency.insert(node.id(), dependencies);
            }
        }
        "bodydot" => {
            if node.child_count() >= 1 {
                //If we have at least 1 child pass on the dependencies from each of the literals
                let mut dependencies : Vec<(HashSet<String>, HashSet<String>)> = Vec::new();

                for child_id in 0 .. node.child_count() {
                    let literal = node.child(child_id).unwrap();

                    match literal.kind() {
                        "literal" | "bodycomma" | "bodydot" | "conjunction" | "lubodyaggregate" => {
                            dependencies.append(&mut semantics.get_dependency_for_node(&literal.id()));
                        }
                        _ => {}
                    }
                }

                semantics.dependency.insert(node.id(), dependencies);
            }   
            else {
                semantics.dependency.insert(node.id(), Vec::new());
            }
        }
        "litvec" | "optcondition" => {
            let mut dependencies : Vec<(HashSet<String>, HashSet<String>)> = Vec::new();

            for child_id in 0 .. node.child_count() {
                let literal = node.child(child_id).unwrap();

                match literal.kind() {
                    "COMMA" => {}
                    _ => {
                        dependencies.append(&mut semantics.get_dependency_for_node(&literal.id()));
                    }
                }
            }

            semantics.dependency.insert(node.id(), dependencies);
        }
        "lubodyaggregate" => {
            if node.child_count() >= 2 {
                let mut aggregate : Option<Node> = None;
                let mut lower_bounds : Option<&str> = None;
                let mut lower_bounds_term : Option<Node> = None;
                let mut upper_bounds : Option<&str> = None;
                let mut upper_bounds_term : Option<Node> = None;

                // When we have an aggregate with bounds, we need to check where the bounds are
                for child_id in 0 .. node.child_count() {
                    let child = node.child(child_id).unwrap();

                    match child.kind() {
                        "bodyaggregate" => {
                            aggregate = Some(child);
                        },
                        "upper" => {
                            if child.child_count() >= 2 {
                                let cmp = child.child(0).unwrap();
                                if cmp.child_count() >= 1 {
                                    upper_bounds = Some(cmp.child(0).unwrap().kind());
                                }
                                upper_bounds_term = Some(child.child(1).unwrap());
                            }
                        },
                        "term" => {
                            lower_bounds_term = Some(child);
                        },
                        "cmp" => {
                            if child.child_count() >= 1 {
                                lower_bounds = Some(child.child(0).unwrap().kind());
                            }
                        }
                        _ => {}
                    }
                }

                let mut dependencies: Vec<(HashSet<String>, HashSet<String>)> = Vec::new();
                let mut global_vars = HashSet::new();

                //Find out which local and aggregate variables we have
                let mut aggr_vars: HashSet<String> = HashSet::new();
                if aggregate.is_some() {
                    aggr_vars = semantics.get_vars_for_node(&aggregate.unwrap().id());
                }

                //Depending if certain bounds exist and their types we now set the dependencies
                if lower_bounds.is_some() && lower_bounds.unwrap() == "EQ" && lower_bounds_term.is_some() {
                    // If there is an assignment then we need a different global dependency
                    let lower_bounds_term_id = &lower_bounds_term.unwrap().id();
                    dependencies.push((semantics.get_provide_for_node(lower_bounds_term_id), aggr_vars.clone()));
                    dependencies.push((HashSet::new(), semantics.get_depend_for_node(lower_bounds_term_id)));

                    global_vars = semantics.get_vars_for_node(lower_bounds_term_id);
                } 
                if upper_bounds.is_some() && upper_bounds.unwrap() == "EQ" {
                    let upper_bounds_term_id = &upper_bounds_term.unwrap().id();
                    dependencies.push((semantics.get_provide_for_node(upper_bounds_term_id), aggr_vars));
                    dependencies.push((HashSet::new(), semantics.get_depend_for_node(upper_bounds_term_id)));

                    global_vars = global_vars.union(&semantics.get_vars_for_node(upper_bounds_term_id)).cloned().collect();
                }
                if !((lower_bounds.is_some() && lower_bounds.unwrap() == "EQ") || (upper_bounds.is_some() && upper_bounds.unwrap() == "EQ")) {
                    // If there isn't an assignment then all global variables will be the dependency for this aggregate
                    let vars = semantics.get_vars_for_node(&node.id());
                    dependencies.push((HashSet::new(), vars));
                }

                info!("Found aggregate with dependencies: {:?}", dependencies);
                semantics.dependency.insert(node.id(), dependencies);
                semantics.global_vars.insert(node.id(), global_vars);
            }
        }
        _ => {}
    }

    // Pass on global vars if they are not handled in dependency check
    match node.kind() {
        "literal" | "conjunction" | "lubodyaggregate" => {},
        _ => {
            let mut global_vars : HashSet<String> = HashSet::new();

            for child_id in 0 .. node.child_count() {
                let child = node.child(child_id).unwrap();

                global_vars.extend(semantics.get_global_vars_for_node(&child.id()))
            }

            semantics.global_vars.insert(node.id(), global_vars);
        }
    }

    //Keep track of any special literals
    match node.kind() {
        "conjunction" | "bodyaggrelem" => {
            semantics.special_literals.insert(node.id(), vec![SpecialLiteralSemantics::new(node, semantics, source.as_bytes())]);
        }
        _ => {
            let mut special_literals : Vec<SpecialLiteralSemantics> = Vec::new();

            for child_id in 0 .. node.child_count() {
                let child = node.child(child_id).unwrap();

                special_literals.extend(semantics.get_special_literals_for_node(&child.id()))
            }

            semantics.special_literals.insert(node.id(), special_literals);
        }
    }
    

    //Inspect Terms
    match node.kind() {
        "dec" | "NUMBER" | "term" | "VARIABLE" | "identifier" => {
            semantics
                .terms
                .insert(node.id(), TermSemantics::new(node, &semantics.terms, source.as_bytes()));

            let info_sem = TermSemantics::new(node, &semantics.terms, source.as_bytes());
            info!("Term inspection: {:?} -- kind: {:?} -- kind: {:?}, operator: {:?}, value: {:?}", node.utf8_text(source.as_bytes()), node.kind(), info_sem.kind, info_sem.operator, info_sem.value);
        }
        _ => {}
    }
}
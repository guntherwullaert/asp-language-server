use std::{collections::HashSet, vec};
use tree_sitter::Node;

use crate::document::DocumentData;

use super::{
    encoding_semantic::{EncodingSemantics, Semantics},
    special_literal_semantic::SpecialLiteralSemantics,
    term_semantic::{TermSemantic, TermType},
};

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
     * Which variables are globally defined in this part of the encoding
     */
    pub global_vars: HashSet<String>,

    /**
     * Which variables are provided by this part of the encoding
     */
    pub provide: HashSet<String>,

    /**
     * Which variables this part of the encoding depend on
     */
    pub depend: HashSet<String>,

    /**
     * A list of sets of variables provided and depended upon. This is later used for checking the safety of a statement
     */
    pub dependencies: Vec<(HashSet<String>, HashSet<String>)>,

    /**
     * For a term in a statement this struct contains the information needed to understand what this term is
     */
    pub term: TermSemantic,

    /**
     * A list of Special literals that need to have more checks, for example for safety
     */
    pub special_literals: Vec<SpecialLiteralSemantics>,
}

impl StatementSemantics {
    pub fn new() -> StatementSemantics {
        StatementSemantics {
            vars: HashSet::new(),
            global_vars: HashSet::new(),
            provide: HashSet::new(),
            depend: HashSet::new(),
            dependencies: Vec::new(),
            term: TermSemantic::new(),
            special_literals: Vec::new(),
        }
    }

    /**
     * Set vars set for this statement
     */
    pub fn with_vars(mut self, vars: HashSet<String>) -> StatementSemantics {
        self.vars = vars;
        self
    }

    /**
     * Set global vars set for this statement
     */
    pub fn with_global_vars(mut self, vars: HashSet<String>) -> StatementSemantics {
        self.global_vars = vars;
        self
    }

    /**
     * Set provide set for this statement
     */
    pub fn with_provide(mut self, provide: HashSet<String>) -> StatementSemantics {
        self.provide = provide;
        self
    }

    /**
     * Set depend set for this statement
     */
    pub fn with_depend(mut self, depend: HashSet<String>) -> StatementSemantics {
        self.depend = depend;
        self
    }

    /**
     * Set dependencies set for this statement
     */
    pub fn with_dependencies(
        mut self,
        dependencies: Vec<(HashSet<String>, HashSet<String>)>,
    ) -> StatementSemantics {
        self.dependencies = dependencies;
        self
    }

    /**
     * Set special literals set for this statement
     */
    pub fn with_special_literals(
        mut self,
        special_literals: Vec<SpecialLiteralSemantics>,
    ) -> StatementSemantics {
        self.special_literals = special_literals;
        self
    }

    /**
     * Set term for this statement
     */
    pub fn with_term(mut self, term: TermSemantic) -> StatementSemantics {
        self.term = term;
        self
    }

    /**
     * Update vars for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_vars_for_node(
        semantics: &EncodingSemantics,
        node_id: usize,
        new_value: HashSet<String>,
    ) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics
                .statement_semantics
                .get_mut(&node_id)
                .unwrap()
                .vars = new_value;
            return;
        }

        semantics
            .statement_semantics
            .insert(node_id, StatementSemantics::new().with_vars(new_value));
    }

    /**
     * Update global vars for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_global_vars_for_node(
        semantics: &EncodingSemantics,
        node_id: usize,
        new_value: HashSet<String>,
    ) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics
                .statement_semantics
                .get_mut(&node_id)
                .unwrap()
                .global_vars = new_value;
            return;
        }

        semantics.statement_semantics.insert(
            node_id,
            StatementSemantics::new().with_global_vars(new_value),
        );
    }

    /**
     * Update provide for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_provide_for_node(
        semantics: &EncodingSemantics,
        node_id: usize,
        new_value: HashSet<String>,
    ) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics
                .statement_semantics
                .get_mut(&node_id)
                .unwrap()
                .provide = new_value;
            return;
        }

        semantics
            .statement_semantics
            .insert(node_id, StatementSemantics::new().with_provide(new_value));
    }

    /**
     * Update depend for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_depend_for_node(
        semantics: &EncodingSemantics,
        node_id: usize,
        new_value: HashSet<String>,
    ) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics
                .statement_semantics
                .get_mut(&node_id)
                .unwrap()
                .depend = new_value;
            return;
        }

        semantics
            .statement_semantics
            .insert(node_id, StatementSemantics::new().with_depend(new_value));
    }

    /**
     * Update dependencies for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_dependencies_for_node(
        semantics: &EncodingSemantics,
        node_id: usize,
        new_value: Vec<(HashSet<String>, HashSet<String>)>,
    ) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics
                .statement_semantics
                .get_mut(&node_id)
                .unwrap()
                .dependencies = new_value;
            return;
        }

        semantics.statement_semantics.insert(
            node_id,
            StatementSemantics::new().with_dependencies(new_value),
        );
    }

    /**
     * Update special literals for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_special_literals_for_node(
        semantics: &EncodingSemantics,
        node_id: usize,
        new_value: Vec<SpecialLiteralSemantics>,
    ) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics
                .statement_semantics
                .get_mut(&node_id)
                .unwrap()
                .special_literals = new_value;
            return;
        }

        semantics.statement_semantics.insert(
            node_id,
            StatementSemantics::new().with_special_literals(new_value),
        );
    }

    /**
     * Update term for a node, if there is no statement semantics object for that node it creates one
     */
    pub fn update_term_for_node(
        semantics: &EncodingSemantics,
        node_id: usize,
        new_value: TermSemantic,
    ) {
        if semantics.statement_semantics.contains_key(&node_id) {
            semantics
                .statement_semantics
                .get_mut(&node_id)
                .unwrap()
                .term = new_value;
            return;
        }

        semantics
            .statement_semantics
            .insert(node_id, StatementSemantics::new().with_term(new_value));
    }

    /**
     * Check if a variable occurs in this node, if not we pass on the variables in our children.
     */
    fn check_for_variables(node: Node, document: &mut DocumentData) {
        match node.kind() {
            "VARIABLE" => {
                let mut set = HashSet::new();
                let var_name = document.get_source_for_range(node.range());
                set.insert(var_name.clone());
                Self::update_vars_for_node(&document.semantics, node.id(), set);
            }
            "source_file" => {} // Ignore any fields above statements
            _ => {
                let mut vars_in_children = HashSet::new();
                for child in node.children(&mut node.walk()) {
                    vars_in_children.extend(
                        document
                            .semantics
                            .get_statement_semantics_for_node(child.id())
                            .vars,
                    );
                }
                Self::update_vars_for_node(&document.semantics, node.id(), vars_in_children);
            }
        }
    }

    /**
     * Returns true if we could evaluate a value for this term (only possible if term is a constant)
     */
    pub fn is_evaluable(node: usize, document: &mut DocumentData) -> bool {
        let semantic = document.semantics.get_statement_semantics_for_node(node);

        if semantic.term.kind == TermType::Constant {
            return true;
        }
        false
    }

    /**
     * Combine every global vars in the children of node and set this as the global vars for this node
     */
    fn pass_on_global_vars_from_children(node: Node, document: &mut DocumentData) {
        let mut global_vars_in_children = HashSet::new();
        for child in node.children(&mut node.walk()) {
            global_vars_in_children.extend(
                document
                    .semantics
                    .get_statement_semantics_for_node(child.id())
                    .global_vars,
            );
        }
        Self::update_global_vars_for_node(&document.semantics, node.id(), global_vars_in_children);
    }

    /**
     * Combine every provide in the children of node and set this as the provide for this node
     */
    fn pass_on_provide_from_children(node: Node, document: &mut DocumentData) {
        let mut provide_in_children = HashSet::new();
        for child in node.children(&mut node.walk()) {
            provide_in_children.extend(
                document
                    .semantics
                    .get_statement_semantics_for_node(child.id())
                    .provide,
            );
        }
        Self::update_provide_for_node(&document.semantics, node.id(), provide_in_children);
    }

    /**
     * Combine every depend in the children of node and set this as the depend for this node.
     * If remove_provide is set, the provide is removed from the depend set which was found
     */
    fn pass_on_depend_from_children(node: Node, document: &mut DocumentData, remove_provide: bool) {
        let mut depend_in_children = HashSet::new();
        for child in node.children(&mut node.walk()) {
            depend_in_children.extend(
                document
                    .semantics
                    .get_statement_semantics_for_node(child.id())
                    .depend,
            );
        }

        if remove_provide {
            depend_in_children = depend_in_children
                .difference(
                    &document
                        .semantics
                        .get_statement_semantics_for_node(node.id())
                        .provide,
                )
                .cloned()
                .collect();
        }

        Self::update_depend_for_node(&document.semantics, node.id(), depend_in_children);
    }

    /**
     * Combine every dependencies in the children of node and set this as the dependencies for this node.
     */
    fn pass_on_dependencies_from_children(
        node: Node,
        document: &mut DocumentData,
        check_for_node_kind: &HashSet<String>,
    ) {
        let mut dependencies_in_children = Vec::new();
        for child in node.children(&mut node.walk()) {
            if check_for_node_kind.len() == 0 || check_for_node_kind.contains(child.kind()) {
                dependencies_in_children.extend(
                    document
                        .semantics
                        .get_statement_semantics_for_node(child.id())
                        .dependencies,
                );
            }
        }

        Self::update_dependencies_for_node(
            &document.semantics,
            node.id(),
            dependencies_in_children,
        );
    }

    /**
     * Combine every special literal in the children of node and set this as the special literals for this node.
     */
    fn pass_on_special_literals_from_children(node: Node, document: &mut DocumentData) {
        let mut special_literals_in_children = Vec::new();
        for child in node.children(&mut node.walk()) {
            special_literals_in_children.extend(
                document
                    .semantics
                    .get_statement_semantics_for_node(child.id())
                    .special_literals,
            );
        }

        Self::update_special_literals_for_node(
            &document.semantics,
            node.id(),
            special_literals_in_children,
        );
    }

    /**
     * Pass on a provide of a specific node and set this as the provide for the to_be_updated_node
     */
    fn pass_on_provide_from_specific_node(
        document: &mut DocumentData,
        to_be_updated: Node,
        from_node: Node,
    ) {
        Self::update_provide_for_node(
            &document.semantics,
            to_be_updated.id(),
            document
                .semantics
                .get_statement_semantics_for_node(from_node.id())
                .provide,
        );
    }

    /**
     * Check what variables a part of a statement provides
     */
    fn check_provide(node: Node, document: &mut DocumentData) {
        match node.kind() {
            "NUMBER" | "identifier" => {} //IGNORE if required an emptyset will be returned by default
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
                                Self::pass_on_provide_from_specific_node(
                                    document,
                                    node,
                                    right_child,
                                );
                            } else if Self::is_evaluable(right_child.id(), document) {
                                // If the right term is a constant then we pass on the provide value of the left child
                                Self::pass_on_provide_from_specific_node(
                                    document, node, left_child,
                                );
                            }
                        }
                        "MUL" => {
                            // We have an term of form a * b
                            document
                                .semantics
                                .get_statement_semantics_for_node(left_child.id())
                                .term
                                .value
                                .contains(&0);
                            if Self::is_evaluable(left_child.id(), document)
                                && !document
                                    .semantics
                                    .get_statement_semantics_for_node(left_child.id())
                                    .term
                                    .value
                                    .contains(&0)
                            {
                                // If the left term is a constant then we pass on the provide value of the right child
                                Self::pass_on_provide_from_specific_node(
                                    document,
                                    node,
                                    right_child,
                                );
                            } else if Self::is_evaluable(right_child.id(), document)
                                && !document
                                    .semantics
                                    .get_statement_semantics_for_node(right_child.id())
                                    .term
                                    .value
                                    .contains(&0)
                            {
                                // If the right term is a constant then we pass on the provide value of the left child
                                Self::pass_on_provide_from_specific_node(
                                    document, node, left_child,
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
            "termvec" => {
                Self::pass_on_provide_from_children(node, document);
            }
            "argvec" => {
                // If we only have 1 child we pass on the provide for the child
                if node.child_count() == 1 {
                    Self::pass_on_provide_from_children(node, document);
                } else if node.child_count() == 3 {
                    // If we have a argvec with a semicolon we have a pool
                    let semicolon = node.child(1).unwrap();
                    if semicolon.kind() == "SEM" {
                        let left_child = node.child(0).unwrap();
                        let right_child = node.child(2).unwrap();

                        // For a Pool we take the difference between the two provides of the children
                        let left_provide = document
                            .semantics
                            .get_statement_semantics_for_node(left_child.id())
                            .provide;
                        let right_provide = document
                            .semantics
                            .get_statement_semantics_for_node(right_child.id())
                            .provide;

                        let provide = left_provide.intersection(&right_provide).cloned().collect();

                        Self::update_provide_for_node(&document.semantics, node.id(), provide);
                    }
                }
            }
            _ => {}
        }
    }

    /**
     * Check what variables a part of a statement depends on
     */
    fn check_depend(node: Node, document: &mut DocumentData) {
        match node.kind() {
            "termvec" => {
                Self::pass_on_depend_from_children(node, document, true);
            }
            "argvec" => {
                // If we only have 1 child we pass on the depend for the child
                if node.child_count() == 1 {
                    Self::pass_on_depend_from_children(node, document, false);
                } else if node.child_count() == 3 {
                    // If we have a argvec with a semicolon we have a pool
                    let semicolon = node.child(1).unwrap();
                    if semicolon.kind() == "SEM" {
                        let left_child = node.child(0).unwrap();
                        let right_child = node.child(2).unwrap();

                        // For a Pool we take the union between the two depends of the children
                        let left_depend = document
                            .semantics
                            .get_statement_semantics_for_node(left_child.id())
                            .depend;
                        let right_depend = document
                            .semantics
                            .get_statement_semantics_for_node(right_child.id())
                            .depend;

                        let mut depend: HashSet<String> =
                            left_depend.union(&right_depend).cloned().collect();

                        //Remove any variables who are provided
                        depend = depend
                            .difference(
                                &document
                                    .semantics
                                    .get_statement_semantics_for_node(node.id())
                                    .provide,
                            )
                            .cloned()
                            .collect();

                        Self::update_depend_for_node(&document.semantics, node.id(), depend);
                    }
                }
            }
            "term" => {
                // If we only have 1 child we pass on the depend for the child
                if node.child_count() == 1 {
                    Self::pass_on_depend_from_children(node, document, false);
                } else if node.child_count() >= 3 {
                    let _left_child = node.child(0).unwrap();
                    let operator = node.child(1).unwrap();
                    let _right_child = node.child(2).unwrap();
                    match operator.kind() {
                        "LPAREN" => {
                            // We have an term of form f(t) pass on the depend value of the child
                            Self::pass_on_depend_from_children(node, document, false);
                        }
                        _ => {
                            // We have a term of form a * b, this means we have to return vars(e) \ pt(e)
                            let mut depend = document
                                .semantics
                                .get_statement_semantics_for_node(node.id())
                                .vars;
                            depend = depend
                                .difference(
                                    &document
                                        .semantics
                                        .get_statement_semantics_for_node(node.id())
                                        .provide,
                                )
                                .cloned()
                                .collect();

                            Self::update_depend_for_node(&document.semantics, node.id(), depend);
                        }
                    }
                }
            }
            "source_file" => {} // Ignore any fields above statements
            _ => {
                Self::pass_on_depend_from_children(node, document, false);
            }
        }
    }

    /**
     * Check what each part of the statement depends and provides as a dependency tuple
     */
    fn check_dependencies(node: Node, document: &mut DocumentData) {
        match node.kind() {
            "statement" => {
                if node.child_count() >= 1 {
                    //If we have one child do the dependency function for a rule (\emptyset, vars(head))
                    let head = node.child(0).unwrap();
                    let mut dependencies = vec![(
                        HashSet::new(),
                        document
                            .semantics
                            .get_statement_semantics_for_node(head.id())
                            .vars,
                    )];

                    if (head.kind() == "SHOW" || head.kind() == "EXTERNAL")
                        && node.child_count() >= 2
                    {
                        // We have a show / external statement
                        let term = node.child(1).unwrap();
                        let term_semantics = document
                            .semantics
                            .get_statement_semantics_for_node(term.id());

                        // Set variables in the term to dependency
                        dependencies.push((HashSet::new(), term_semantics.vars.clone()));

                        // Also register variables in this term as that is not handled due to it not being a literal
                        Self::update_global_vars_for_node(
                            &document.semantics,
                            term.id(),
                            term_semantics.vars.clone(),
                        );

                        if node.child_count() >= 4 {
                            // We have a colon and bodydot after the show statement
                            let body = node.child(3).unwrap();
                            if node.child(2).unwrap().kind() == "COLON" && body.kind() == "bodydot"
                            {
                                dependencies.extend(
                                    document
                                        .semantics
                                        .get_statement_semantics_for_node(body.id())
                                        .dependencies,
                                )
                            }
                        }
                    } else if (head.kind() == "IF" || head.kind() == "WIF")
                        && node.child_count() >= 2
                    {
                        let body = node.child(1).unwrap();
                        if body.kind() == "bodydot" {
                            dependencies.extend(
                                document
                                    .semantics
                                    .get_statement_semantics_for_node(body.id())
                                    .dependencies,
                            )
                        }

                        //If we have a weak constraint we need to handle the weight and tuple
                        if head.kind() == "WIF" {
                            let weight = node.child(3).unwrap();
                            let weight_semantics = document
                                .semantics
                                .get_statement_semantics_for_node(weight.id());

                            dependencies.push((HashSet::new(), weight_semantics.vars.clone()));
                            Self::update_global_vars_for_node(
                                &document.semantics,
                                weight.id(),
                                weight_semantics.vars.clone(),
                            );

                            if node.child_count() >= 5 {
                                let tuple = node.child(4).unwrap();
                                let tuple_semantics = document
                                    .semantics
                                    .get_statement_semantics_for_node(tuple.id());
                                dependencies.push((HashSet::new(), tuple_semantics.vars.clone()));
                                Self::update_global_vars_for_node(
                                    &document.semantics,
                                    tuple.id(),
                                    tuple_semantics.vars.clone(),
                                );
                            }
                        }
                    } else if node.child_count() >= 3 {
                        let body = node.child(2).unwrap();
                        if body.kind() == "bodydot"
                            || body.kind() == "maxelemlist"
                            || body.kind() == "minelemlist"
                        {
                            dependencies.extend(
                                document
                                    .semantics
                                    .get_statement_semantics_for_node(body.id())
                                    .dependencies,
                            );
                        }
                    }

                    Self::update_dependencies_for_node(
                        &document.semantics,
                        node.id(),
                        dependencies,
                    );
                }
            }
            "atom" => {
                if node.child_count() >= 3 {
                    //If we have three children do the dependency function for an atom (pt(t), \emptyset), (\emptyset, dt(t))
                    let argument = node.child(2).unwrap();
                    let argument_semantics = document
                        .semantics
                        .get_statement_semantics_for_node(argument.id());
                    let dependencies = vec![
                        (argument_semantics.provide, HashSet::new()),
                        (HashSet::new(), argument_semantics.depend),
                    ];

                    Self::update_dependencies_for_node(
                        &document.semantics,
                        node.id(),
                        dependencies,
                    );
                }
            }
            "literal" => {
                if node.child_count() == 1 {
                    //If we have one child pass on the dependency from the atom
                    Self::pass_on_dependencies_from_children(node, document, &HashSet::new());
                } else if node.child_count() > 1 {
                    let mut atom_id = 0;

                    // Find out where the atom starts without NOT
                    for child_id in 0..node.child_count() {
                        if node.child(child_id).unwrap().kind() != "NOT" {
                            atom_id = child_id;
                            break;
                        }
                    }

                    let atom = node.child(atom_id).unwrap();

                    //If we have a comparison
                    if node.child_count() >= 3 && atom_id <= node.child_count() - 3 {
                        //We need to grab the child where the atom starts
                        let operator = node.child(atom_id + 1).unwrap();
                        let right_atom = node.child(atom_id + 2).unwrap();

                        match operator.kind() {
                            "cmp" => {
                                // We have an comparison literal, set all variables to a dependency
                                let mut dep = Vec::new();
                                let left_semantics = document
                                    .semantics
                                    .get_statement_semantics_for_node(atom.id());
                                let right_semantics = document
                                    .semantics
                                    .get_statement_semantics_for_node(right_atom.id());

                                let vars: HashSet<String> = left_semantics
                                    .vars
                                    .union(&right_semantics.vars)
                                    .cloned()
                                    .collect();
                                let mut comparison = operator.child(0).unwrap_or(operator).kind();
                                if node.child_count() >= 4 {
                                    comparison =
                                        TermSemantic::negate_comparison_operator(comparison);
                                }

                                // If the comparison is not an assignment
                                if comparison != "EQ" {
                                    dep.push((HashSet::new(), vars));
                                } else {
                                    // (pt(t1), vars(t2))
                                    dep.push((left_semantics.provide, right_semantics.vars));
                                    // (pt(t2), vars(t1))
                                    dep.push((right_semantics.provide, left_semantics.vars));
                                    // (∅, dt(t1) ∪ dt(t2))
                                    dep.push((
                                        HashSet::new(),
                                        left_semantics
                                            .depend
                                            .union(&right_semantics.depend)
                                            .cloned()
                                            .collect(),
                                    ));
                                }

                                Self::update_dependencies_for_node(
                                    &document.semantics,
                                    node.id(),
                                    dep,
                                );
                            }
                            _ => {}
                        }
                    } else {
                        let dep = vec![(
                            HashSet::new(),
                            document
                                .semantics
                                .get_statement_semantics_for_node(atom.id())
                                .vars,
                        )];
                        Self::update_dependencies_for_node(&document.semantics, node.id(), dep);
                    }
                }
            }
            "conjunction" => {
                // We have a conditional literal
                if node.child_count() >= 3 {
                    let vars = document
                        .semantics
                        .get_statement_semantics_for_node(node.id())
                        .vars;
                    let local_literal = node.child(0).unwrap();
                    let condition = node.child(2).unwrap();

                    let dep = vec![(HashSet::new(), vars)];
                    Self::update_dependencies_for_node(&document.semantics, node.id(), dep);

                    // for the global context we return all variables that are not in the local context
                    let local_context = document
                        .semantics
                        .get_statement_semantics_for_node(local_literal.id());
                    let condition_context = document
                        .semantics
                        .get_statement_semantics_for_node(condition.id());
                    Self::update_global_vars_for_node(
                        &document.semantics,
                        node.id(),
                        local_context
                            .vars
                            .difference(&condition_context.vars)
                            .cloned()
                            .collect(),
                    );
                }
            }
            "bodycomma" => {
                // If we only have 2 children we pass on the dependency for the first children (ignore the last child which is a comma)
                if node.child_count() >= 2 {
                    let mut hash = HashSet::new();
                    hash.insert("literal".to_string());
                    hash.insert("bodycomma".to_string());
                    hash.insert("bodydot".to_string());
                    hash.insert("conjunction".to_string());
                    hash.insert("lubodyaggregate".to_string());
                    Self::pass_on_dependencies_from_children(node, document, &hash);
                }
            }
            "bodydot" => {
                if node.child_count() >= 1 {
                    //If we have at least 1 child pass on the dependencies from each of the literals
                    let mut hash = HashSet::new();
                    hash.insert("literal".to_string());
                    hash.insert("bodycomma".to_string());
                    hash.insert("bodydot".to_string());
                    hash.insert("conjunction".to_string());
                    hash.insert("lubodyaggregate".to_string());
                    Self::pass_on_dependencies_from_children(node, document, &hash);
                }
            }
            "litvec" | "optcondition" | "optimizelitvec" | "optimizecond" => {
                Self::pass_on_dependencies_from_children(node, document, &HashSet::new());
            }
            "lubodyaggregate" => {
                if node.child_count() >= 2 {
                    let mut aggregate: Option<Node> = None;
                    let mut lower_bounds: Option<&str> = None;
                    let mut lower_bounds_term: Option<Node> = None;
                    let mut upper_bounds: Option<&str> = None;
                    let mut upper_bounds_term: Option<Node> = None;

                    // When we have an aggregate with bounds, we need to check where the bounds are
                    for child in node.children(&mut node.walk()) {
                        match child.kind() {
                            "bodyaggregate" => {
                                aggregate = Some(child);
                            }
                            "upper" => {
                                if child.child_count() >= 2 {
                                    let cmp = child.child(0).unwrap();
                                    if cmp.child_count() >= 1 {
                                        upper_bounds = Some(cmp.child(0).unwrap().kind());
                                    }
                                    upper_bounds_term = Some(child.child(1).unwrap());
                                }
                            }
                            "term" => {
                                lower_bounds_term = Some(child);
                            }
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
                    if let Some(aggr) = aggregate {
                        aggr_vars = document
                            .semantics
                            .get_statement_semantics_for_node(aggr.id())
                            .vars;
                    }

                    //Depending if certain bounds exist and their types we now set the dependencies
                    if lower_bounds.is_some()
                        && lower_bounds.unwrap() == "EQ"
                        && lower_bounds_term.is_some()
                    {
                        // If there is an assignment then we need a different global dependency
                        let lower_bounds_term_semantics = document
                            .semantics
                            .get_statement_semantics_for_node(lower_bounds_term.unwrap().id());
                        dependencies.push((lower_bounds_term_semantics.provide, aggr_vars.clone()));
                        dependencies.push((HashSet::new(), lower_bounds_term_semantics.depend));

                        global_vars.extend(
                            document
                                .semantics
                                .get_statement_semantics_for_node(lower_bounds_term.unwrap().id())
                                .vars,
                        );
                    }
                    if upper_bounds.is_some() && upper_bounds.unwrap() == "EQ" {
                        let upper_bounds_term_semantics = document
                            .semantics
                            .get_statement_semantics_for_node(upper_bounds_term.unwrap().id());
                        dependencies.push((upper_bounds_term_semantics.provide, aggr_vars));
                        dependencies.push((HashSet::new(), upper_bounds_term_semantics.depend));

                        global_vars.extend(
                            document
                                .semantics
                                .get_statement_semantics_for_node(upper_bounds_term.unwrap().id())
                                .vars,
                        );
                    }
                    if !((lower_bounds.is_some() && lower_bounds.unwrap() == "EQ")
                        || (upper_bounds.is_some() && upper_bounds.unwrap() == "EQ"))
                    {
                        // If there isn't an assignment then all variables will be the dependency for this aggregate
                        let vars = document
                            .semantics
                            .get_statement_semantics_for_node(node.id())
                            .vars;
                        dependencies.push((HashSet::new(), vars));
                    }

                    Self::update_dependencies_for_node(
                        &document.semantics,
                        node.id(),
                        dependencies,
                    );
                    Self::update_global_vars_for_node(&document.semantics, node.id(), global_vars);
                }
            }
            "altheadaggrelemvec" => {
                let mut global_vars = HashSet::new();

                if node.child_count() == 1 {
                    Self::pass_on_global_vars_from_children(node, document);
                    Self::pass_on_special_literals_from_children(node, document);
                } else if node.child_count() == 2 {
                    let terms = node.child(0).unwrap();
                    let terms_semantics = document
                        .semantics
                        .get_statement_semantics_for_node(terms.id());
                    let condition = node.child(1).unwrap();
                    let condition_semantics = document
                        .semantics
                        .get_statement_semantics_for_node(condition.id());
                    let mut local_dependency = Vec::new();

                    // if there is an condition only pass on the variables which are not in the condition
                    let vars: HashSet<String> = terms_semantics
                        .vars
                        .difference(&condition_semantics.vars)
                        .cloned()
                        .collect();
                    global_vars.extend(vars);

                    // find out the local dependencies
                    local_dependency.push((HashSet::new(), terms_semantics.vars));
                    local_dependency.extend(condition_semantics.dependencies);

                    //update the special literal semantics
                    Self::update_global_vars_for_node(&document.semantics, node.id(), global_vars);
                    Self::update_special_literals_for_node(
                        &document.semantics,
                        node.id(),
                        vec![SpecialLiteralSemantics::new_with_dep(
                            &node,
                            local_dependency,
                        )],
                    );
                } else if node.child_count() >= 3 {
                    let left_child = node.child(0).unwrap();
                    let left_semantics = document
                        .semantics
                        .get_statement_semantics_for_node(left_child.id());
                    let operator = node.child(1).unwrap();
                    let right_child = node.child(2).unwrap();
                    let right_semantics = document
                        .semantics
                        .get_statement_semantics_for_node(right_child.id());

                    if operator.kind() != "SEM" {
                        return;
                    }

                    // pass on everything from the left child
                    global_vars = global_vars
                        .union(&left_semantics.global_vars)
                        .cloned()
                        .collect::<HashSet<String>>();
                    let mut special_literals = left_semantics.special_literals;

                    if node.child_count() >= 4 {
                        let condition = node.child(3).unwrap();
                        let mut local_dependency = Vec::new();
                        let condition_semantics = document
                            .semantics
                            .get_statement_semantics_for_node(condition.id());

                        // if there is an condition only pass on the variables which are not in the condition
                        let vars: HashSet<String> = right_semantics
                            .vars
                            .difference(&condition_semantics.vars)
                            .cloned()
                            .collect();
                        global_vars.extend(vars);

                        // find out the local dependencies
                        local_dependency.push((HashSet::new(), right_semantics.vars));
                        local_dependency.extend(condition_semantics.dependencies);

                        //update the special literal semantics
                        special_literals.push(SpecialLiteralSemantics::new_with_dep(
                            &node,
                            local_dependency,
                        ));
                    } else {
                        // pass on everything from the right child if there is no condition
                        global_vars = global_vars.union(&right_semantics.vars).cloned().collect();

                        //pass on all the dependencies from both children
                        special_literals.extend(right_semantics.special_literals.clone());
                    }

                    Self::update_global_vars_for_node(&document.semantics, node.id(), global_vars);
                    Self::update_special_literals_for_node(
                        &document.semantics,
                        node.id(),
                        special_literals,
                    );
                }
            }
            "disjunction" => {
                // We have a disjunction in the head
                let vars = document
                    .semantics
                    .get_statement_semantics_for_node(node.id())
                    .vars;
                let dep = vec![(HashSet::new(), vars)];
                let mut global_vars = HashSet::new();
                let mut offset = 0;

                if node.child_count() == 0 {
                    return;
                }

                if node.child_count() >= 1 {
                    let disjunctionsep = node.child(0).unwrap();

                    if disjunctionsep.kind() == "disjunctionsep" {
                        //Pass on any global variables from the disjunction seperator children
                        global_vars.extend(
                            document
                                .semantics
                                .get_statement_semantics_for_node(disjunctionsep.id())
                                .global_vars,
                        );
                        offset += 1;
                    }
                }

                if node.child_count() == 2 && node.child(0).unwrap().kind() == "disjunctionsep" {
                    //There is no condition to this disjunction this means that this child should be a literal and needs to pass on its global vars
                    global_vars.extend(
                        document
                            .semantics
                            .get_statement_semantics_for_node(node.child(1).unwrap().id())
                            .global_vars,
                    );
                } else if node.child_count() >= 3 {
                    // literal -- colon -- litvec
                    let local_literal = node.child(offset).unwrap();
                    let seperator = node.child(offset + 1).unwrap();

                    //Find the correct node that represents the condition
                    let condition: Node = if seperator.kind() == "COLON" {
                        node.child(offset + 2).unwrap()
                    } else if seperator.kind() == "optcondition" && seperator.child_count() >= 2 {
                        seperator.child(2).unwrap()
                    } else {
                        // Should never trigger
                        node.child(0).unwrap()
                    };

                    // for the global context we return all variables that are not in the local context
                    global_vars.extend(
                        document
                            .semantics
                            .get_statement_semantics_for_node(local_literal.id())
                            .vars
                            .difference(
                                &document
                                    .semantics
                                    .get_statement_semantics_for_node(condition.id())
                                    .vars,
                            )
                            .cloned(),
                    );
                }

                Self::update_global_vars_for_node(&document.semantics, node.id(), global_vars);
                Self::update_dependencies_for_node(&document.semantics, node.id(), dep);
            }
            "headaggregate" => {}
            "luheadaggregate" => {}
            "minelemlist" | "maxelemlist" => {
                let mut dependencies = Vec::new();

                // we have an optimization statement
                if node.child_count() >= 2 {
                    let weight = node.child(0).unwrap();
                    let weight_semantics = document
                        .semantics
                        .get_statement_semantics_for_node(weight.id());
                    let condition;
                    if node.child_count() >= 3 {
                        let tuple = node.child(1).unwrap();
                        let tuple_semantics = document
                            .semantics
                            .get_statement_semantics_for_node(tuple.id());
                        condition = node.child(2).unwrap();

                        //Add all variables in the tuple to the dependency list

                        dependencies.push((HashSet::new(), tuple_semantics.vars.clone()));

                        //Add all variables in the tuple to the global variables list
                        Self::update_global_vars_for_node(
                            &document.semantics,
                            tuple.id(),
                            tuple_semantics.vars,
                        );
                    } else {
                        condition = node.child(1).unwrap();
                    }

                    let condition_semantics = document
                        .semantics
                        .get_statement_semantics_for_node(condition.id());

                    //Add all variables in the weight to the global variables list
                    Self::update_global_vars_for_node(
                        &document.semantics,
                        weight.id(),
                        weight_semantics.vars.clone(),
                    );

                    //Add all variables in the weight to the dependency list
                    dependencies.push((HashSet::new(), weight_semantics.vars));

                    //Take all the dependencies from the condition
                    dependencies.extend(condition_semantics.dependencies);
                }

                Self::update_dependencies_for_node(&document.semantics, node.id(), dependencies);
            }
            _ => {}
        }
    }

    /**
     * Check what each part of the statement depends and provides as a dependency tuple
     */
    fn check_global_vars(node: Node, document: &mut DocumentData) {
        // Pass on global vars if they are not handled in dependency check
        match node.kind() {
            "literal" => {
                // If we have a literal then pass on all variables as global (Could be to increase performance moved into vars check)
                Self::update_global_vars_for_node(
                    &document.semantics,
                    node.id(),
                    document
                        .semantics
                        .get_statement_semantics_for_node(node.id())
                        .vars,
                );
            }
            "conjunction" | "lubodyaggregate" | "altheadaggrelemvec" | "disjunction" => {}
            "source_file" => {} // Ignore any fields above statements
            _ => {
                Self::pass_on_global_vars_from_children(node, document);
            }
        }
    }

    /**
     * Pass on special literals if needed
     */
    fn check_special_literals(node: Node, document: &mut DocumentData) {
        // Pass on special literals if needed
        match node.kind() {
            "conjunction" | "altheadaggrelemvec" | "disjunction" => {}
            "bodyaggrelem" => Self::update_special_literals_for_node(
                &document.semantics,
                node.id(),
                vec![SpecialLiteralSemantics::new(&node, document)],
            ),
            "source_file" => {} // Ignore any fields above statements
            _ => {
                Self::pass_on_special_literals_from_children(node, document);
            }
        }
    }
}

impl Semantics for StatementSemantics {
    fn on_node(node: Node, document: &mut DocumentData) {
        StatementSemantics::check_for_variables(node, document);
        StatementSemantics::check_provide(node, document);
        StatementSemantics::check_depend(node, document);
        StatementSemantics::check_dependencies(node, document);
        StatementSemantics::check_global_vars(node, document);
        StatementSemantics::check_special_literals(node, document);
    }
}

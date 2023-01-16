/**
 * Predicate Occurence Semantics infers information where a predicate occured
 */
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PredicateOccurenceSemantics {
    pub node_id: usize,
    pub range: tree_sitter::Range,
    pub location: PredicateOccurenceLocation,
}

/**
 * The location of an occurence in the encoding
 */
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum PredicateOccurenceLocation {
    Head,
    Body,
    Condition,
}

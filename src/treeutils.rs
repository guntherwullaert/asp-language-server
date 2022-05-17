use tree_sitter::{Query, QueryCursor, TreeCursor};

/**
 * Convert a token value into a human readable string
 */
pub fn humanize_token(token: &String) -> String {
    if token == "RPAREN" {
        ")".to_string()
    } else if token == "LPAREN" {
        "(".to_string()
    } else if token == "RBRACE" {
        "}".to_string()
    } else if token == "LBRACE" {
        "{".to_string()
    } else {
        return format!("UNKNOWN TOKEN CONVERSION: {}", token);
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

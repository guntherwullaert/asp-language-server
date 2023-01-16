use tree_sitter::TreeCursor;

/**
 * Convert a token value into a human readable string
 */
pub fn humanize_token(token: &str) -> &str {
    match token {
        "RPAREN" => ")",
        "LPAREN" => "(",
        "RBRACE" => "{",
        "LBRACE" => "}",
        "COMMA" => ",",
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

use tree_sitter::TreeCursor;

/**
 * Convert a token value into a human readable string
 */
pub fn humanize_token(token: &String) -> String {
    if token == "RPAREN" {
        ")".to_string()
    } else if token == "LPAREN" {
        "(".to_string()
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

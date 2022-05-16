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

use super::context_location::{get_location_from_context, ContextLocation};
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation, InsertTextFormat};
use tree_sitter::Node;

/**
 * Resolve a keyword completion
 */
pub fn keyword_completion_resolver(node: Option<Node>) -> Option<Vec<CompletionItem>> {
    let mut items = Vec::new();
    let context_location = get_location_from_context(node);

    match context_location {
        ContextLocation::Body | ContextLocation::Head => {
            items.extend(create_completion_items_for_body_head())
        }
        _ => items.extend(create_completion_items_for_statement()),
    }

    Some(items)
}

/**
 * Create a completion item
 * keyword: The keyword that is going to be shown in bold
 * replace_text: The snippet used by the client to generate the new text after completion
 * documentation: The documentation shown to the user
 * detail: Some detailed info shown on the right to distinguish between multiple variants
 */
pub fn create_keyword_completion_item(
    keyword: &str,
    replace_text: &str,
    documentation: &str,
    detail: &str,
) -> CompletionItem {
    CompletionItem {
        label: keyword.to_string(),
        insert_text: Some(replace_text.to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        kind: Some(CompletionItemKind::KEYWORD),
        documentation: Some(Documentation::String(documentation.to_string())),
        detail: Some(detail.to_string()),
        ..Default::default()
    }
}

/**
 * Create all completion items while a user is creating a new statement
 */
pub fn create_completion_items_for_statement() -> Vec<CompletionItem> {
    vec![
        (create_keyword_completion_item(
            "show",
            "show $1.\n$0",
            "Shows only a specific number of items in the output answer set",
            "show (1).",
        )),
        (create_keyword_completion_item(
            "minimize",
            "minimize{${1:()}@${2:()},${3:()}:${4:()}}.\n$0",
            "Optimization statement",
            "minimize{(1)@(2),(3):(4)}.",
        )),
        (create_keyword_completion_item(
            "maximize",
            "maximize{${1:()}@${2:()},${3:()}:${4:()}}.\n$0",
            "Optimization statement",
            "maximize{(1)@(2),(3):(4)}.",
        )),
        (create_keyword_completion_item(
            "minimise",
            "minimise{${1:()}@${2:()},${3:()}:${4:()}}.\n$0",
            "Optimization statement",
            "minimise{(1)@(2),(3):(4)}.",
        )),
        (create_keyword_completion_item(
            "maximise",
            "maximise{${1:()}@${2:()},${3:()}:${4:()}}.\n$0",
            "Optimization statement",
            "maximise{(1)@(2),(3):(4)}.",
        )),
        (create_keyword_completion_item(
            "external",
            "external $1.\n$0",
            "Do not let the grounder optimize this predicate",
            "external (1).",
        )),
        (create_keyword_completion_item(
            "program",
            "program $1.\n$0",
            "Organize the encoding in multiple program parts",
            "program (1).",
        )),
        (create_keyword_completion_item(
            "const",
            "const $1.\n$0",
            "Declare a constant to be replaced by the grounder",
            "const (1).",
        )),
        (create_keyword_completion_item("edge", "edge($1).\n$0", "???", "edge((1)).")),
        (create_keyword_completion_item("heuristic", "heuristic $1.\n$0", "???", "heuristic (1).")),
        (create_keyword_completion_item("project", "project $1.\n$0", "???", "project (1).")),
        (create_keyword_completion_item("script", "script $1.\n$0", "???", "script (1).")),
        (create_keyword_completion_item(
            "defined",
            "defined $1.\n$0",
            "Denote to the grounder that a predicate is defined in another file",
            "defined (1).",
        )),
    ]
}

/**
 * Create all completion items while a user is creating the body or head of a statement
 */
pub fn create_completion_items_for_body_head() -> Vec<CompletionItem> {
    vec![
        (create_keyword_completion_item(
            "sup",
            "sup$0",
            "represents the greates element among all variable-free terms",
            "sup",
        )),
        (create_keyword_completion_item(
            "supremum",
            "supremum$0",
            "represents the greates element among all variable-free terms",
            "supremum",
        )),
        (create_keyword_completion_item(
            "inf",
            "inf$0",
            "represents the smallest element among all variable-free terms",
            "inf",
        )),
        (create_keyword_completion_item(
            "infimum",
            "infimum$0",
            "represents the smallest element among all variable-free terms",
            "infimum",
        )),
        (create_keyword_completion_item(
            "sum",
            "sum{${1:()} : ${2:()}}$0",
            "sum up the weights",
            "sum{(1) : (2)}",
        )),
        (create_keyword_completion_item(
            "sum+",
            "sum+{${1:()} : ${2:()}}$0",
            "sum up the number of positive weights",
            "sum+{(1) : (2)}",
        )),
        (create_keyword_completion_item(
            "count",
            "count{${1:()} : ${2:()}}$0",
            "count the number of elements",
            "count{(1) : (2)}",
        )),
        (create_keyword_completion_item(
            "min",
            "min{${1:()} : ${2:()}}$0",
            "returns the minimum weight",
            "min{(1) : (2)}",
        )),
        (create_keyword_completion_item(
            "max",
            "max{${1:()} : ${2:()}}$0",
            "returns the maximum weight",
            "max{(1) : (2)}",
        )),
        (create_keyword_completion_item("false", "false$0", "???", "false")),
        (create_keyword_completion_item("true", "true$0", "???", "true")),
    ]
}

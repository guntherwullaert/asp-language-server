use log::info;
use tower_lsp::lsp_types::{CompletionContext, CompletionItem, CompletionItemKind, InsertTextFormat};
use tree_sitter::Node;

use crate::document::DocumentData;

use super::context_location::get_location_from_context;

/**
 * Resolve a keyword completion
 */
pub fn predicate_completion_resolver(document: &DocumentData, context: CompletionContext, node: Option<Node>) -> Option<Vec<CompletionItem>>{
    let mut items = Vec::new();
    let context_location = get_location_from_context(document, context, node);

    if node.is_some() {
        let mut parent = node.unwrap().parent();
        while parent.is_some() {
            if parent.unwrap().kind() == "statement" {
                //Find all variables used in this statement and return this to the user
                let vars = document.semantics.get_statement_semantics_for_node(parent.unwrap().id()).vars;

                for var in vars {
                    items.push(create_variable_completion_item(var));
                }

                break;
            }
            parent = parent.unwrap().parent();
        }

        // Give a suggestion for each atom in the document
        for ((identifier, arity), occurences) in document.semantics.predicate_semantics.predicates.clone() {
            let mut insert_text_snippet = identifier.clone();

            // Check if this predicate isn't at the current location of the cursor, if it is we do not suggest this predicate
            if occurences.len() == 1 && document.get_source_for_range(node.unwrap().range()) == identifier{
                continue;
            }

            if arity > 0 {
                insert_text_snippet += "(";

                for i in 1 .. arity + 1 {
                    insert_text_snippet += &format!("${{{:?}:()}}", i).to_string();
                    if arity >= i+1 {
                        insert_text_snippet += ", ";
                    }
                }

                insert_text_snippet += ")";
            }

            insert_text_snippet += "$0";

            items.push(create_predicate_completion_item(identifier, arity, insert_text_snippet));
        }
    }

    Some(items)
}

/**
 * Create a completion item for predicates
 * identifier: The identifier that is going to be shown in bold with it's arity
 * insert_text_snippet: The snippet used by the client to generate the new text after completion
 */
pub fn create_predicate_completion_item(identifier: String, arity: usize, insert_text_snippet: String) -> CompletionItem {
    CompletionItem {
        label: identifier + "/" + &arity.to_string(),
        insert_text: Some(insert_text_snippet),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        kind: Some(CompletionItemKind::FIELD),
        ..Default::default()
    }
}

/**
 * Create a completion item for variables
 * variable: The variable that is going to be shown in bold
 */
pub fn create_variable_completion_item(variable: String) -> CompletionItem {
    CompletionItem {
        label: variable.clone(),
        kind: Some(CompletionItemKind::VARIABLE),
        ..Default::default()
    }
}
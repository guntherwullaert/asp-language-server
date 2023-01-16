use tower_lsp::lsp_types::{CompletionContext, CompletionItem, CompletionTriggerKind, Position};
use tree_sitter::{Node, Point};

use crate::document::DocumentData;

use self::{
    keyword_competion::keyword_completion_resolver,
    predicate_completion::predicate_completion_resolver,
};

mod context_location;
mod keyword_competion;
mod predicate_completion;

/**
 * Upon detecting a completion trigger, check what the trigger was and run the correct completion resolver
 */
pub fn check_completion(
    document: &DocumentData,
    context: CompletionContext,
    trigger_character: String,
    position: Position,
) -> Option<Vec<CompletionItem>> {
    //Client requested completion

    let node: Option<Node> = if position.character > 0 {
        document.tree.root_node().descendant_for_point_range(
            Point {
                row: position.line as usize,
                column: (position.character - 1) as usize,
            },
            Point {
                row: position.line as usize,
                column: (position.character - 1) as usize,
            },
        )
    } else {
        None
    };

    if trigger_character == "#" {
        return keyword_completion_resolver(node);
    } else if context.trigger_kind == CompletionTriggerKind::INVOKED {
        return predicate_completion_resolver(document, node);
    }

    None
}

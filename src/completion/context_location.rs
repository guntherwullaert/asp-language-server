use log::info;
use tower_lsp::lsp_types::{CompletionContext, Position};
use tree_sitter::{Point, Node};

use crate::document::DocumentData;

#[derive(Clone, Debug, PartialEq)]
pub enum ContextLocation {
    Unknown,
    Statement,
    Head,
    Body
}

pub fn get_location_from_context(document: &DocumentData, context : CompletionContext, context_node: Option<Node>) -> ContextLocation{

    if let Some(node) = context_node {
        let mut parent = Some(node);
        while parent.is_some() {
            let unwraped_parent = parent.unwrap();
            let mut location = ContextLocation::Unknown;

            if let Some(prev_sibling) = unwraped_parent.prev_sibling() {
                if prev_sibling.kind() == "IF" {
                    location = ContextLocation::Body;
                }
            } else {            
                location = match unwraped_parent.kind() {
                    "head" => ContextLocation::Head,
                    "bodydot" | "bodycomma" => ContextLocation::Body,
                    "source_file" | "statement" => ContextLocation::Statement,
                    _ => ContextLocation::Unknown
                }
            }

            if location != ContextLocation::Unknown {
                return location;
            }
            parent = unwraped_parent.parent();
        }
    }

    ContextLocation::Unknown
}
use tree_sitter::Node;

/**
 * A location in the document
 */
#[derive(Clone, Debug, PartialEq)]
pub enum ContextLocation {
    Unknown,
    Statement,
    Head,
    Body,
}

/**
 * Based on the context of the node, return the location of it in the document
 * e.g. in the head or body.
 */
pub fn get_location_from_context(context_node: Option<Node>) -> ContextLocation {
    //Keep checking the parent node until we reach a head, body or reach the root of the document, that is our location
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
                    _ => ContextLocation::Unknown,
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

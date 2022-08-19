use std::time::Instant;

use log::info;
use rust_lapper::Lapper;
use tree_sitter::Range;

use crate::document::DocumentData;

use self::encoding_semantic::{EncodingSemantics};

pub mod encoding_semantic;
mod error_semantic;
mod missing_semantic;
mod statement_semantic;
mod term_semantic;
mod syntax;

/**
 * Goes through the tree post order and populates the encoding semantics object in the document
 */
pub fn analyze_tree(
    document: &mut DocumentData,
    changed_ranges: &Option<Lapper<usize, usize>>
) {
    let doc = document.clone();
    let mut cursor = doc.tree.walk();

    let time = Instant::now();

    EncodingSemantics::startup(document);

    let duration = time.elapsed();
    info!("Time needed for starting up semantic analysis: {:?}", duration);

    let time = Instant::now();
    let mut reached_root = false;
    while !reached_root {
        if cursor.goto_first_child() {
            continue;
        }

        let node = cursor.node();

        if cursor.goto_next_sibling() {
            EncodingSemantics::on_node(node, document, changed_ranges);
            continue;
        }

        loop {
            EncodingSemantics::on_node(cursor.node(), document, changed_ranges);

            if !cursor.goto_parent() {
                reached_root = true;
                break;
            }

            let node = cursor.node();

            if cursor.goto_next_sibling() {
                EncodingSemantics::on_node(node, document, changed_ranges);
                break;
            }
        }
    }

    let duration = time.elapsed();
    info!("Time needed for semantic analysis: {:?}", duration);

    let time = Instant::now();
    EncodingSemantics::cleanup(document);
    let duration = time.elapsed();
    info!("Time needed for semantic analysis cleanup: {:?}", duration);
}

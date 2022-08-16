use std::time::Instant;

use log::info;
use ropey::Rope;
use tower_lsp::lsp_types::{Url, TextDocumentContentChangeEvent, Position};
use tree_sitter::{Tree, InputEdit, Point, Parser, Range};

use crate::semantics::{analyze_tree, encoding_semantic::EncodingSemantics};

#[derive(Debug, Clone)]
pub struct DocumentData {
    pub uri: Url,
    pub tree: Tree,
    pub source: Rope,
    pub version: i32,
    pub semantics: EncodingSemantics
}
impl DocumentData {
    pub fn new(uri: Url, tree: Tree, source: Rope, version: i32) -> DocumentData {
        DocumentData {
            uri,
            tree,
            source,
            version,
            semantics: EncodingSemantics::new()
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut array = Vec::with_capacity(self.source.len_bytes());

        for byte in self.source.bytes() {
            array.push(byte);
        }

        array
    }

    pub fn convert_position_to_point(position : Position) -> Point {
        Point {
            row: position.line as usize,
            column: position.character as usize
        }
    }

    pub fn get_source_for_range(&self, range: Range) -> String {
        return self.source.byte_slice(range.start_byte..range.end_byte).as_str().unwrap().to_string();
    }

    pub fn update_document(&mut self, changes: Vec<TextDocumentContentChangeEvent>, parser: &mut Parser) {
        
        let old_tree = &self.tree.clone();
        let mut changed_ranges_test: Vec<(usize, usize)> = Vec::with_capacity(10);

        // Go over each change in order and apply them to to the rope
        for change in changes {
            if change.range.is_none() {
                info!("Got a text document change without a range: {:?}", change);
                continue;
            }

            // Figure out where we should replace this rope
            let time = Instant::now();
            let range = change.range.unwrap();
            let start_char = self.source.line_to_char(range.start.line as usize) + range.start.character as usize;
            let end_char = self.source.line_to_char(range.end.line as usize) + range.end.character as usize;

            let start_byte = self.source.char_to_byte(start_char);
            let old_end_byte = self.source.char_to_byte(end_char);

            //First remove the range from the rope
            self.source.remove(start_char..end_char);

            //Then add the new changes to the rope
            self.source.insert(start_char, &change.text);

            let new_end_char = start_char + change.text.chars().count();
            let new_end_byte = start_byte + change.text.len();
            let new_end_line = self.source.char_to_line(new_end_char);
            let new_end_column = new_end_byte - self.source.line_to_byte(new_end_line);
            let new_end_position = Point {
                row: new_end_line,
                column: new_end_column
            };

            let duration = time.elapsed();
            info!("Time needed for updating the rope: {:?}", duration);

            let time = Instant::now();
            //Update the abstract syntax tree
            self.tree.edit(&InputEdit {
                start_byte,
                start_position: DocumentData::convert_position_to_point(range.start),
                old_end_byte,
                old_end_position: DocumentData::convert_position_to_point(range.end),
                new_end_byte,
                new_end_position 
            });

            if start_byte <= new_end_byte {
                changed_ranges_test.push((start_byte, new_end_byte));
            } else  {
                changed_ranges_test.push((new_end_byte, start_byte));
            }

            let duration = time.elapsed();
            info!("Time needed for editing the tree: {:?}", duration);
        }

        let time = Instant::now();

        self.tree = parser.parse(self.get_bytes(), Some(&self.tree)).unwrap();

        let duration = time.elapsed();
        info!("Time needed for parsing the rope: {:?}", duration);

        let time = Instant::now();
        let mut changed_ranges: Vec<(usize, usize)> = Vec::with_capacity(10);
        // As we only use the start and end we combine duplicate values to significantly increase performance in the semantic analysis
        /*for change in old_tree.changed_ranges(&self.tree) {
            let mut found = false;

            for i in 0..changed_ranges.len() {
                // start_byte < end && end_byte > start (Is this range already contained in this changed_range)
                if change.start_byte < changed_ranges[i].1 && change.end_byte > changed_ranges[i].0 {
                    // If so we extend the start_byte and end_byte
                    let mut start = changed_ranges[i].0;
                    let mut end = changed_ranges[i].1;
                    if change.start_byte < start {
                        start = change.start_byte;
                    }
                    if change.end_byte > end {
                        end = change.end_byte;
                    }

                    changed_ranges[i] = (start, end);
                    found = true;
                    break;
                }
            }
            if !found {
                changed_ranges.push((change.start_byte, change.end_byte));
            }
        }*/

        for (range_start, range_end) in changed_ranges_test.clone() {
            let mut found = false;

            for i in 0..changed_ranges.len() {
                // start_byte < end && end_byte > start (Is this range already contained in this changed_range)
                if range_start < changed_ranges[i].1 && range_end > changed_ranges[i].0 {
                    // If so we extend the start_byte and end_byte
                    let mut start = changed_ranges[i].0;
                    let mut end = changed_ranges[i].1;
                    if range_start < start {
                        start = range_start;
                    }
                    if range_end > end {
                        end = range_end;
                    }

                    changed_ranges[i] = (start, end);
                    found = true;
                    break;
                }
            }
            if !found {
                changed_ranges.push((range_start, range_end));
            }
        }

        let duration = time.elapsed();
        info!("Time needed for finding the ranges that changed: {:?}", duration);


        info!("Changed Ranges: {:?}", changed_ranges_test);

        self.generate_semantics(Some(changed_ranges));
    }

    pub fn generate_semantics(&mut self, changed_ranges: Option<Vec<(usize, usize)>>) {
        analyze_tree(self, &changed_ranges);

        //info!("Statement Semantics: {:?}", self.semantics.statement_semantics);
    }
}

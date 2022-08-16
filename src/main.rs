use std::thread;
use std::time::Instant;

use dashmap::DashMap;
use diagnostics::run_diagnostics;
use document::DocumentData;
use log::info;
use semantics::encoding_semantic::EncodingSemantics;
use serde::{Deserialize, Serialize};
use ropey::Rope;
use tokio::task::{self, JoinHandle};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use tree_sitter::{Parser, Point};

mod diagnostics;
mod document;
mod semantics;

#[cfg(test)]
mod test_utils;

struct Backend {
    client: Client,
    document_map: DashMap<String, DocumentData>,
    analysis_handle: Option<JoinHandle<EncodingSemantics>>,
    diagnostics_handle: Option<JoinHandle<()>>
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["#".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        info!("File {} opened with text: {:?} and version {:?}", params.text_document.uri, params.text_document.text, params.text_document.version);

        let time = Instant::now();

        // Use rope for an efficient way to access byte offsets and string slices
        let rope = ropey::Rope::from_str(&params.text_document.text);

        // Parse the document and save the parse tree in a hashmap
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_clingo::language()).expect("Error loading clingo grammar");
        let tree = parser.parse(params.text_document.text.clone(), None).unwrap();
        let mut doc = DocumentData::new(params.text_document.uri.clone(), tree, rope.clone(), params.text_document.version);

        let duration = time.elapsed();
        info!("Time needed for first time generating the document: {:?}", duration);
        doc.generate_semantics(None);
        self.document_map.insert(params.text_document.uri.to_string(), doc);

        // Run diagnostics for that file
        /*let time = Instant::now();
        run_diagnostics(
            &self.client,
            &mut self.document_map.get(&params.text_document.uri.to_string()).unwrap().clone(),
            100,
        )
        .await;
        let duration = time.elapsed();
        info!("Time needed for diagnostics: {:?}", duration);*/
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let client_copy = self.client.clone();
        let uri = params.text_document.uri.clone().to_string();

        /*if !self.document_map.contains_key(&uri) {
            self.client
                .log_message(
                    MessageType::ERROR,
                    format!("Document {} changed before opening!", uri),
                )
                .await;
            return;
        }*/

        //TODO: Figure out if we are running a semantic analysis if so, cancel that semantic analysis
        info!("Document change incoming for document: {}\nWith the following changes: {:?}", uri, params.content_changes.clone());
        
        let mut document = self.document_map.get_mut(&uri).unwrap().clone();

        info!("Got document reference");
        
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_clingo::language()).expect("Error loading clingo grammar");

        document.update_document(params.content_changes, &mut parser);

        /*if self.diagnostics_handle.is_some() {
            self.diagnostics_handle.unwrap().abort();
        }

        self.diagnostics_handle = Some(task::spawn(async {
            
        }));

        let result = self.diagnostics_handle.unwrap().await;

        if result.is_err() {
            info!("ERROR:  {:?}", result)
        }*/

        let time = Instant::now();
        let diagnostics = run_diagnostics(
            document.clone(),
            100,
        );
        info!("Fetched Diagnostics");
        client_copy.publish_diagnostics(
            params.text_document.uri.clone(),
            Vec::new(),
            Some(1),
        ).await;
        let duration = time.elapsed();
        info!("Time needed for diagnostics: {:?}", duration);

        // Run diagnostics for that file
        //tokio::spawn(async {});
        /*let time = Instant::now();
        run_diagnostics(
            &self.client,
            &mut document.value().clone(),
            100,
        )
        .await;
        let duration = time.elapsed();
        info!("Time needed for diagnostics: {:?}", duration);*/

        
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;

        if !self.document_map.contains_key(&uri) {
            self.client
                .log_message(
                    MessageType::ERROR,
                    format!("Document {} closed before opening!", uri),
                )
                .await;
            return;
        }

        // Remove our information for this file
        self.document_map.remove(&uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let context = params.context;

        let mut ret = Vec::new();

        let completions = || -> Option<Vec<CompletionItem>> {
            let document = self.document_map.get(&uri.to_string())?;

            //TODO: Keep track if analysis has been done yet
            /*let semantics = analyze_tree(&document.tree, &document.source.to_string());

            if context.is_some() {
                let c = context.unwrap();
                
                if c.trigger_kind == CompletionTriggerKind::TRIGGER_CHARACTER && c.trigger_character.is_some() && c.trigger_character.unwrap() == "#" {
                    let completions = [
                        ("show", "show $1.\n$0", "show (1).", "Shows only a specific number of items in the output answer set"), 
                        ("external", "external $1.\n$0", "external (1).", "Do not let the grounder optimize this predicate"), 
                        ("count", "count{${1:()} : ${2:()}}$0", "count{(1) : (2)}", "count how many different values"), 
                        ("sum", "sum{${1:()} : ${2:()}}$0", "sum{(1) : (2)}", "sum up the values"),
                        ("minimize", "minimize{${1:()}@${2:()},${3:()}:${4:()}}.\n$0", "#minimize{(1)@(2),(3):(4)}.","optimization statement"), 
                        ("maximize", "maximize{${1:()}@${2:()},${3:()}:${4:()}}.\n$0", "#maximize{(1)@(2),(3):(4)}.","optimization statement")
                    ];

                    //We have a completion request that started with #, output all the possible options for this
                    for (keyword, to_replace, detail, documentation) in completions{
                        ret.push(CompletionItem {
                            label: keyword.to_string(),
                            insert_text: Some(to_replace.to_string()),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            kind: Some(CompletionItemKind::KEYWORD),
                            documentation: Some(Documentation::String(documentation.to_string())),
                            detail: Some(detail.to_string()),
                            ..Default::default()
                        })
                    }
                } else if c.trigger_kind == CompletionTriggerKind::INVOKED {
                    //Client requested completion
                    //TODO: This could lead to an underflow
                    let node = document.tree.root_node().descendant_for_point_range(Point { row: position.line as usize, column: (position.character - 1) as usize }, Point { row: position.line as usize, column: (position.character - 1) as usize });

                    if node.is_some() {
                        //info!("Client wants to complete {:?} with value {:?}", node.unwrap().kind(), node.unwrap().utf8_text(document.get_bytes()));

                        let mut parent = node.unwrap().parent();
                        while parent.is_some() {
                            if parent.unwrap().kind() == "statement" {
                                //Find all variables used in this statement and return this to the user
                                let vars = semantics.get_vars_for_node(&parent.unwrap().id());

                                for var in vars {
                                    ret.push(CompletionItem {
                                        label: var.clone(),
                                        kind: Some(CompletionItemKind::VARIABLE),
                                        ..Default::default()
                                    });
                                }

                                break;
                            }
                            parent = parent.unwrap().parent();
                        }

                        // Give a suggestion for each atom in the document
                        for ((identifier, arity), atom) in semantics.atoms {
                            let mut insert_text_snippet = identifier.clone();

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

                            ret.push(CompletionItem {
                                label: identifier + "/" + &arity.to_string(),
                                insert_text: Some(insert_text_snippet),
                                insert_text_format: Some(InsertTextFormat::SNIPPET),
                                kind: Some(CompletionItemKind::FIELD),
                                ..Default::default()
                            });
                        }
                    }
                    
                    //IDEAS:
                    // Give all variables occuring in statement as completion
                    // Allow user to select predicate occuring in document with its arity --> This should give the option for variable fields to be easily went through with tab
                    // More ?

                }
            }*/
            Some(ret)
        }();
        Ok(completions.map(CompletionResponse::Array))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let definition_list = || -> Option<Vec<Location>> {
            let document = self.document_map.get(&uri.to_string())?;

            //TODO: Keep track if analysis has been done yet
            //let semantics = analyze_tree(&document.tree, &document.source);

            let mut node = document.tree.root_node().descendant_for_point_range(Point { row: position.line as usize, column: (position.character) as usize }, Point { row: position.line as usize, column: (position.character) as usize });
            let mut ret = Vec::new();

            /*while node.is_some() {
                // If we have an predicate with an identifier
                info!("reference node {:?}", node);
                if (node.unwrap().kind() == "atom" || node.unwrap().kind() == "term") && node.unwrap().child_count() >= 3 && node.unwrap().child(0).unwrap().kind() == "identifier" {
                    //TODO: Maybe create a function for this ?!?
                    let node_identifier = node.unwrap().child(0).unwrap().utf8_text(document.source.as_bytes()).unwrap().to_string();
                    let node_arity = semantics.get_atoms_arity_for_node(&node.unwrap().child(2).unwrap().id()) + 1;

                    for ((identifier, arity), atom) in semantics.atoms {
                        // Find if this is the correct identifier and arity 
                        if identifier == node_identifier && arity == node_arity {
                            // Return all occurences that are in the head
                            for occurence in atom.occurences {
                                if occurence.location == AtomOccurenceLocation::Head {
                                    let range = Range::new(Position { line: occurence.range.start_point.row as u32, character: occurence.range.start_point.column as u32}, Position { line: occurence.range.end_point.row as u32, character: occurence.range.end_point.column as u32});

                                    ret.push(Location::new(uri.clone(), range));
                                }
                            }

                            break;
                        }
                    }
                    break;
                }
                node = node.unwrap().parent();
            }*/
            Some(ret)
        }();
        let definition = Some(GotoDefinitionResponse::Array(definition_list.unwrap()));
        Ok(definition)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let reference_list = || -> Option<Vec<Location>> {
            let document = self.document_map.get(&uri.to_string())?;

            //TODO: Keep track if analysis has been done yet
            //let semantics = analyze_tree(&document.tree, &document.source);

            let mut node = document.tree.root_node().descendant_for_point_range(Point { row: position.line as usize, column: (position.character) as usize }, Point { row: position.line as usize, column: (position.character) as usize });
            let mut ret = Vec::new();

            /*while node.is_some() {
                // If we have an predicate with an identifier
                info!("reference node {:?}", node);
                if (node.unwrap().kind() == "atom" || node.unwrap().kind() == "term") && node.unwrap().child_count() >= 3 && node.unwrap().child(0).unwrap().kind() == "identifier" {
                    //TODO: Maybe create a function for this ?!?
                    let node_identifier = node.unwrap().child(0).unwrap().utf8_text(document.source.as_bytes()).unwrap().to_string();
                    let node_arity = semantics.get_atoms_arity_for_node(&node.unwrap().child(2).unwrap().id()) + 1;

                    for ((identifier, arity), atom) in semantics.atoms {
                        // Find if this is the correct identifier and arity 
                        if identifier == node_identifier && arity == node_arity {
                            // Return all occurences that are in the body or condition
                            for occurence in atom.occurences {
                                if occurence.location == AtomOccurenceLocation::Body || occurence.location == AtomOccurenceLocation::Condition {
                                    let range = Range::new(Position { line: occurence.range.start_point.row as u32, character: occurence.range.start_point.column as u32}, Position { line: occurence.range.end_point.row as u32, character: occurence.range.end_point.column as u32});

                                    ret.push(Location::new(uri.clone(), range));
                                }
                            }

                            break;
                        }
                    }
                    break;
                }
                node = node.unwrap().parent();
            }*/
            Some(ret)
        }();
        Ok(reference_list)
    }
}
#[derive(Debug, Deserialize, Serialize)]
struct InlayHintParams {
    path: String,
}

enum CustomNotification {}
impl Notification for CustomNotification {
    type Params = InlayHintParams;
    const METHOD: &'static str = "custom/notification";
}
impl Backend {

}

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {client,document_map:DashMap::new(),analysis_handle:None, diagnostics_handle: None })
    .finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}

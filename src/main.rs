use dashmap::DashMap;
use diagnostics::run_diagnostics;
use diagnostics::tree_utils::analyze_tree;
use document::DocumentData;
use log::info;
use serde::{Deserialize, Serialize};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use tree_sitter::{Parser, Point};

mod diagnostics;
mod document;

#[cfg(test)]
mod test_utils;

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, DocumentData>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec!["#".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                }),                

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
        self.client
            .log_message(
                MessageType::INFO,
                format!("file {} opened!", params.text_document.uri),
            )
            .await;

        self.on_change(
            &params.text_document.uri,
            &params.text_document.text,
            params.text_document.version,
        )
        .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        if !self.document_map.contains_key(&uri) {
            self.client
                .log_message(
                    MessageType::ERROR,
                    format!("Document {} changed before opening!", uri),
                )
                .await;
            return;
        }

        self.client
            .log_message(
                MessageType::LOG,
                format!("Document change incomming for document: {}\n", uri),
            )
            .await;

        for change in params.content_changes {
            self.on_change(
                &params.text_document.uri,
                &change.text,
                params.text_document.version,
            )
            .await;
        }
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;

        //TODO: Remove the file from our list
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let context = params.context;

        let mut ret = Vec::new();

        let completions = || -> Option<Vec<CompletionItem>> {
            let document = self.document_map.get(&uri.to_string())?;

            //TODO: Keep track if analysis has been done yet
            let semantics = analyze_tree(&document.tree, &document.source);

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
                        info!("Client wants to complete {:?} with value {:?}", node.unwrap().kind(), node.unwrap().utf8_text(document.source.as_bytes()));

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
            }
            Some(ret)
        }();
        Ok(completions.map(CompletionResponse::Array))
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
    async fn on_change(&self, uri: &Url, document: &String, version: i32) {
        // Create a Parser for this document
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_clingo::language())
            .expect("Error loading clingo grammar");

        // Parse the document and save the parse tree in a hashmap
        let tree = parser.parse(document, None).unwrap();
        let doc = DocumentData::new(uri.clone(), tree, document.clone(), version);
        self.document_map.insert(uri.to_string(), doc);

        // Run diagnostics for that file
        run_diagnostics(
            &self.client,
            &mut self.document_map.get(&uri.to_string()).unwrap().clone(),
            100,
        )
        .await;
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        document_map: DashMap::new(),
    })
    .finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}

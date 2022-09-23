use std::sync::Arc;
use std::thread;
use std::time::Instant;

use completion::check_completion;
use dashmap::DashMap;
use diagnostics::run_diagnostics;
use document::DocumentData;
use goto::definition::check_goto_definition;
use goto::references::check_goto_references;
use log::info;
use semantics::analyze_tree;
use semantics::encoding_semantic::EncodingSemantics;
use serde::{Deserialize, Serialize};
use ropey::Rope;
use tokio::runtime::Handle;
use tokio::task::{self, JoinHandle};
use tower_lsp::jsonrpc::{Result, self};
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use tree_sitter::{Parser, Point};

mod diagnostics;
mod document;
mod semantics;
mod completion;
mod goto;

#[cfg(test)]
mod test_utils;

struct Backend {
    client: Client,
    document_map: DashMap<String, DocumentData>
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
        let mut doc = DocumentData::new(params.text_document.uri.clone(), tree, rope, params.text_document.version);

        let duration = time.elapsed();
        info!("Time needed for first time generating the document: {:?}", duration);
        doc.generate_semantics(None);
        self.document_map.insert(params.text_document.uri.to_string(), doc.clone());

        // Run diagnostics for that file
        let time = Instant::now();
        let diagnostics = run_diagnostics(
            doc,
            100,
        );
        self.client.publish_diagnostics(
            params.text_document.uri.clone(),
            diagnostics,
            Some(1),
        ).await;
        let duration = time.elapsed();
        info!("Time needed for diagnostics: {:?}", duration);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let client_copy = self.client.clone();
        let uri = params.text_document.uri.clone().to_string();

        if !self.document_map.contains_key(&uri) {
            self.client
                .log_message(
                    MessageType::ERROR,
                    format!("Document {} changed before opening!", uri),
                )
                .await;
            return;
        }

        //TODO: Figure out if we are running a semantic analysis if so, cancel that semantic analysis
        info!("Document change incoming for document: {}\nWith the following changes: {:?}", uri, params.content_changes.clone());
        
        let mut document = self.document_map.get(&uri).unwrap().clone();

        info!("Got document reference");
        
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_clingo::language()).expect("Error loading clingo grammar");

        document.update_document(params.content_changes, &mut parser);
        let doc = document.clone();

        self.document_map.insert(uri, document);

        let time = Instant::now();
        let diagnostics = run_diagnostics(
            doc,
            100,
        );
        client_copy.publish_diagnostics(
            params.text_document.uri.clone(),
            diagnostics,
            Some(1),
        ).await;
        let duration = time.elapsed();
        info!("Time needed for diagnostics: {:?}", duration);
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

        let completions = || -> Option<Vec<CompletionItem>> {
            let document = self.document_map.get(&uri.to_string())?;

            if let Some(context) = params.context {
                let mut trigger_character = "".to_string();
                if let Some(trigger) = context.trigger_character.clone() {
                    trigger_character = trigger;
                }

                return check_completion(document.value(), context, trigger_character, position);
            }

            //TODO: Keep track if analysis has been done yet
            Some(vec![])
        }();
        Ok(completions.map(CompletionResponse::Array))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        if let Some(document) = self.document_map.get(&uri.to_string()) {
            return Ok(Some(GotoDefinitionResponse::Array(check_goto_definition(document.value(), position).unwrap())));
        }        
        
        Result::Err(tower_lsp::jsonrpc::Error::new(tower_lsp::jsonrpc::ErrorCode::InternalError))
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        if let Some(document) = self.document_map.get(&uri.to_string()) {
            return Ok(check_goto_references(document.value(), position));
        }

        Result::Err(tower_lsp::jsonrpc::Error::new(tower_lsp::jsonrpc::ErrorCode::InternalError))
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

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client: client.clone(),
        document_map:DashMap::new()})
    .finish();
    Server::new(stdin, stdout, socket).serve(service).await;
}

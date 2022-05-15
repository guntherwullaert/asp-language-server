use std::collections::{HashMap, HashSet};

use dashmap::DashMap;
use ropey::Rope;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tower_lsp::jsonrpc::{ErrorCode, Result};
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use tree_sitter::{Parser, Language};

extern "C" { fn tree_sitter_clingo() -> Language; }

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, tree_sitter::Tree>,
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
            .log_message(MessageType::INFO, format!("file {} opened!", params.text_document.uri))
            .await;
        
        self.on_change(&params.text_document.uri.to_string(), &params.text_document.text).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        if (!self.document_map.contains_key(&uri)){
            self.client.log_message(MessageType::ERROR, format!("Document {} changed before opening!", uri))
            .await;
            return;
        }

        self.client.log_message(MessageType::LOG, format!("Document change incomming for document: {}\n", uri))
        .await;

        for change in params.content_changes{
            let range;
            match change.range {
                None => {
                        self.client.log_message(MessageType::ERROR, format!("Document change event for {} without range provided!", uri))
                        .await;
                        continue;
                    }
                Some(r) => range = r
            }

            self.client.log_message(MessageType::LOG, format!("Start: (c:{}, l:{})\nEnd: (c:{}, l:{})\nChange: {}\n", range.start.character, range.start.line, range.end.character, range.end.line, change.text))
            .await;

            self.on_change(&uri, &change.text).await;
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
    
    async fn on_change(&self, uri: &String, document: &String) {
        // Create a Parser for this document
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_clingo::language())
            .expect("Error loading clingo grammar");
        
        // Parse the document and save the parse tree in a hashmap
        let tree = parser.parse(document, None).unwrap();
        self.document_map.insert(uri.clone(), tree);
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
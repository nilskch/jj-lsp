use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tower_lsp_server::{Client, LanguageServer};

use crate::conflict::Analyzer;

pub struct Backend {
    client: Client,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut analyzer = Analyzer::new(&params.text_document.text);
        let _conflicts = analyzer.find_conflicts();
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let Some(content) = params.content_changes.first() else {
            self.client
                .log_message(
                    MessageType::ERROR,
                    "LSP client is supposed to always send the complete file contents.",
                )
                .await;
            return;
        };
        let mut analyzer = Analyzer::new(&content.text);
        let _conflicts = analyzer.find_conflicts();
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

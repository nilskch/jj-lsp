use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tower_lsp_server::{Client, LanguageServer};

pub struct Backend {
    _client: Client,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self { _client: client }
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

    async fn did_open(&self, _params: DidOpenTextDocumentParams) {}

    async fn did_change(&self, _params: DidChangeTextDocumentParams) {}

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

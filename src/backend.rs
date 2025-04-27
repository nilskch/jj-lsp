use std::ops::Deref;
use std::sync::Arc;

use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tower_lsp_server::{Client, LanguageServer};

use crate::conflict::Analyzer;
use crate::types::Conflict;

pub struct Backend {
    inner: Arc<BackendInner>,
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct BackendInner {
    client: Client,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            inner: Arc::new(BackendInner { client }),
        }
    }

    pub fn get_diagnostics_from_conflicts(&self, conflicts: &[Conflict]) -> Vec<Diagnostic> {
        conflicts
            .iter()
            .flat_map(|conflict| {
                let header_diagnostic = Diagnostic {
                    message: "Found conflicting changes".to_string(),
                    range: conflict.title_range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    ..Default::default()
                };

                let block_diagnostics = conflict.blocks.iter().map(|conflict_block| Diagnostic {
                    message: "Conflicting change".to_string(),
                    range: conflict_block.title_range,
                    severity: Some(DiagnosticSeverity::WARNING),
                    ..Default::default()
                });

                block_diagnostics.chain(std::iter::once(header_diagnostic))
            })
            .collect()
    }
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: Some("jj".to_string()),
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        work_done_progress_options: Default::default(),
                    },
                )),
                ..Default::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut analyzer = Analyzer::new(&params.text_document.text);
        let conflicts = analyzer.find_conflicts();
        let diagnostics = self.get_diagnostics_from_conflicts(conflicts);

        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await
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
        let conflicts = analyzer.find_conflicts();
        let diagnostics = self.get_diagnostics_from_conflicts(conflicts);

        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

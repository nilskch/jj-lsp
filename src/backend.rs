use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use tokio::sync::Mutex;
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
    diagnostics_and_code_actions: Mutex<HashMap<Uri, Vec<(Diagnostic, CodeActionResponse)>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            inner: Arc::new(BackendInner {
                client,
                diagnostics_and_code_actions: Default::default(),
            }),
        }
    }

    pub fn get_diagnostics_and_code_actions_from_conflicts(
        &self,
        conflicts: &[Conflict],
        uri: &Uri,
    ) -> Vec<(Diagnostic, CodeActionResponse)> {
        conflicts
            .iter()
            .flat_map(|conflict| {
                let header_diagnostic = Diagnostic {
                    message: "Found conflicting changes".to_string(),
                    range: conflict.title_range,
                    severity: Some(DiagnosticSeverity::ERROR),
                    ..Default::default()
                };

                let block_diagnostics_and_code_actions =
                    conflict
                        .blocks
                        .iter()
                        .enumerate()
                        .map(|(idx, conflict_block)| {
                            let diagnostic = Diagnostic {
                                message: "Conflicting change".to_string(),
                                range: conflict_block.title_range,
                                severity: Some(DiagnosticSeverity::WARNING),
                                ..Default::default()
                            };

                            let text_edit = TextEdit {
                                new_text: conflict_block.content.clone(),
                                range: conflict.range,
                            };

                            let workspace_edit = WorkspaceEdit {
                                changes: Some([(uri.clone(), vec![text_edit])].into()),
                                ..Default::default()
                            };

                            let code_actions = vec![CodeActionOrCommand::CodeAction(CodeAction {
                                title: format!("Accept change #{}", idx + 1),
                                diagnostics: Some(vec![diagnostic.clone()]),
                                edit: Some(workspace_edit),
                                ..Default::default()
                            })];

                            (diagnostic, code_actions)
                        });

                let header_code_actions = block_diagnostics_and_code_actions
                    .clone()
                    .flat_map(|(_, code_actions)| code_actions)
                    .collect::<Vec<_>>();

                block_diagnostics_and_code_actions
                    .chain(std::iter::once((header_diagnostic, header_code_actions)))
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
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let mut analyzer = Analyzer::new(&params.text_document.text);
        let conflicts = analyzer.find_conflicts();
        let diagnostics_and_code_actions = self
            .get_diagnostics_and_code_actions_from_conflicts(conflicts, &params.text_document.uri);

        let diagnostics = diagnostics_and_code_actions
            .iter()
            .map(|(diagnostic, _)| diagnostic.clone())
            .collect::<Vec<_>>();

        self.diagnostics_and_code_actions.lock().await.insert(
            params.text_document.uri.clone(),
            diagnostics_and_code_actions,
        );

        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let Some(content) = params.content_changes.first() else {
            // panic here, because the LSP won't work if the LSP client implements this incorrectly
            panic!("LSP client is supposed to always send the complete file contents.");
        };

        let mut analyzer = Analyzer::new(&content.text);
        let conflicts = analyzer.find_conflicts();
        let diagnostics_and_code_actions = self
            .get_diagnostics_and_code_actions_from_conflicts(conflicts, &params.text_document.uri);

        let diagnostics = diagnostics_and_code_actions
            .iter()
            .map(|(diagnostic, _)| diagnostic.clone())
            .collect::<Vec<_>>();

        let mut diagnostics_map = self.diagnostics_and_code_actions.lock().await;
        let uri_clone = params.text_document.uri.clone();

        if diagnostics_map.get(&uri_clone) != Some(&diagnostics_and_code_actions) {
            diagnostics_map.insert(uri_clone, diagnostics_and_code_actions);

            self.client
                .publish_diagnostics(params.text_document.uri, diagnostics, None)
                .await
        }
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let Some(diagnostics_and_code_actions) = self
            .diagnostics_and_code_actions
            .lock()
            .await
            .get(&params.text_document.uri)
            .cloned()
        else {
            return Ok(None);
        };

        let code_actions = diagnostics_and_code_actions
            .into_iter()
            .filter_map(|(diagnostic, code_actions)| {
                // NOTE: there are only very few diagnostics per file, so it's ok to iterate here
                // instead of using a hash map; hashing will be more expensive than iterating in
                // most cases
                if params.context.diagnostics.iter().any(|x| x == &diagnostic) {
                    Some(code_actions)
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        Ok(Some(code_actions))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

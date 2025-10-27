use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    client: Client,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        self.client
            .log_message(MessageType::INFO, "Initializing SurrealDB Language Server")
            .await;
        self.client
            .log_message(MessageType::INFO, "Version: 0.0.1")
            .await;
        self.client
            .log_message(MessageType::INFO, "Author: Manuel Sanchez")
            .await;
        self.client
            .log_message(MessageType::INFO, "Email: manuel@msanchezdev.com")
            .await;
        self.client
            .log_message(MessageType::INFO, "Website: https://msanchez.dev")
            .await;
        self.client
            .log_message(MessageType::INFO, "License: MIT")
            .await;
        self.client
            .log_message(MessageType::INFO, "Copyright: 2025 Manuel Sanchez")
            .await;
        self.client
            .log_message(MessageType::INFO, "GitHub: https://github.com/msanchezdev")
            .await;
        self.client
            .log_message(
                MessageType::INFO,
                "LinkedIn: https://linkedin.com/in/msanchezdev",
            )
            .await;
        self.client
            .log_message(
                MessageType::INFO,
                "================================================",
            )
            .await;
        self.client
            .log_message(
                MessageType::INFO,
                format!("Initialize params:\n{:#?}", params),
            )
            .await;

        let result = InitializeResult {
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions {
                    completion_item: Some(CompletionOptionsCompletionItem::default()),
                    ..Default::default()
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        };
        self.client
            .log_message(
                MessageType::INFO,
                format!("Initialize result:\n{:#?}", result),
            )
            .await;
        Ok(result)
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(
                MessageType::INFO,
                "SurrealDB Language Server initialized".to_string(),
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "Text document opened".to_string())
            .await;
        self.client
            .log_message(
                MessageType::LOG,
                format!("Text document:\n{:#?}", params.text_document),
            )
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "Text document changed".to_string())
            .await;
        self.client
            .log_message(MessageType::LOG, format!("Change params:\n{:#?}", params))
            .await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "Text document closed".to_string())
            .await;
        self.client
            .log_message(
                MessageType::LOG,
                format!("Text document:\n{:#?}", params.text_document),
            )
            .await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello 1".to_string(), "Hello World 1!".to_string()),
            CompletionItem::new_simple("Hello 2".to_string(), "Hello World 2!".to_string()),
            CompletionItem::new_simple("Hello 3".to_string(), "Hello World 3!".to_string()),
        ])))
    }
}

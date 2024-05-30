use std::collections::HashMap;

use super::lsp::{Position, ServerCapabilities, ServerInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub id: Option<i32>,
    pub result: LspResult,
}

pub struct InitializeResponse;
impl InitializeResponse {
    pub fn new(id: Option<i32>) -> Response {
        return Response {
            rpc: "2.0".to_string(),
            id,
            result: LspResult::InitializeResult {
                capabilities: ServerCapabilities {
                    text_document_sync: 1,
                    hover_provider: true,
                    definition_provider: true,
                    completion_provider: HashMap::new(),
                },
                server_info: ServerInfo {
                    name: "GDScript_LSP".to_string(),
                    version: "0.0.1".to_string(),
                },
            },
        };
    }
}

pub struct HoverResponse {}
impl HoverResponse {
    pub fn new(id: Option<i32>, contents: String) -> Response {
        return Response {
            rpc: "2.0".to_string(),
            id,
            result: LspResult::HoverResult { contents },
        };
    }
}

pub struct CompletionResponse {}
impl CompletionResponse {
    pub fn new(id: Option<i32>, items: Vec<CompletionItem>) -> Response {
        return Response {
            rpc: "2.0".to_string(),
            id,
            result: LspResult::CompletionResult { items },
        };
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompletionItem {
    pub label: Option<String>,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

pub struct DefinitionResponse {}
impl DefinitionResponse {
    pub fn new(id: Option<i32>, location: Location) -> Response {
        return Response {
            rpc: "2.0".to_string(),
            id,
            result: LspResult::DefinitionResult {
                uri: location.uri,
                range: location.range,
            },
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum LspResult {
    #[serde(rename_all = "camelCase")]
    InitializeResult {
        capabilities: ServerCapabilities,
        server_info: ServerInfo,
    },
    HoverResult {
        contents: String,
    },
    CompletionResult {
        items: Vec<CompletionItem>,
    },
    DefinitionResult {
        uri: String,
        range: Range,
    },
}

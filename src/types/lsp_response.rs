use super::lsp::{ServerCapabilities, ServerInfo};
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    pub server_info: ServerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoverResult {
    contents: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum LspResult {
    HoverResult {
        contents: String,
    },
    #[serde(rename_all = "camelCase")]
    InitializeResult {
        capabilities: ServerCapabilities,
        server_info: ServerInfo,
    },
}

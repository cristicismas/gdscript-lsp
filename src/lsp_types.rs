use serde::{Deserialize, Serialize};

use crate::text_document::{TextDocumentContentChangeEvent, TextDocumentItem};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Message {
    pub jsonrpc: String,
    pub id: Option<i32>,
    pub method: String,
    pub params: Option<MessageParams>,
    #[serde(flatten)]
    pub other_fields: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageParams {
    // all
    pub text_document: Option<TextDocumentItem>,
    // didChange
    pub content_changes: Option<Vec<TextDocumentContentChangeEvent>>,
    // hover
    pub position: Option<Position>,
}

pub struct HoverResponse {}

impl HoverResponse {
    pub fn new(id: Option<i32>, contents: String) -> Response {
        return Response {
            rpc: "2.0".to_string(),
            id,
            result: LspResult {
                capabilities: None,
                server_info: None,
                contents: Some(contents),
            },
        };
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub id: Option<i32>,
    pub method: String,
}

pub struct InitializeResponse {}

impl InitializeResponse {
    pub fn new(id: Option<i32>) -> Response {
        return Response {
            rpc: "2.0".to_string(),
            id,
            result: LspResult {
                capabilities: Some(ServerCapabilities {
                    text_document_sync: 1,
                    hover_provider: true,
                }),
                server_info: Some(ServerInfo {
                    name: "GDScript_LSP".to_string(),
                    version: "0.0.1".to_string(),
                }),
                contents: None,
            },
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub id: Option<i32>,
    pub result: LspResult,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LspResult {
    pub capabilities: Option<ServerCapabilities>,
    pub server_info: Option<ServerInfo>,
    pub contents: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub method: String,
    pub params: Option<NotificationParams>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationParams {
    pub text_document: Option<TextDocumentItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    pub text_document_sync: i32,
    pub hover_provider: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

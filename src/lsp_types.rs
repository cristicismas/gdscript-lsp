use serde::{Deserialize, Serialize};

use crate::text_document::TextDocumentItem;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Message {
    pub jsonrpc: String,
    pub id: Option<i32>,
    pub method: String,
    pub params: Option<MessageParams>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct MessageParams {
    #[serde(rename = "textDocument")]
    pub text_document: Option<TextDocumentItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub id: Option<i32>,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeResponse {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub id: Option<i32>,
    pub result: InitializeResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "jsonrpc")]
    pub rpc: String,
    pub method: String,
    pub params: Option<NotificationParams>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NotificationParams {
    #[serde(rename = "textDocument")]
    pub text_document: Option<TextDocumentItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(rename = "textDocumentSync")]
    pub text_document_sync: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

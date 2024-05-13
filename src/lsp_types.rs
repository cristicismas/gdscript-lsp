use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Message {
    pub jsonrpc: String,
    pub id: Option<i32>,
    pub method: String,
    pub params: Option<MessageParams>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct MessageParams {}

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

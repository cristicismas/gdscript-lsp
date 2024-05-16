use serde::{Deserialize, Serialize};

use crate::types::text_document::{TextDocumentContentChangeEvent, TextDocumentItem};

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

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Position {
    pub line: u32,
    pub character: u32,
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
    pub uri: String,
    pub language_id: Option<String>,
    pub version: Option<i32>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextDocumentIdentifier {
    pub uri: String,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextDocumentContentChangeEvent {
    pub text: String,
}

use std::collections::HashMap;

use crate::{
    logger::{self, print_error},
    types::{
        lsp::Position,
        lsp_response::{
            CompletionItem, CompletionResponse, DefinitionResponse, HoverResponse, Location, Range,
            Response,
        },
    },
};

use super::completion::get_completion_items;

pub struct State {
    pub documents: HashMap<String, String>,
}

impl State {
    pub fn new() -> Self {
        return State {
            documents: HashMap::new(),
        };
    }

    pub fn open_document(&mut self, uri: &str, text: &str) -> () {
        self.documents.insert(uri.to_string(), text.to_string());
    }

    pub fn update_document(&mut self, uri: &str, change: &str) -> () {
        let current_document_text = self
            .documents
            .get_mut(uri)
            .expect("Cannot get uri on State documents.");

        *current_document_text = change.to_string();
    }

    pub fn hover(&mut self, id: i32, uri: &str, _position: Position) -> Response {
        let document = get_document_contents(&self.documents, uri);

        let contents = format!("File: {}, characters: {}", uri, document.len());

        let response = HoverResponse::new(Some(id), contents);

        return response;
    }

    pub fn completion(&mut self, id: i32, uri: &str) -> Response {
        let file_contents = self
            .documents
            .get(uri)
            .expect("Expected to find corresponding key in the documents HashMap");

        let items: Vec<CompletionItem> = get_completion_items(file_contents);
        let response = CompletionResponse::new(Some(id), items);

        return response;
    }

    pub fn definition(&mut self, id: i32, uri: &str, position: Position) -> Response {
        let line_position = if position.line == 0 {
            0
        } else {
            position.line - 1
        };

        let response = DefinitionResponse::new(
            Some(id),
            Location {
                uri: uri.to_string(),
                range: Range {
                    start: Position {
                        line: line_position,
                        character: 0,
                    },
                    end: Position {
                        line: line_position,
                        character: 1,
                    },
                },
            },
        );

        return response;
    }
}

fn get_document_contents<'a>(documents: &'a HashMap<String, String>, uri: &str) -> &'a String {
    let document = match documents.get(uri) {
        Some(v) => v,
        None => print_error("The uri should already be added to the documents map.".to_string()),
    };

    return document;
}

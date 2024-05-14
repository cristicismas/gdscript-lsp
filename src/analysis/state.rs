use std::collections::HashMap;

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
}

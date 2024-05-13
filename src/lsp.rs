use super::logger;
use super::lsp_types::{
    InitializeResponse, InitializeResult, Message, ServerCapabilities, ServerInfo,
};
use super::rpc;
use super::writer;
use crate::unwrap_or_return;

fn new_initialize_response(id: Option<i32>) -> InitializeResponse {
    return InitializeResponse {
        rpc: "2.0".to_string(),
        id,
        result: InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: 1,
            },
            server_info: ServerInfo {
                name: "GDScript_LSP".to_string(),
                version: "0.0.1".to_string(),
            },
        },
    };
}

pub fn handle_message(message: Message) {
    match message.method.as_str() {
        "initialize" => {
            let initialize_response = new_initialize_response(message.id);

            let encoded_message = rpc::encode(initialize_response);

            writer::write_stdout(encoded_message.as_bytes());
        }
        "initialized" => {}
        "textDocument/didOpen" => {
            let params = unwrap_or_return!(message.params);
            let text_document = unwrap_or_return!(params.text_document);

            logger::print_logs(format!(
                "File opened with text_document: {:?}",
                text_document
            ));
        }
        "shutdown" => {
            logger::print_logs("Quitting program...".to_string());
            std::process::exit(0);
        }
        method => {
            let error_message = format!("Cannot find method from message: {}", method);

            logger::print_logs(error_message);
        }
    }
}

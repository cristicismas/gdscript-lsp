use super::logger;
use super::lsp_types::{
    InitializeResponse, InitializeResult, Message, Response, ServerCapabilities, ServerInfo,
};
use super::rpc;
use super::writer;

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

            logger::print_logs("Sent an initalize response...".to_string());
        }
        "initialized" => {}
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

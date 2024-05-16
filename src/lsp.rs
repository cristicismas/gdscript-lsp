use crate::analysis::state::State;
use crate::logger;
use crate::rpc;
use crate::types::lsp::Message;
use crate::types::lsp_response::{HoverResponse, InitializeResponse};
use crate::unwrap_or_return;
use crate::writer;

pub fn handle_message(message: Message, state: &mut State) {
    match message.method.as_str() {
        "initialize" => {
            let initialize_response = InitializeResponse::new(message.id);

            let encoded_message = rpc::encode(initialize_response);

            writer::write_stdout(encoded_message.as_bytes());
        }
        "initialized" => {}
        "textDocument/didOpen" => {
            let params = unwrap_or_return!(message.params);
            let text_document = unwrap_or_return!(params.text_document);
            let text = unwrap_or_return!(text_document.text);

            state.open_document(&text_document.uri, &text);
        }
        "textDocument/didChange" => {
            let params = unwrap_or_return!(message.params);
            let text_document = unwrap_or_return!(params.text_document);
            let content_changes = unwrap_or_return!(params.content_changes);

            for index in 0..content_changes.len() {
                let change = &content_changes[index].text;
                logger::print_logs(format!("Content changed in: {:?}", &text_document.uri));
                state.update_document(&text_document.uri, &change);
            }
        }
        "textDocument/didSave" => {}
        "textDocument/hover" => {
            let hover_response = HoverResponse::new(message.id, "hello world".to_string());
            let encoded_message = rpc::encode(hover_response);

            logger::print_logs(format!("encoded message: {:?}", encoded_message));
            writer::write_stdout(encoded_message.as_bytes());
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

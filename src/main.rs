mod logger;
mod lsp;
mod lsp_types;
mod reader;
mod rpc;
mod writer;

use lsp_types::Message;

fn main() {
    logger::clear_logs(None);
    logger::print_logs("Starting up...".to_string());

    loop {
        let input: Vec<u8> = reader::read_stdin();
        let message: Message = rpc::decode(&input);

        logger::print_logs(format!(
            "Received message with method: {:?} and id: {:?}\n",
            message.method, message.id
        ));

        lsp::handle_message(message);
    }
}

mod logger;
mod lsp;
mod reader;
mod rpc;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct Message {
    jsonrpc: String,
    id: i32,
    method: String,
    params: Option<MessageParams>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
struct MessageParams {}

fn main() {
    logger::clear_logs(None).unwrap();

    let input: Vec<u8> = reader::read_stdin();
    let message: Message = rpc::decode(&input);

    logger::print_logs(
        format!("Received message with method: {:?}\n", message.method),
        None,
    )
    .unwrap();
}

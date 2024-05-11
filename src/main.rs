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

    logger::print_logs(String::from("starting lsp up..."), None).unwrap();

    let input = reader::read_stdin();
    let input_string = String::from(std::str::from_utf8(&input).unwrap());

    logger::print_logs(format!("input: {}\n", input_string), None).unwrap();
}

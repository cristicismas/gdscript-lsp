mod logger;
mod lsp;
mod rpc;

use rpc::get_content_length;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Read};

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

    let encoded: String = rpc::encode(Message {
        jsonrpc: String::from("2.0"),
        id: 1,
        method: String::from("textDocument/completion"),
        params: None,
    });

    logger::print_logs(String::from("starting up..."), None).unwrap();

    let mut first_line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut first_line).unwrap();

    let content_length: i32 = rpc::get_content_length(first_line);

    logger::print_logs(format!("content length: {}\n", content_length), None).unwrap();

    let mut current_bytes: Vec<u8> = Vec::new();

    for i in io::stdin().bytes() {
        let current_byte = match i {
            Ok(v) => v,
            Err(_) => continue,
        };

        current_bytes.push(current_byte);

        if current_bytes.len() == content_length as usize {
            let bytes_as_str = String::from(std::str::from_utf8(&current_bytes).unwrap());

            logger::print_logs(format!("current_bytes: {}", bytes_as_str), None).unwrap();
        }
    }
}

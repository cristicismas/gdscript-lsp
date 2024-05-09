mod logger;
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
    let encoded: String = rpc::encode(Message {
        jsonrpc: String::from("2.0"),
        id: 1,
        method: String::from("textDocument/completion"),
        params: None,
    });

    logger::print_logs(format!("{}", encoded), None).unwrap();
}

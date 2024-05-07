mod rpc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Message {
    jsonrpc: String,
    id: i32,
    method: String,
    params: Option<MessageParams>,
}

#[derive(Deserialize, Serialize)]
struct MessageParams {}

fn main() {
    let encoded: String = rpc::encode(Message {
        jsonrpc: String::from("2.0"),
        id: 1,
        method: String::from("textDocument/completion"),
        params: None,
    });
    println!("{}", encoded)
}

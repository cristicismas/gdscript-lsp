use super::logger;
use super::rpc;
use std::io::{self, Read};

// Read the first line from stdin, and then read the next 'n' bytes
// based on the "Content-Length" input from the first byte
pub fn read_stdin() -> Vec<u8> {
    let mut first_line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut first_line).unwrap();

    let content_length: i32 = rpc::get_content_length(first_line);

    let mut current_bytes: Vec<u8> = Vec::new();

    for i in io::stdin().bytes() {
        let current_byte = match i {
            Ok(v) => v,
            Err(_) => continue,
        };

        current_bytes.push(current_byte);

        if current_bytes.len() == content_length as usize {
            break;
        }
    }

    return current_bytes;
}

use super::logger;
use super::rpc;
use std::io::{self, Read};

// Read the first line from stdin, and then read the next 'n' bytes
// based on the "Content-Length" input from the first byte
pub fn read_stdin() -> Vec<u8> {
    let content_length_bytes: Vec<u8> = read_bytes_until_char('\n');

    let content_length_line: &str = match std::str::from_utf8(&content_length_bytes) {
        Ok(v) => v,
        Err(e) => {
            let error_message =
                format!("Cannot convert content_length_bytes to utf8 string: {}", e);

            logger::print_error(error_message.clone());
        }
    };

    let content_length: i32 = rpc::get_content_length(content_length_line.to_string());

    let current_bytes: Vec<u8> = read_bytes_until_length(content_length as usize);

    return current_bytes;
}

fn read_bytes_until_length(length: usize) -> Vec<u8> {
    let mut current_bytes: Vec<u8> = Vec::with_capacity(length);

    for i in io::stdin().bytes() {
        let current_byte = match i {
            Ok(v) => v,
            Err(_) => continue,
        };

        current_bytes.push(current_byte);

        if current_bytes.len() == length {
            break;
        }
    }

    return current_bytes;
}

fn read_bytes_until_char(character: char) -> Vec<u8> {
    let mut current_bytes: Vec<u8> = Vec::new();

    while current_bytes.len() == 0 {
        for i in io::stdin().bytes() {
            let current_byte = match i {
                Ok(v) => v,
                Err(_) => continue,
            };

            current_bytes.push(current_byte);

            if current_byte as char == character {
                break;
            }
        }
    }

    return current_bytes;
}

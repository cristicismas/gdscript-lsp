use super::logger;
use super::Message;
use core::panic;
use serde_json::json;

#[derive(Debug)]
struct DecodeError;

pub fn encode(msg: Message) -> String {
    let json_encoded_msg: String = json!(msg).to_string();
    let msg_length = json_encoded_msg.len();

    let encoded_msg_with_length: String =
        format!("Content-Length: {}\r\n\r\n{}", msg_length, json_encoded_msg);

    return encoded_msg_with_length;
}

pub fn decode(msg: &[u8]) -> Message {
    let string_msg: String = match std::str::from_utf8(msg) {
        Ok(str) => str.to_string(),
        Err(e) => panic!("Invalid utf8 sequence: {}", e),
    };

    let message: Message = match serde_json::from_str(&string_msg.trim()) {
        Ok(msg) => msg,
        Err(err) => panic!("Cannot parse message contents to Message type: {}", err),
    };

    return message;
}

pub fn get_content_length(line: String) -> i32 {
    // This is the second \r\n after Content-Length
    // we need to add this to the length.
    let separator = "\r\n";

    if !line.starts_with("Content-Length") {
        logger::print_logs(format!("line doesn't start with Content-Length."), None).unwrap();
        panic!("line_parts needs to have length == 2.");
    }

    let line_parts: Vec<&str> = line.split(" ").collect();

    if line_parts.len() != 2 {
        logger::print_logs(format!("line_parts needs to have length == 2."), None).unwrap();
        panic!("line_parts needs to have length == 2.");
    }

    let length_str: &str = line_parts[1].trim();

    let length: i32 = match length_str.parse::<usize>() {
        Ok(v) => {
            let total_length: i32 = v as i32 + separator.len() as i32;
            total_length
        }
        Err(err) => {
            logger::print_logs(
                format!("cannot parse content length to i32: {}\n", err),
                None,
            )
            .unwrap();

            return 0;
        }
    };

    return length;
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_ENCODED_STRING: &str = "Content-Length: 73\r\n\r\n{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"textDocument/completion\",\"params\":null}";

    #[test]
    fn encode_test() {
        let expected: String = String::from(TEST_ENCODED_STRING);

        let actual: String = encode(Message {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from("textDocument/completion"),
            params: None,
        });

        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_test() {
        let expected = Message {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from("textDocument/completion"),
            params: None,
        };

        let actual = decode(TEST_ENCODED_STRING.as_bytes());

        assert_eq!(expected, actual);
    }
}

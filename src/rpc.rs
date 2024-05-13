use super::logger;
use super::Message;
use serde_json::json;

#[derive(Debug)]
struct DecodeError;

pub fn encode<T: serde::Serialize>(msg: T) -> String {
    let json_encoded_msg: String = json!(msg).to_string();
    let msg_length: usize = json_encoded_msg.len();

    let encoded_msg_with_length: String =
        format!("Content-Length: {}\r\n\r\n{}", msg_length, json_encoded_msg);

    return encoded_msg_with_length;
}

pub fn decode(msg: &[u8]) -> Message {
    let string_msg: String = match std::str::from_utf8(msg) {
        Ok(str) => str.to_string(),
        Err(e) => {
            let error = format!("Invalid utf8 sequence: {}", e);
            logger::print_error(error);
        }
    };

    let message: Message = match serde_json::from_str(&string_msg.trim()) {
        Ok(msg) => msg,
        Err(e) => {
            let error = format!("Cannot parse message contents to Message type: {}", e);
            logger::print_error(error);
        }
    };

    return message;
}

pub fn get_content_length(line: String) -> i32 {
    // This is the second \r\n after Content-Length
    // we need to add this to the length.
    let separator = "\r\n";

    if !line.starts_with("Content-Length") {
        logger::print_logs(format!("line: {}", line));
        logger::print_error(format!("line doesn't start with Content-Length: {}", line));
    }

    let line_parts: Vec<&str> = line.split(" ").collect();

    if line_parts.len() != 2 {
        logger::print_error(format!("line_parts needs to have length == 2."));
    }

    let length_str: &str = line_parts[1].trim();

    let length: i32 = match length_str.parse::<i32>() {
        Ok(v) => v,
        Err(err) => {
            logger::print_error(format!("cannot parse content length to i32: {}\n", err));
        }
    };

    let total_length = length + separator.len() as i32;

    return total_length;
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_ENCODED_STRING: &str = "Content-Length: 73\r\n\r\n{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"textDocument/completion\",\"params\":null}";

    const TEST_DECODED_STRING: &str =
        "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"textDocument/completion\",\"params\":null}";

    #[test]
    fn encode_test() {
        let expected: String = String::from(TEST_ENCODED_STRING);

        let actual: String = encode(Message {
            jsonrpc: String::from("2.0"),
            id: Some(1),
            method: String::from("textDocument/completion"),
            params: None,
        });

        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_test() {
        let expected = Message {
            jsonrpc: String::from("2.0"),
            id: Some(1),
            method: String::from("textDocument/completion"),
            params: None,
        };

        let actual = decode(TEST_DECODED_STRING.as_bytes());

        assert_eq!(expected, actual);
    }
}

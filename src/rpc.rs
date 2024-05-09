use super::Message;
use serde_json::json;
use std::convert::TryFrom;

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

    let split_msg: Vec<&str> = string_msg.split("\r\n\r\n").collect();

    let [_content_length, content] = <[&str; 2]>::try_from(split_msg).ok().unwrap();

    let message: Message = match serde_json::from_str(content) {
        Ok(msg) => msg,
        Err(err) => panic!("Cannot parse message contents to Message type: {}", err),
    };

    return message;
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

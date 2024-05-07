use super::Message;
use serde_json::json;

struct DecodeError;

pub fn encode(msg: Message) -> String {
    let json_encoded_msg: String = json!(msg).to_string();
    let msg_length = json_encoded_msg.len();

    let encoded_msg_with_length: String =
        format!("Content-Length: {}\r\n\r\n{}", msg_length, json_encoded_msg);

    return encoded_msg_with_length;
}

pub fn decode(msg: Vec<u8>) -> Result<Message, DecodeError> {
    return Err(DecodeError);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_test() {
        let expected_result: String =String::from("Content-Length: 73\r\n\r\n{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"textDocument/completion\",\"params\":null}");

        let actual_result: String = encode(Message {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from("textDocument/completion"),
            params: None,
        });

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn decode_test() {}
}

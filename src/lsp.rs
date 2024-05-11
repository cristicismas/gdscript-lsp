struct Request {
    rpc: String,
    id: i32,
    method: String,
}

struct Response {
    rpc: String,
    id: Option<i32>,
}

struct Notification {
    rpc: String,
    method: String,
}

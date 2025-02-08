mod analysis;
mod helpers;
mod lexer;
mod logger;
mod lsp;
mod reader;
mod rpc;
mod types;
mod writer;

use analysis::state::State;
use types::lsp::Message;

pub mod macros {
    #[macro_export(local_inner_macros)]
    macro_rules! unwrap_or_return {
        ($e:expr) => {
            match $e {
                Some(v) => v,
                None => return,
            }
        };
    }
}

fn main() {
    logger::clear_logs(None);
    logger::print_logs("Starting up...".to_string());

    let mut state = State::new();

    loop {
        let input: Vec<u8> = reader::read_stdin();
        let message: Message = rpc::decode(&input);

        logger::print_logs(format!(
            "Received message with method: {:?} and id: {:?}\n",
            message.method, message.id
        ));

        lsp::handle_message(message, &mut state);
    }
}

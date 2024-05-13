use std::io::{self, Write};

use super::logger;

pub fn write_stdout(message: &[u8]) {
    let mut stdout = io::stdout().lock();

    match stdout.write_all(message) {
        Ok(_) => (),
        Err(e) => logger::print_error(format!("Failed to write to stdout: {}", e)),
    };

    match stdout.flush() {
        Ok(_) => (),
        Err(e) => logger::print_error(format!("Failed to flush stdout: {}", e)),
    };
}

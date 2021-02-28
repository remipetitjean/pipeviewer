use std::env;
use std::io::{self, Read, Write};

// const CHUNK_SIZE: usize = 16 * 1024;
const CHUNK_SIZE: usize = 10;

fn main() {
    let silent = env::var("PV_SILENT").unwrap_or(String::new()).len() > 0;

    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        io::stdout().write_all(&buffer[..num_read]).unwrap();
        total_bytes += num_read;
    }
    if !silent {
        eprintln!("num_read: {}", total_bytes);
    };
}

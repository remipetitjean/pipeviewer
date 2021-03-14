#![allow(clippy::mutex_atomic)]

use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();

    let Args {
        infile,
        outfile,
        silent,
    } = args;

    let quit = Arc::new(Mutex::new(false));
    let (read_quit, stats_quit, write_quit) = (quit.clone(), quit.clone(), quit);

    let read_handle = thread::spawn(move || read::read_loop(&infile, read_quit));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_quit));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_quit));

    // crash if any threads have crashed
    // `.join()` returns a `thread::Result<io::Result<()>>`
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // Return an error if any of the threads returned an error
    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}

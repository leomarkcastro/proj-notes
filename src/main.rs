#![deny(clippy::all)]

use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn load_args() -> Vec<String> {
    env::args().collect()
}

fn check_args(args: &Vec<String>) -> bool {
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return false;
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = load_args();

    if !check_args(&args) {
        return Ok(());
    }

    let note = args[1].clone();

    let time_now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("notes.txt")
        .unwrap();

    file.write_all(b"<!--")?;
    file.write_all(time_now.as_bytes())?;
    file.write_all(b"-->\n\n")?;

    file.write_all(note.as_bytes())?;
    file.write_all(b"\n\n")?;

    Ok(())
}

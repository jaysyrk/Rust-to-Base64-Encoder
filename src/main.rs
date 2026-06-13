use rust_base64::{decode, encode};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: rust-base64 [encode|decode] <file_path>");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    let file_bytes = match fs::read(file_path) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("Whoops, could not find that file!");
            return;
        }
    };

    if command == "encode" {
        println!("{}", encode(&file_bytes));
    } else if command == "decode" {
        let text = String::from_utf8_lossy(&file_bytes);
        let decoded_bytes = decode(&text);

        let decoded_text =
            String::from_utf8(decoded_bytes).unwrap_or_else(|_| "Invalid UTF-8".to_string());
        println!("{}", decoded_text);
    } else {
        println!("Unknown command. Use 'encode' or 'decode'.");
    }
}

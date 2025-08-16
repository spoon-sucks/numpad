use std::{str::Chars, env::args, path::Path, fs::read_to_string};
use colored::Colorize;

fn send_error(message: &str) {
    println!("{} {}", "error:".bold().red(), message);
}

fn lexer(content: String) -> Vec<u8> {
    let mut tokens: Vec<u8> = Vec::new();

    let chars: Chars<'_> = content.chars();

    for c in chars {
        if c.is_numeric() {
            let token: u8 = c.to_digit(10).unwrap() as u8;
            tokens.push(token);
        }
    }

    tokens
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() >= 3 {
        send_error("found too many arguments in the command line.");
        ()
    }
    if args.len() <= 1 {
        send_error("found too little arguments in the command line.");
        ()
    }

    let file_path: &String = &args[1];
    let path: &Path = Path::new(file_path);

    if !path.exists() {
        send_error("file path does not exist.");
        ()
    }
    if path.extension().and_then(|ext| ext.to_str()) != Some("numpad") {
        send_error("file provided is not a numpad file (.numpad extension)");
        ()
    }

    let content: String = match read_to_string(path) {
        Ok(s) => s,
        Err(_e) => {
            send_error("failed to read file.");
            return
        }
    };

    let tokens: Vec<u8> = lexer(content);
    println!("{:?}", tokens);
}

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
        return
    }
    if args.len() <= 1 {
        send_error("found too little arguments in the command line.");
        return
    }

    let file_path: &String = &args[1];
    let path: &Path = Path::new(file_path);

    if !path.exists() {
        send_error("file path does not exist.");
        return
    }
    if path.extension().and_then(|ext| ext.to_str()) != Some("numpad") {
        send_error("file provided is not a numpad file (.numpad extension)");
        return
    }

    let content: String = match read_to_string(path) {
        Ok(s) => s,
        Err(_e) => {
            send_error("failed to read file.");
            return
        }
    };

    let mut stack: Vec<i64> = Vec::new();

    let tokens: Vec<u8> = lexer(content);
    let mut current_index: usize = 0;

    let mut marker: usize = current_index.clone();

    while current_index < tokens.len() {
        let token: u8 = tokens[current_index];
        match token {
            0 => stack.push(1),
            1 => match stack.last() {
                Some(val) => println!("{}", val),
                None => {
                    send_error("tried to print top value from empty stack.");
                    return
                },
            }
            2 => {
                let length: usize = stack.len();
                if length == 0 {
                    send_error("tried to pop top value from empty stack.");
                    return
                } else {
                    stack.pop();
                }
            }
            3 => {
                let length: usize = stack.len();
                if length < 2 {
                    send_error("tried to swap the top two values of stack with fewer elements.");
                    return
                }
                stack.swap(length-1, length-2);
            }
            4 => {
                let length: usize = stack.len();
                if length < 2 {
                    send_error("tried to add two top values of a stack with fewer elements");
                    return
                }

                stack.push(stack[length-2] + stack[length-1]);
            }
            5 => {
                let length: usize = stack.len();
                if length < 2 {
                    send_error("tried to subtract two top values of a stack with fewer elements");
                    return
                }

                stack.push(stack[length-2] - stack[length-1]);
            }
            6 => {
                if stack.len() == 0 {
                    stack.push(0);
                } else {
                    stack.push(1);
                }
            }
            7 => {
                let length: usize = stack.len();
                if length < 2 {
                    send_error("tried to compare two top values of a stack with fewer elements");
                    return
                }

                let compare: bool = stack[length-1] == stack[length-2];
                if compare {
                    stack.push(1);
                } else {
                    stack.push(0);
                }
            }
            8 => {
                let condition = match stack.last() {
                    Some(val) => val,
                    None => {
                        send_error("no condition found for if-goto, stack does not have enough elements.");
                        return
                    }
                };

                if condition != &0 {
                    current_index = marker.clone();
                }
            }
            9 => {
                marker = current_index.clone();
            }
            _ => {
                send_error("found invalid token during interpretation, what the fuck is this.");
                return
            }
        }
        current_index += 1;

        // println!("{:?}", stack)
    }
}

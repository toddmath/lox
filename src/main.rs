use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = &args[0];

    if args.len() < 3 {
        eprintln!("Usage: {program_name} tokenize <filename>");
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename)
                .inspect_err(|_| eprintln!("Failed to read file {filename}"))
                .unwrap_or_default();

            tokenize(file_contents);
        }
        _ => {
            eprintln!("Unknown command: {command}");
        }
    }
}

// (){};,+-*!===<=>=!=<>/.
fn tokenize(input: impl AsRef<str>) {
    for char in input.as_ref().chars() {
        match char {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '}' => println!("RIGHT_BRACE }} null"),
            '{' => println!("LEFT_BRACE {{ null"),
            ';' => println!("SEMICOLON ; null"),
            ',' => println!("COMMA , null"),
            '+' => println!("PLUS + null"),
            '-' => println!("MINUS - null"),
            '*' => println!("STAR * null"),
            '<' => println!("LESS < null"),
            '>' => println!("GREATER > null"),
            '/' => println!("SLASH / null"),
            '.' => println!("DOT . null"),
            _ => {}
        }
    }
    println!("EOF  null");
}

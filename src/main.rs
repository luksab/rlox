mod interpreter;

use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let tokens = interpreter::lex(&file_contents);
            match tokens {
                Ok(tokens) => {
                    for token in tokens {
                        println!("{}", token);
                    }
                }
                Err(()) => {
                    println!("Failed to tokenize input");
                    std::process::exit(65);
                }
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let expr = interpreter::parse_expr(&file_contents);
            match expr {
                Ok(expr) => {
                    println!("{}", expr);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(65);
                }
            }
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let result = interpreter::eval(&file_contents);
            match result {
                Ok(result) => {
                    println!("{}", result);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(70);
                }
            }
        }
        "run" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let result = interpreter::run(&file_contents);
            if let Err(err) = result {
                eprintln!("{}", err);
                let code = match err {
                    interpreter::InterpreterError::LexError => 65,
                    interpreter::InterpreterError::ParseError(_) => 65,
                    interpreter::InterpreterError::ResolverError(_) => 75,
                    interpreter::InterpreterError::ExecError(_) => 70,
                };
                std::process::exit(code);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

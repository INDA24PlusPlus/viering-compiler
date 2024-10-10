use std::{env, fs};

use ast::AstParser;
use lexer::Lexer;

pub mod ast;
pub mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: trunkpp <file_path>");
        return;
    }

    match fs::read_to_string(args[1].clone()) {
        Ok(contents) => {
            let mut lexer = Lexer::new(contents);
            let tokens = lexer.tokenize();

            match tokens {
                Ok(tokens) => {
                    let mut ast = AstParser::new(tokens.clone());
                    println!("{:?}", ast.parse());
                }
                Err(err) => {
                    println!("{}", err)
                }
            };
        }
        Err(_) => {
            println!("Invalid file path supplied");
        }
    }
}

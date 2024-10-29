use std::{env, fs};

use ast::AstParser;
use lexer::Lexer;
use transpiler::Transpiler;

pub mod ast;
pub mod lexer;
pub mod transpiler;

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
                    let mut ast_parser = AstParser::new(tokens.clone());
                    let ast = ast_parser.parse();

                    let transpiler = Transpiler::new();
                    println!("{}", transpiler.compile(ast));
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

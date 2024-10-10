use ast::AstParser;
use lexer::{Lexer, TokenType};

pub mod ast;
pub mod lexer;

fn main() {
    let example_program = r#"
        var balls = (balls + 4) * 2 + 4 * 5!
    "#;

    let mut lexer = Lexer::new(example_program.to_string());
    let tokens = lexer.tokenize();

    match tokens {
        Ok(tokens) => {
            let mut ast = AstParser::new(tokens.clone());
            println!("{:?}", ast.parse());

            for token in tokens.iter() {
                print!("{:?} ", token.token_type);

                if token.token_type == TokenType::Bang {
                    println!();
                }
            }
            println!();
        }
        Err(err) => {
            println!("{}", err)
        }
    };
}

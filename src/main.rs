use ast::AstParser;
use lexer::{Lexer, TokenType};

pub mod ast;
pub mod lexer;

fn main() {
    let example_program = r#"
        var balls = 0!
        var test_5 = 0!
        var balls = test_5 * 4 - 4 + 2!
        
        if balls == 10 {
            print(test_5)!
        }
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

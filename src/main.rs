use ast::AstParser;
use lexer::{Lexer, TokenType};

pub mod ast;
pub mod lexer;

fn main() {
    let example_program = r#"
        var a = 0!
        var b = 1!
        var n = 0!

        loop {
            if(n == 10){
                break!
            }

            var c = a + b!
            a = b!
            b = c!
            n = n + 1!
        }
    "#;

    let mut lexer = Lexer::new(example_program.to_string());
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

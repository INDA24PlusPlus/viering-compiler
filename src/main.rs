pub mod lexer;

fn main() {
    let example_program = r#"
        var test_5 = 0!
        var balls = test_5 * 4 - 4 + 2!
        
        if balls == 10 {
            print(test_5)!
        }
    "#;

    let mut lexer = lexer::Lexer::new(example_program.to_string());
    let tokens = lexer.tokenize();

    match tokens {
        Ok(tokens) => {
            for token in tokens.iter() {
                print!("{:?} ", token.token_type);

                if token.token_type == lexer::TokenType::Bang {
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

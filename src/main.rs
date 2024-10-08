#[derive(Debug, PartialEq)]
enum TokenType {
    // Simple
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    // Complex
    Equal,
    EqualEqual,
    Semicolon,
    SemicolonEqual,
    Identifier(String),
    Integer(i64),
    Bool(bool),
    String(String),

    // Keywords
    Var,
    If,
    Previous,
    Loop,
    Break,

    Eof,
    Invalid,
}

struct Token {
    token_type: TokenType,
    index: usize,
}

struct Lexer {
    code: String,
    chr_index: usize,
    token_index: usize,
}

impl Lexer {
    fn new(code: String) -> Self {
        Self {
            code,
            chr_index: 0,
            token_index: 0,
        }
    }

    fn next_chr(&mut self) -> Option<char> {
        let chr = self.code.chars().nth(self.chr_index);
        chr
    }

    fn consume_chr(&mut self) {
        self.chr_index += 1;
    }

    fn parse_token(&mut self) -> Option<Token> {
        let c = self.next_chr();
        self.consume_chr();

        let mut token = Token {
            token_type: TokenType::Invalid,
            index: self.token_index,
        };

        if let Some(c) = c {
            match c {
                ' ' => return None,
                '\n' => return None,
                '\t' => return None,
                '+' => token.token_type = TokenType::Plus,
                '-' => token.token_type = TokenType::Minus,
                '*' => token.token_type = TokenType::Star,
                '/' => token.token_type = TokenType::Slash,
                '!' => token.token_type = TokenType::Bang,
                '(' => token.token_type = TokenType::OpenParen,
                ')' => token.token_type = TokenType::CloseParen,
                '{' => token.token_type = TokenType::OpenBrace,
                '}' => token.token_type = TokenType::CloseBrace,
                '"' => {
                    let mut data = String::new();
                    while let Some(next_chr) = self.next_chr() {
                        self.consume_chr();
                        if next_chr == '"' {
                            break;
                        }

                        data.push(next_chr);
                    }

                    token.token_type = TokenType::String(data);
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut data = c.to_string();
                    while let Some(next_chr) = self.next_chr() {
                        if !next_chr.is_alphanumeric() && next_chr != '_' {
                            break;
                        }

                        self.consume_chr();
                        data.push(next_chr);
                    }

                    token.token_type = match data.as_str() {
                        "if" => TokenType::If,
                        "prev" => TokenType::Previous,
                        "loop" => TokenType::Loop,
                        "break" => TokenType::Break,
                        "var" => TokenType::Var,
                        "true" => TokenType::Bool(true),
                        "false" => TokenType::Bool(false),
                        _ => TokenType::Identifier(data),
                    }
                }
                '0'..='9' => {
                    let mut data = c.to_string();
                    while let Some(next_chr) = self.next_chr() {
                        if !next_chr.is_numeric() {
                            break;
                        }

                        self.consume_chr();
                        data.push(next_chr);
                    }
                    token.token_type = TokenType::Integer(data.parse().unwrap())
                }
                '=' | ';' => {
                    let mut data = c.to_string();
                    while let Some(next_chr) = self.next_chr() {
                        if next_chr.is_alphanumeric() || next_chr.is_whitespace() {
                            break;
                        }

                        self.consume_chr();
                        data.push(next_chr);
                    }
                    token.token_type = match data.as_str() {
                        "=" => TokenType::Equal,
                        "==" => TokenType::EqualEqual,
                        ";" => TokenType::Semicolon,
                        ";=" => TokenType::SemicolonEqual,
                        _ => TokenType::Invalid,
                    }
                }

                _ => token.token_type = TokenType::Invalid,
            }
        } else {
            token.token_type = TokenType::Eof
        }

        self.token_index += 1;

        Some(token)
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            if let Some(next_token) = self.parse_token() {
                if next_token.token_type == TokenType::Eof {
                    break;
                }

                tokens.push(next_token)
            }
        }

        tokens
    }
}

fn main() {
    let example_program = r#"
        var balls = 0 + 2* 10 + 4/2 !
        var balls_2 = true!
        var balls_balle = "among us 69420 * 4"!

        if balls ;= 0100 {
            print(balls)!
        }

        if balls_2 == false {
            print(69420)!
        }
    "#;

    let mut lexer = Lexer::new(example_program.to_string());
    let tokens = lexer.tokenize();

    for token in tokens.iter() {
        print!("{:?} ", token.token_type);

        if token.token_type == TokenType::Bang {
            println!();
        }
    }
    println!();
}

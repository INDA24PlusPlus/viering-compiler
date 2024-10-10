#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
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
    Bool(bool),     // TODO if i have time
    String(String), // TODO if i have time

    // Keywords
    Var,
    Print,
    If,
    Previous, // TODO if i have time
    Loop,
    Break,

    Eof,
    Invalid,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub index: usize,
}

pub struct Lexer {
    pub code: String,
    token_index: usize,
    chr_index: usize,
}

#[derive(Debug)]
pub enum LexerError {
    InsignificantToken,
    InvalidNumber,
}

impl Lexer {
    pub fn new(code: String) -> Self {
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

    fn parse_token(&mut self) -> Result<Token, LexerError> {
        let c = self.next_chr();
        self.consume_chr();

        let mut token = Token {
            token_type: TokenType::Invalid,
            index: self.token_index,
        };

        if let Some(c) = c {
            match c {
                ' ' | '\n' | '\t' => return Err(LexerError::InsignificantToken),
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
                        "print" => TokenType::Print,
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
                        if next_chr.is_alphabetic() || next_chr == '_' {
                            return Err(LexerError::InvalidNumber);
                        }

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

        Ok(token)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            match self.parse_token() {
                Ok(next_token) => {
                    if next_token.token_type == TokenType::Eof {
                        break;
                    }

                    tokens.push(next_token)
                }
                Err(e) => {
                    let message: String = match e {
                        LexerError::InsignificantToken => continue,
                        _ => {
                            format!("Error: {:?}. At: {}", e, self.token_index)
                        }
                    };

                    return Err(message);
                }
            }
        }

        Ok(tokens)
    }
}

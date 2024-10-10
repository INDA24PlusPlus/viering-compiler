use crate::lexer::Token;
use crate::lexer::TokenType;

// TODO:
// Parse loops, break, print, if statements, etc
// Don't just panic, return errors
// OPTIONAL IF TIME: Nice errors which actually show which line is at fault (for both ast and lexer)

#[derive(Debug)]
pub struct Ast {
    statements: Vec<Statement>,
}

pub struct AstParser {
    tokens: Vec<Token>,
    index: usize,
}

impl AstParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        AstParser { tokens, index: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.peek_fw(0)
    }

    fn peek_fw(&self, amount: usize) -> Option<&Token> {
        if let Some(token) = self.tokens.get(self.index + amount) {
            return Some(token);
        }

        None
    }

    fn consume(&mut self) {
        self.index += 1;
    }

    pub fn parse(&mut self) -> Ast {
        let mut ast = Ast {
            statements: Vec::new(),
        };

        while self.index < self.tokens.len() {
            let next_token_type = self.tokens[self.index].token_type.clone();

            self.consume();

            let statement = match next_token_type {
                TokenType::Var => self.parse_variable_declaration(),
                _ => {
                    panic!("Token type not handled {}", self.index);
                }
            };
            ast.statements.push(statement);
        }

        ast
    }

    fn parse_variable_declaration(&mut self) -> Statement {
        let next_token = self.peek().cloned().unwrap_or_else(|| {
            panic!("Bad variable declaration: var (<identifier>)");
        });
        self.consume();

        let identifier = match &next_token.token_type {
            TokenType::Identifier(name) => name.to_string(),
            _ => panic!("Bad variable declaration: var (<identifier>)"),
        };

        if self
            .peek()
            .map_or(true, |token| token.token_type != TokenType::Equal)
        {
            panic!("Bad variable declaration: var <identifier> (=)");
        }
        self.consume();

        let expression = Statement::VariableDeclaration(identifier, self.parse_expression());

        if self
            .peek()
            .map_or(true, |token| token.token_type != TokenType::Bang)
        {
            panic!("Expected bang");
        }
        self.consume();

        expression
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_add_sub_expression()
    }

    fn parse_add_sub_expression(&mut self) -> Expression {
        // Multiplication and division has higher priority
        let mut left = self.parse_mult_div_expression();

        while let Some(token) = self.peek().cloned() {
            match token.token_type {
                TokenType::Plus | TokenType::Minus => {
                    self.consume();
                    let right = self.parse_mult_div_expression();
                    left = Expression::BinaryOperation(
                        Box::new(left),
                        match token.token_type {
                            TokenType::Plus => BinaryOperator::Add,
                            TokenType::Minus => BinaryOperator::Subtract,
                            _ => unreachable!(),
                        },
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }

        left
    }

    fn parse_mult_div_expression(&mut self) -> Expression {
        // Parse identifiers / integers / parenthesis first
        let mut left = self.parse_primary();

        while let Some(token) = self.peek().cloned() {
            match token.token_type {
                TokenType::Star | TokenType::Slash => {
                    self.consume();
                    let right = self.parse_primary();
                    left = Expression::BinaryOperation(
                        Box::new(left),
                        match token.token_type {
                            TokenType::Star => BinaryOperator::Multiply,
                            TokenType::Slash => BinaryOperator::Divide,
                            _ => unreachable!(),
                        },
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }

        left
    }

    fn parse_primary(&mut self) -> Expression {
        if let Some(token) = self.peek().cloned() {
            match &token.token_type {
                TokenType::Integer(value) => {
                    self.consume();
                    Expression::Integer(*value)
                }
                TokenType::Identifier(name) => {
                    self.consume();
                    Expression::Variable(name.to_string())
                }
                TokenType::OpenParen => {
                    self.consume();
                    let expr = self.parse_expression();
                    if let Some(token) = self.peek() {
                        if matches!(token.token_type, TokenType::CloseParen) {
                            self.consume();
                            Expression::ParenthesisExpression(Box::new(expr))
                        } else {
                            panic!("Expected closing parenthesis");
                        }
                    } else {
                        panic!("Expected closing parenthesis");
                    }
                }
                _ => panic!("Unexpected token in expression"),
            }
        } else {
            panic!("Expected an expression");
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    Variable(String),
    BinaryOperation(Box<Expression>, BinaryOperator, Box<Expression>),
    ParenthesisExpression(Box<Expression>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Compare,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(String, Expression),
    Assignment(String, Expression),
    IfStatement(Expression, Vec<Statement>),
    LoopStatement(Vec<Statement>),
    BreakStatement,
    PrintStatement(Expression),
}

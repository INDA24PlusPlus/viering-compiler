use crate::lexer::Token;
use crate::lexer::TokenType;

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
            let statement = self.parse_statement();
            ast.statements.push(statement);
        }

        ast
    }

    fn parse_statement(&mut self) -> Statement {
        let next_token_type = self.tokens[self.index].token_type.clone();

        self.consume();

        match next_token_type {
            TokenType::Break => {
                self.consume();
                Statement::BreakStatement
            }
            TokenType::Identifier(identifier) => self.parse_assignment(identifier),
            TokenType::Loop => self.parse_loop(),
            TokenType::Var => self.parse_variable_declaration(),
            TokenType::If => self.parse_if_statement(),
            TokenType::Print => self.parse_print_statement(),
            _ => {
                panic!(
                    "Unexpected statement, began with token type {:?}",
                    next_token_type
                );
            }
        }
    }

    fn parse_print_statement(&mut self) -> Statement {
        let expression = self.parse_expression();

        self.expect_bang();

        Statement::PrintStatement(expression)
    }

    fn parse_loop(&mut self) -> Statement {
        // Opening bracket
        let next_token = self.peek().cloned().unwrap_or_else(|| {
            panic!("Bad loop: Expected opening bracket");
        });

        match &next_token.token_type {
            TokenType::OpenBrace => {}
            _ => panic!("Bad loop : loop{{<statements>}}"),
        };
        self.consume();

        // Parse the block of statements
        let mut statements: Vec<Statement> = Vec::new();
        while let Some(token) = self.peek() {
            if token.token_type == TokenType::CloseBrace {
                break;
            }
            statements.push(self.parse_statement());
        }

        // Closing bracket
        let next_token = self.peek().cloned().unwrap_or_else(|| {
            panic!("Bad loop : loop{{<statements>}}");
        });
        match &next_token.token_type {
            TokenType::CloseBrace => {}
            _ => panic!("Bad loop : Expected closing bracket"),
        };
        self.consume();

        Statement::LoopStatement(statements)
    }

    fn parse_assignment(&mut self, identifier: String) -> Statement {
        // Identifier
        let next_token = self.peek().cloned().unwrap_or_else(|| {
            panic!("Bad assignment: <identifier> = <expression>!");
        });

        match &next_token.token_type {
            TokenType::Equal => {}
            _ => panic!("Bad assignment: expected '='"),
        };
        self.consume();

        let expression = self.parse_expression();

        self.expect_bang();

        Statement::Assignment(identifier, expression)
    }

    fn parse_if_statement(&mut self) -> Statement {
        // Opening parenthesis
        let next_token = self.peek().cloned().unwrap_or_else(|| {
            panic!("Bad if statement: if(<condition>){{<statements>}}");
        });
        match &next_token.token_type {
            TokenType::OpenParen => {}
            _ => panic!("Bad if statement: Expected open parenthesis"),
        };
        self.consume();

        // Parse expression
        let condition = self.parse_expression();

        // Closing parenthesis
        let next_token = self.peek().cloned().unwrap_or_else(|| {
            panic!("Bad if statement: if(<condition>){{<statements>}}");
        });
        match &next_token.token_type {
            TokenType::CloseParen => {}
            _ => panic!("Bad if statement: Expected closing parenthesis"),
        };
        self.consume();

        // Opening bracket
        let next_token = self.peek().cloned().unwrap_or_else(|| {
            panic!("Bad if statement: if(<condition>){{<statements>}}");
        });
        match &next_token.token_type {
            TokenType::OpenBrace => {}
            _ => panic!("Bad if statement: Expected opening bracket"),
        };
        self.consume();

        // Parse the block of statements
        let mut statements: Vec<Statement> = Vec::new();
        while let Some(token) = self.peek() {
            if token.token_type == TokenType::CloseBrace {
                break;
            }
            statements.push(self.parse_statement());
        }

        // Closing bracket
        let next_token = self.peek().cloned().unwrap_or_else(|| {
            panic!("Bad if statement: if(<condition>){{<statements>}}");
        });
        match &next_token.token_type {
            TokenType::CloseBrace => {}
            _ => panic!("Bad if statement: Expected closing bracket"),
        };
        self.consume();

        Statement::IfStatement(condition, statements)
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

        self.expect_bang();

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
        let mut left = self.parse_comparision_expression();

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

    fn parse_comparision_expression(&mut self) -> Expression {
        let mut left = self.parse_primary();

        while let Some(token) = self.peek().cloned() {
            match token.token_type {
                TokenType::EqualEqual => {
                    self.consume();
                    let right = self.parse_primary();
                    left = Expression::BinaryOperation(
                        Box::new(left),
                        match token.token_type {
                            TokenType::EqualEqual => BinaryOperator::Equal,
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

    fn expect_bang(&mut self) {
        if self
            .peek()
            .map_or(true, |token| token.token_type != TokenType::Bang)
        {
            panic!("Expected bang (!)");
        }
        self.consume();
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

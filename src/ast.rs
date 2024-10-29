use core::fmt;

use crate::lexer::Token;
use crate::lexer::TokenType;

#[derive(Debug)]
pub enum AstError {
    UnexpectedStatement(TokenType),
    BadAssignment(String),
    BadLoop,
    BadVariableDeclaration,
    ExpectedBang,
    BadIfStatement,
    UnexpectedToken(TokenType),
    ExpectedClosingParenthesis,
    ExpectedExpression,
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstError::UnexpectedStatement(token_type) => write!(
                f,
                "Unexpected statement, began with token type {:?}",
                token_type
            ),
            AstError::BadAssignment(identifier) => write!(f, "Bad assignment for {}", identifier),
            AstError::BadLoop => write!(f, "Bad loop"),
            AstError::BadVariableDeclaration => write!(f, "Bad variable declaration"),
            AstError::ExpectedBang => write!(f, "Expected bang"),
            AstError::BadIfStatement => write!(f, "Bad if statement"),
            AstError::UnexpectedToken(token_type) => {
                write!(f, "Unexpected token of type {:?}", token_type)
            }
            AstError::ExpectedClosingParenthesis => write!(f, "Expected closing parenthesis"),
            AstError::ExpectedExpression => write!(f, "Expected an expression"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn print(&self) {
        for statement in self.statements.iter() {
            Self::print_statement(statement, 0);
        }
    }

    // kinda pretty prints the ast (enough to be readable at least)
    fn print_statement(statement: &Statement, indentation: usize) {
        match statement {
            Statement::LoopStatement(statements) => {
                Self::print_indented("Loop:".to_string(), indentation);
                for statement in statements {
                    Self::print_statement(statement, indentation + 1);
                }
            }
            Statement::IfStatement(expression, statements) => {
                Self::print_indented("If:".to_string(), indentation);
                Self::print_indented(format!("{:?}", expression), indentation + 1);

                Self::print_indented("Then:".to_string(), indentation);
                for statement in statements {
                    Self::print_statement(statement, indentation + 1);
                }
            }
            _ => Self::print_indented(format!("{:?}", statement), indentation),
        };
    }

    fn print_indented(to_print: String, indentation: usize) {
        println!("{}{}", " ".repeat(indentation * 4), to_print);
    }
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

    pub fn parse(&mut self) -> Result<Ast, AstError> {
        let mut ast = Ast {
            statements: Vec::new(),
        };

        while self.index < self.tokens.len() {
            let statement = self.parse_statement()?;
            ast.statements.push(statement);
        }

        Ok(ast)
    }

    fn parse_statement(&mut self) -> Result<Statement, AstError> {
        let next_token_type = self.tokens[self.index].token_type.clone();

        self.consume();

        match next_token_type {
            TokenType::Break => {
                self.consume();
                Ok(Statement::BreakStatement)
            }
            TokenType::Identifier(identifier) => self.parse_assignment(identifier),
            TokenType::Loop => self.parse_loop(),
            TokenType::Var => self.parse_variable_declaration(),
            TokenType::If => self.parse_if_statement(),
            TokenType::Print => self.parse_print_statement(),
            _ => Err(AstError::UnexpectedStatement(next_token_type)),
        }
    }

    fn parse_print_statement(&mut self) -> Result<Statement, AstError> {
        let expression = self.parse_expression()?;

        self.expect_bang()?;

        Ok(Statement::PrintStatement(expression))
    }

    fn parse_loop(&mut self) -> Result<Statement, AstError> {
        // Opening bracket
        let next_token = match self.peek().cloned() {
            Some(token) => token,
            None => return Err(AstError::BadLoop),
        };

        match &next_token.token_type {
            TokenType::OpenBrace => {}
            _ => return Err(AstError::BadLoop),
        };
        self.consume();

        // Parse the block of statements
        let mut statements: Vec<Statement> = Vec::new();
        while let Some(token) = self.peek() {
            if token.token_type == TokenType::CloseBrace {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        // Closing bracket
        let next_token = match self.peek().cloned() {
            Some(token) => token,
            None => return Err(AstError::BadLoop),
        };

        match &next_token.token_type {
            TokenType::CloseBrace => {}
            _ => return Err(AstError::BadLoop),
        };
        self.consume();

        Ok(Statement::LoopStatement(statements))
    }

    fn parse_assignment(&mut self, identifier: String) -> Result<Statement, AstError> {
        // Identifier
        let next_token = match self.peek().cloned() {
            Some(token) => token,
            None => return Err(AstError::BadAssignment(identifier)),
        };

        match &next_token.token_type {
            TokenType::Equal => {}
            _ => return Err(AstError::BadAssignment(identifier)),
        };
        self.consume();

        let expression = self.parse_expression()?;

        self.expect_bang()?;

        Ok(Statement::Assignment(identifier, expression))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, AstError> {
        // Opening parenthesis
        let next_token = match self.peek().cloned() {
            Some(token) => token,
            None => return Err(AstError::BadIfStatement),
        };

        match &next_token.token_type {
            TokenType::OpenParen => {}
            _ => return Err(AstError::BadIfStatement),
        };

        self.consume();

        // Parse expression
        let condition = self.parse_expression()?;

        // Closing parenthesis
        let next_token = match self.peek().cloned() {
            Some(token) => token,
            None => return Err(AstError::BadIfStatement),
        };

        match &next_token.token_type {
            TokenType::CloseParen => {}
            _ => return Err(AstError::BadIfStatement),
        };
        self.consume();

        // Opening bracket
        let next_token = match self.peek().cloned() {
            Some(token) => token,
            None => return Err(AstError::BadIfStatement),
        };

        match &next_token.token_type {
            TokenType::OpenBrace => {}
            _ => return Err(AstError::BadIfStatement),
        };
        self.consume();

        // Parse the block of statements
        let mut statements: Vec<Statement> = Vec::new();
        while let Some(token) = self.peek() {
            if token.token_type == TokenType::CloseBrace {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        // Closing bracket
        let next_token = match self.peek().cloned() {
            Some(token) => token,
            None => return Err(AstError::BadIfStatement),
        };

        match &next_token.token_type {
            TokenType::CloseBrace => {}
            _ => return Err(AstError::BadIfStatement),
        };
        self.consume();

        Ok(Statement::IfStatement(condition, statements))
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, AstError> {
        let next_token = match self.peek().cloned() {
            Some(token) => token,
            None => return Err(AstError::BadVariableDeclaration),
        };

        self.consume();

        let identifier = match &next_token.token_type {
            TokenType::Identifier(name) => name.to_string(),
            _ => return Err(AstError::BadVariableDeclaration),
        };

        if self
            .peek()
            .map_or(true, |token| token.token_type != TokenType::Equal)
        {
            return Err(AstError::BadVariableDeclaration);
        }
        self.consume();

        let expression = Statement::VariableDeclaration(identifier, self.parse_expression()?);

        self.expect_bang()?;

        Ok(expression)
    }

    fn parse_expression(&mut self) -> Result<Expression, AstError> {
        Ok(self.parse_add_sub_expression()?)
    }

    fn parse_add_sub_expression(&mut self) -> Result<Expression, AstError> {
        // Multiplication and division has higher priority
        let mut left = self.parse_mult_div_expression()?;

        while let Some(token) = self.peek().cloned() {
            match token.token_type {
                TokenType::Plus | TokenType::Minus => {
                    self.consume();
                    let right = self.parse_mult_div_expression()?;
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

        Ok(left)
    }

    fn parse_mult_div_expression(&mut self) -> Result<Expression, AstError> {
        // Parse identifiers / integers / parenthesis first
        let mut left = self.parse_comparision_expression()?;

        while let Some(token) = self.peek().cloned() {
            match token.token_type {
                TokenType::Star | TokenType::Slash => {
                    self.consume();
                    let right = self.parse_primary()?;
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

        Ok(left)
    }

    fn parse_comparision_expression(&mut self) -> Result<Expression, AstError> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.peek().cloned() {
            match token.token_type {
                TokenType::EqualEqual | TokenType::SemicolonEqual => {
                    self.consume();
                    let right = self.parse_primary()?;
                    left = Expression::BinaryOperation(
                        Box::new(left),
                        match token.token_type {
                            TokenType::EqualEqual => BinaryOperator::Equal,
                            TokenType::SemicolonEqual => BinaryOperator::NotEqual,
                            _ => unreachable!(),
                        },
                        Box::new(right),
                    );
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, AstError> {
        if let Some(token) = self.peek().cloned() {
            match &token.token_type {
                TokenType::Integer(value) => {
                    self.consume();
                    Ok(Expression::Integer(*value))
                }
                TokenType::Identifier(name) => {
                    self.consume();
                    Ok(Expression::Variable(name.to_string()))
                }
                TokenType::OpenParen => {
                    self.consume();
                    let expr = self.parse_expression()?;
                    if let Some(token) = self.peek() {
                        if matches!(token.token_type, TokenType::CloseParen) {
                            self.consume();
                            Ok(Expression::ParenthesisExpression(Box::new(expr)))
                        } else {
                            return Err(AstError::ExpectedClosingParenthesis);
                        }
                    } else {
                        return Err(AstError::BadIfStatement);
                    }
                }
                _ => return Err(AstError::UnexpectedToken(token.token_type)),
            }
        } else {
            return Err(AstError::ExpectedExpression);
        }
    }

    fn expect_bang(&mut self) -> Result<(), AstError> {
        if self
            .peek()
            .map_or(true, |token| token.token_type != TokenType::Bang)
        {
            return Err(AstError::ExpectedBang);
        }
        self.consume();

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Variable(String),
    BinaryOperation(Box<Expression>, BinaryOperator, Box<Expression>),
    ParenthesisExpression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(String, Expression),
    Assignment(String, Expression),
    IfStatement(Expression, Vec<Statement>),
    LoopStatement(Vec<Statement>),
    BreakStatement,
    PrintStatement(Expression),
}

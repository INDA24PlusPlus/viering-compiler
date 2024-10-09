use crate::lexer::Token;
use crate::lexer::TokenType;

#[derive(Debug)]
pub struct Ast {
    nodes: Vec<Statement>,
}

pub struct AstParser {
    tokens: Vec<Token>,
    index: usize,
}

impl AstParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        AstParser { tokens, index: 0 }
    }

    pub fn parse(&mut self) -> Ast {
        while self.index < self.tokens.len() {
            let statement: Statement;
            match self.tokens[self.index].token_type {
                TokenType::Var => {
                    statement = self.parse_variable_declaration();
                }
                _ => {
                    panic!("Token type not handled");
                }
            }
            println!("stmt: {:?}", statement);

            self.index += 1;
        }

        Ast { nodes: Vec::new() }
    }

    fn parse_variable_declaration(&mut self) -> Statement {
        let next_token = self.tokens.get(self.index + 1).unwrap_or_else(|| {
            panic!("Bad variable declaration: var (<identifier>)");
        });

        let identifier = match &next_token.token_type {
            TokenType::Identifier(name) => name.to_string(),
            _ => panic!("Bad variable declaration: var (<identifier>)"),
        };

        if self
            .tokens
            .get(self.index + 2)
            .map_or(true, |token| token.token_type != TokenType::Equal)
        {
            panic!("Bad variable declaration: var <identifier> (=)");
        }
        self.index += 2;

        Statement::VariableDeclaration(identifier, self.parse_expression())
    }

    fn parse_expression(&mut self) -> Expression {
        todo!();
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

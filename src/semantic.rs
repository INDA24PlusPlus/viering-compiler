use core::fmt;

use crate::ast::{Ast, Expression, Statement};

#[derive(Debug)]
pub enum SemanticError {
    VariableAlreadyDeclared(String),
    UnknownVariable(String),
    BreakOutsideLoop,
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SemanticError::BreakOutsideLoop => write!(f, "Used break outside of for loop"),
            SemanticError::UnknownVariable(name) => write!(f, "Unknown variable {}", name),
            SemanticError::VariableAlreadyDeclared(name) => {
                write!(f, "Variable {} already declared", name)
            }
        }
    }
}

pub struct Semantic {
    ast: Ast,
}

impl Semantic {
    pub fn new(ast: Ast) -> Self {
        Semantic { ast }
    }

    pub fn check(&self) -> Result<(), SemanticError> {
        Self::check_statement_block_scope(&self.ast.statements, Vec::new(), false)
    }

    fn check_expression_scope(
        expression: &Expression,
        variables: Vec<String>,
    ) -> Result<(), SemanticError> {
        match expression {
            Expression::Variable(name) => {
                if !variables.contains(name) {
                    return Err(SemanticError::UnknownVariable(name.to_string()));
                }
            }
            Expression::ParenthesisExpression(inner) => {
                Self::check_expression_scope(inner, variables)?;
            }
            Expression::BinaryOperation(left, _, right) => {
                Self::check_expression_scope(left, variables.clone())?;
                Self::check_expression_scope(right, variables)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn check_statement_block_scope(
        statements: &[Statement],
        mut variables: Vec<String>,
        inside_loop: bool,
    ) -> Result<(), SemanticError> {
        for statement in statements.iter() {
            match statement {
                Statement::IfStatement(condition, statements) => {
                    Self::check_expression_scope(condition, variables.clone())?;
                    Self::check_statement_block_scope(statements, variables.clone(), inside_loop)?;
                }
                Statement::LoopStatement(statements) => {
                    Self::check_statement_block_scope(statements, variables.clone(), true)?;
                }
                Statement::VariableDeclaration(name, value) => {
                    if variables.contains(name) {
                        return Err(SemanticError::VariableAlreadyDeclared(name.to_string()));
                    }

                    Self::check_expression_scope(value, variables.clone())?;

                    variables.push(name.to_string());
                }
                Statement::Assignment(name, value) => {
                    if !variables.contains(name) {
                        return Err(SemanticError::UnknownVariable(name.to_string()));
                    }

                    Self::check_expression_scope(value, variables.clone())?;
                }
                Statement::BreakStatement => {
                    if !inside_loop {
                        return Err(SemanticError::BreakOutsideLoop);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

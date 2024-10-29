use crate::ast::{Ast, BinaryOperator, Expression, Statement};

fn template(code: &str) -> String {
    let head = "#include <stdlib.h>\n#include <stdio.h>\nint main(){\n";
    let tail = "return 0;\n}";

    let mut result: String = head.to_string();
    result.push_str(code);
    result.push_str(tail);

    result
}

pub struct Transpiler {}

impl Transpiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn transpile(&self, ast: Ast) -> String {
        let mut code = String::new();

        for statement in ast.statements {
            code.push_str(&Self::compile_statement(&statement));
        }

        template(&code)
    }

    fn compile_expression(expression: &Expression) -> String {
        match expression {
            Expression::Integer(value) => value.to_string(),
            Expression::Variable(name) => name.to_string(),
            Expression::ParenthesisExpression(inner) => {
                format!("({})", Self::compile_expression(inner))
            }
            Expression::BinaryOperation(left, operation, right) => {
                format!(
                    "{} {} {}",
                    Self::compile_expression(left),
                    match operation {
                        BinaryOperator::Add => "+",
                        BinaryOperator::Subtract => "-",
                        BinaryOperator::Multiply => "*",
                        BinaryOperator::Divide => "/",
                        BinaryOperator::Equal => "==",
                        BinaryOperator::NotEqual => "!=",
                    },
                    Self::compile_expression(right)
                )
            }
        }
    }

    fn compile_statement(statement: &Statement) -> String {
        let mut code = String::new();

        match statement {
            Statement::VariableDeclaration(name, value) => {
                let var_type = "int"; // only integers supported atm
                code.push_str(
                    format!(
                        "{} {} = {};\n",
                        var_type,
                        name,
                        Self::compile_expression(value)
                    )
                    .as_str(),
                );
            }
            Statement::Assignment(name, expression) => {
                code.push_str(&format!(
                    "{} = {};\n",
                    name,
                    Self::compile_expression(expression)
                ));
            }
            Statement::LoopStatement(statements) => {
                let mut new_code = String::new();
                new_code += "while(1){\n";

                for statement in statements.iter() {
                    new_code += &Self::compile_statement(statement);
                }

                new_code += "}\n";

                code.push_str(&new_code);
            }
            Statement::IfStatement(condition, statements) => {
                let mut new_code = String::new();
                new_code += "if(";
                new_code += &Self::compile_expression(condition);
                new_code += "){\n";

                for statement in statements.iter() {
                    new_code += &Self::compile_statement(statement);
                }

                new_code += "}\n";

                code.push_str(&new_code);
            }
            Statement::PrintStatement(expression) => {
                code.push_str(&format!(
                    "printf(\"%d\\n\", {});\n",
                    Self::compile_expression(expression)
                ));
            }
            Statement::BreakStatement => {
                code.push_str("break;\n");
            }
        }

        code
    }
}

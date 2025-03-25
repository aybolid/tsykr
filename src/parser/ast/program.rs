use crate::eval::Eval;

use super::{Node, Statement};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Box<Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }

    pub fn push_statement(&mut self, statement: Box<Statement>) {
        self.statements.push(statement);
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        "".to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for Program {
    fn eval(
        &self,
        _env: std::rc::Rc<std::cell::RefCell<crate::eval::ExecutionEnvironment>>,
    ) -> Result<std::rc::Rc<crate::eval::Value>, crate::eval::EvalError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Position, Token, TokenKind};
    use crate::parser::Expression;

    #[test]
    fn test_program() {
        let mut program = Program::new();
        program.push_statement(Box::new(Statement::new_expression(
            Token::new(TokenKind::Integer(5), Position(0, 0)),
            Box::new(Expression::new_integer(Token::new(
                TokenKind::Integer(5),
                Position(0, 0),
            ))),
        )));
        program.push_statement(Box::new(Statement::new_return(
            Token::new(TokenKind::Return, Position(0, 0)),
            Box::new(Expression::new_boolean(Token::new(
                TokenKind::True,
                Position(0, 0),
            ))),
        )));

        assert!(program.as_any().is::<Program>());
        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.token_literal(), "");
        assert_eq!(program.to_string(), "5\nreturn true")
    }
}

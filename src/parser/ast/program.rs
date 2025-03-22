use super::{Node, Statement};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }

    pub fn push_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }

    pub fn eval_program(&self) {
        for statement in &self.statements {
            let result = statement.eval().unwrap();
            println!("{}", result.inspect())
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Position, Token, TokenKind};
    use crate::parser::{Boolean, ExpressionStatement, Integer, ReturnStatement};

    #[test]
    fn test_program() {
        let mut program = Program::new();
        program.push_statement(Box::new(ExpressionStatement::new(
            Token::new(TokenKind::Integer(5), Position(0, 0)),
            Box::new(Integer::new(Token::new(
                TokenKind::Integer(5),
                Position(0, 0),
            ))),
        )));
        program.push_statement(Box::new(ReturnStatement::new(
            Token::new(TokenKind::Return, Position(0, 0)),
            Box::new(Boolean::new(Token::new(TokenKind::True, Position(0, 0)))),
        )));

        assert!(program.as_any().is::<Program>());
        assert_eq!(program.statements.len(), 2);
        assert_eq!(program.token_literal(), "");
        assert_eq!(program.to_string(), "5\nreturn true")
    }
}

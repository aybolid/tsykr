use super::{Block, Identifier, Node};
use crate::lexer::{Token, TokenKind};

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub token: Token,
    pub identifier: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: Block,
}

impl FunctionDeclaration {
    pub fn new(
        token: Token,
        identifier: Identifier,
        parameters: Vec<Identifier>,
        body: Block,
    ) -> Self {
        assert_eq!(token.kind, TokenKind::Function, "expected function token");
        FunctionDeclaration {
            token,
            identifier,
            parameters,
            body,
        }
    }
}

impl ToString for FunctionDeclaration {
    fn to_string(&self) -> String {
        let mut out = String::from(self.token_literal());
        out.push(' ');

        out.push_str(&self.identifier.to_string());
        out.push('(');
        out.push_str(
            &self
                .parameters
                .iter()
                .map(|ident| ident.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        out.push_str(") ");
        out.push_str(&self.body.to_string());

        out
    }
}

impl Node for FunctionDeclaration {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Position,
        parser::{Expression, Statement},
    };

    use super::*;

    #[test]
    fn test_function_declaration_statement() {
        let block = Block::new(
            Token::new(TokenKind::LeftCurly, Position(0, 0)),
            vec![
                Box::new(Statement::new_let(
                    Token::new(TokenKind::Let, Position(0, 0)),
                    Identifier::new(Token::new(
                        TokenKind::Identifier("x".to_string()),
                        Position(0, 0),
                    )),
                    Box::new(Expression::new_int(Token::new(
                        TokenKind::Integer(5),
                        Position(0, 0),
                    ))),
                )),
                Box::new(Statement::new_return(
                    Token::new(TokenKind::Return, Position(0, 0)),
                    Box::new(Expression::new_ident(Token::new(
                        TokenKind::Identifier("x".to_string()),
                        Position(0, 0),
                    ))),
                )),
            ],
        );
        let params = vec![
            Identifier::new(Token::new(
                TokenKind::Identifier("_a".to_string()),
                Position(0, 0),
            )),
            Identifier::new(Token::new(
                TokenKind::Identifier("_b".to_string()),
                Position(0, 0),
            )),
        ];
        let function = FunctionDeclaration::new(
            Token::new(TokenKind::Function, Position(0, 0)),
            Identifier::new(Token::new(
                TokenKind::Identifier("add".to_string()),
                Position(0, 0),
            )),
            params,
            block,
        );

        assert!(function.as_any().is::<FunctionDeclaration>());
        assert_eq!(
            function.token,
            Token::new(TokenKind::Function, Position(0, 0))
        );
        assert_eq!(function.token_literal(), function.token.literal());
        assert_eq!(
            function.to_string(),
            "fn add(_a, _b) {\n  let x = 5\n  return x\n}"
        )
    }
}

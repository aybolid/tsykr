use super::{Block, Expression, Identifier, Node};
use crate::lexer::Token;

#[derive(Debug)]
pub struct FunctionExpression {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Block,
}

impl FunctionExpression {
    pub fn new(token: Token, parameters: Vec<Identifier>, body: Block) -> Self {
        assert_eq!(token, Token::Function);
        FunctionExpression {
            token,
            parameters,
            body,
        }
    }
}

impl ToString for FunctionExpression {
    fn to_string(&self) -> String {
        let mut out = String::from(self.token_literal());
        out.push(' ');

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

impl Node for FunctionExpression {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for FunctionExpression {}

#[cfg(test)]
mod tests {
    use crate::parser::{Integer, LetStatement, ReturnStatement};

    use super::*;

    #[test]
    fn test_function_expression() {
        let block = Block::new(
            Token::LeftCurly,
            vec![
                Box::new(LetStatement::new(
                    Token::Let,
                    Identifier::new(Token::Identifier("x".to_string())),
                    Box::new(Integer::new(Token::Integer(5))),
                )),
                Box::new(ReturnStatement::new(
                    Token::Return,
                    Box::new(Identifier::new(Token::Identifier("x".to_string()))),
                )),
            ],
        );
        let params = vec![
            Identifier::new(Token::Identifier("_a".to_string())),
            Identifier::new(Token::Identifier("_b".to_string())),
        ];
        let function = FunctionExpression::new(Token::Function, params, block);

        assert!(function.as_any().is::<FunctionExpression>());
        assert_eq!(function.token, Token::Function);
        assert_eq!(function.token_literal(), function.token.literal());
        assert_eq!(
            function.to_string(),
            "fn (_a, _b) {\n  let x = 5\n  return x\n}"
        )
    }
}

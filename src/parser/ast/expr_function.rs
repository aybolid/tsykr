use std::sync::Arc;

use super::{Block, Identifier, Node};
use crate::{
    eval::{Eval, EvalError, ExecEnvironment, Object},
    lexer::{Token, TokenKind},
};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionExpression {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Block,
}

impl FunctionExpression {
    pub fn new(token: Token, parameters: Vec<Identifier>, body: Block) -> Self {
        assert_eq!(token.kind, TokenKind::Function);
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

impl Eval for FunctionExpression {
    fn eval(&self, _env: &mut ExecEnvironment) -> Result<Option<Arc<Object>>, EvalError> {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         lexer::Position,
//         parser::{Integer, LetStatement, ReturnStatement},
//     };

//     use super::*;

//     #[test]
//     fn test_function_expression() {
//         let block = Block::new(
//             Token::new(TokenKind::LeftCurly, Position(0, 0)),
//             vec![
//                 Box::new(LetStatement::new(
//                     Token::new(TokenKind::Let, Position(0, 0)),
//                     Identifier::new(Token::new(
//                         TokenKind::Identifier("x".to_string()),
//                         Position(0, 0),
//                     )),
//                     Box::new(Integer::new(Token::new(
//                         TokenKind::Integer(5),
//                         Position(0, 0),
//                     ))),
//                 )),
//                 Box::new(ReturnStatement::new(
//                     Token::new(TokenKind::Return, Position(0, 0)),
//                     Box::new(Identifier::new(Token::new(
//                         TokenKind::Identifier("x".to_string()),
//                         Position(0, 0),
//                     ))),
//                 )),
//             ],
//         );
//         let params = vec![
//             Identifier::new(Token::new(
//                 TokenKind::Identifier("_a".to_string()),
//                 Position(0, 0),
//             )),
//             Identifier::new(Token::new(
//                 TokenKind::Identifier("_b".to_string()),
//                 Position(0, 0),
//             )),
//         ];
//         let function = FunctionExpression::new(
//             Token::new(TokenKind::Function, Position(0, 0)),
//             params,
//             block,
//         );

//         assert!(function.as_any().is::<FunctionExpression>());
//         assert_eq!(
//             function.token,
//             Token::new(TokenKind::Function, Position(0, 0))
//         );
//         assert_eq!(function.token_literal(), function.token.literal());
//         assert_eq!(
//             function.to_string(),
//             "fn (_a, _b) {\n  let x = 5\n  return x\n}"
//         )
//     }
// }

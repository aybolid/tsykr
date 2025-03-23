use std::{rc::Rc, sync::Arc};

use super::{Block, Identifier, Node};
use crate::{
    eval::{Eval, EvalError, ExecEnvironment, FunctionObject, Object},
    lexer::{Token, TokenKind},
};

#[derive(Debug, PartialEq, Clone)]
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
        assert_eq!(token.kind, TokenKind::Function);
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

impl Eval for FunctionDeclaration {
    fn eval(&self, env: &mut ExecEnvironment) -> Result<Option<Arc<Object>>, EvalError> {
        let function_obj = FunctionObject::new_object(
            Rc::new(env.clone()),
            self.parameters.clone(),
            self.body.clone(),
        );

        env.set(self.identifier.to_string(), Arc::new(function_obj));

        Ok(None)
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
//     fn test_function_declaration_statement() {
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
//         let function = FunctionDeclaration::new(
//             Token::new(TokenKind::Function, Position(0, 0)),
//             Identifier::new(Token::new(
//                 TokenKind::Identifier("add".to_string()),
//                 Position(0, 0),
//             )),
//             params,
//             block,
//         );

//         assert!(function.as_any().is::<FunctionDeclaration>());
//         assert_eq!(
//             function.token,
//             Token::new(TokenKind::Function, Position(0, 0))
//         );
//         assert_eq!(function.token_literal(), function.token.literal());
//         assert_eq!(
//             function.to_string(),
//             "fn add(_a, _b) {\n  let x = 5\n  return x\n}"
//         )
//     }
// }

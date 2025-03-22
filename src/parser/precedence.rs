use crate::lexer::{Token, TokenKind};

/// Expression precedence
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    /// ==, !=
    Equals,
    /// >, <, <=, >=
    LessOrGreater,
    /// +
    Sum,
    /// *, /
    Product,
    /// -x, !x
    Prefix,
    /// function()
    Call,
    /// array[0], hash_map["key"]
    Index,
}

impl Precedence {
    pub fn from_token(token: Token) -> Self {
        match token.kind {
            TokenKind::EqualsEquals | TokenKind::BangEquals => Self::Equals,

            TokenKind::GreaterThan
            | TokenKind::LessThan
            | TokenKind::GreaterThanEquals
            | TokenKind::LessThanEquals => Self::LessOrGreater,

            TokenKind::Plus | TokenKind::Minus => Self::Sum,
            TokenKind::Asterisk | TokenKind::Slash => Self::Product,
            TokenKind::LeftParen => Self::Call,
            TokenKind::LeftBracket => Self::Index,

            _ => Self::Lowest,
        }
    }
}

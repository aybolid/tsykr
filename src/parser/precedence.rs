use crate::lexer::Token;

/// Expression precedence
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
        match token {
            Token::EqualsEquals | Token::BangEquals => Self::Equals,

            Token::GreaterThan
            | Token::LessThan
            | Token::GreaterThanEquals
            | Token::LessThanEquals => Self::LessOrGreater,

            Token::Plus | Token::Minus => Self::Sum,
            Token::Asterisk | Token::Slash => Self::Product,
            Token::LeftParen => Self::Call,
            Token::LeftBracket => Self::Index,

            _ => Self::Lowest,
        }
    }
}

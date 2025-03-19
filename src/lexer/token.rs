#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Plus,
    Equals,
    Minus,
    Asterisk,
    Slash,
    Bang,
    GreaterThan,
    LessThan,

    SemiColon,
    Colon,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,

    EqualsEquals,
    BangEquals,
    GreaterThanEquals,
    LessThanEquals,

    Identifier(String),
    Integer(i64),
    Float(f64),

    Let,
    Function,
    Return,
    If,
    Else,

    ILLEGAL(char),
}

impl TokenKind {
    /// If keyword, returns the corresponding keyword token.
    /// Otherwise, returns an identifier token.
    pub fn new_alphabetic(string: String) -> TokenKind {
        match &string[..] {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Function,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,

            _ => TokenKind::Identifier(string),
        }
    }
}

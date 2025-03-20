#[derive(Debug, PartialEq)]
pub enum Token {
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

    True,
    False,

    Let,
    Function,
    Return,
    If,
    Else,

    ILLEGAL(char),
}

impl Token {
    /// If keyword, returns the corresponding keyword token.
    /// Otherwise, returns an identifier token.
    pub fn new_alphabetic(string: String) -> Token {
        match &string[..] {
            "let" => Token::Let,
            "fn" => Token::Function,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,

            _ => Token::Identifier(string),
        }
    }
}

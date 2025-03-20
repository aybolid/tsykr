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

    pub fn literal(&self) -> String {
        match self {
            Self::Plus => "+".to_string(),
            Self::Equals => "=".to_string(),
            Self::Minus => "-".to_string(),
            Self::Asterisk => "*".to_string(),
            Self::Slash => "/".to_string(),
            Self::Bang => "!".to_string(),
            Self::GreaterThan => ">".to_string(),
            Self::LessThan => "<".to_string(),

            Self::SemiColon => ";".to_string(),
            Self::Colon => ":".to_string(),
            Self::LeftParen => "(".to_string(),
            Self::RightParen => ")".to_string(),
            Self::LeftCurly => "{".to_string(),
            Self::RightCurly => "}".to_string(),
            Self::LeftBracket => "[".to_string(),
            Self::RightBracket => "]".to_string(),
            Self::Comma => ",".to_string(),
            Self::Dot => ".".to_string(),

            Self::EqualsEquals => "==".to_string(),
            Self::BangEquals => "!=".to_string(),
            Self::GreaterThanEquals => ">=".to_string(),
            Self::LessThanEquals => "<=".to_string(),

            Self::Identifier(str) => str.clone(),
            Self::Integer(integer) => integer.to_string(),
            Self::Float(float) => float.to_string(),

            Self::True => "true".to_string(),
            Self::False => "false".to_string(),

            Self::Let => "let".to_string(),
            Self::Function => "fn".to_string(),
            Self::Return => "return".to_string(),
            Self::If => "if".to_string(),
            Self::Else => "else".to_string(),

            Self::ILLEGAL(char) => char.to_string(),
        }
    }
}

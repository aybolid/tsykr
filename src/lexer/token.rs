use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
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

    True,
    False,

    Let,
    Function,
    Return,
    If,
    Else,

    ILLEGAL(char),
}

impl TokenKind {
    /// If keyword, returns the corresponding keyword token kind.
    /// Otherwise, returns an identifier token kind.
    pub fn new_alphabetic(string: String) -> Self {
        match &string[..] {
            "let" => Self::Let,
            "fn" => Self::Function,
            "return" => Self::Return,
            "if" => Self::If,
            "else" => Self::Else,
            "true" => Self::True,
            "false" => Self::False,

            _ => Self::Identifier(string),
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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(name) => write!(f, "Identifier({})", name),
            Self::Integer(value) => write!(f, "Integer({})", value),
            Self::Float(value) => write!(f, "Float({})", value),
            Self::ILLEGAL(c) => write!(f, "Illegal({})", c),
            _ => write!(f, "{}", self.literal()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position(pub usize, pub usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::ILLEGAL(' '),
            position: Position(0, 0),
        }
    }
}

impl Token {
    pub fn new(kind: TokenKind, position: Position) -> Self {
        Self { kind, position }
    }

    pub fn literal(&self) -> String {
        self.kind.literal()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}

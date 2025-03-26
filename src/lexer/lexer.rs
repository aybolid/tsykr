use super::{Position, Token, TokenKind};

#[derive(Debug)]
pub struct Lexer {
    /// Input string to be lexed
    input: String,
    /// Pointer to the current character in the input string
    current_pos: usize,
    /// Current character being processed (`current_pos` points to)
    current_ch: Option<char>,
    /// Next character being processed
    peek_ch: Option<char>,

    /// Current position in the input string
    position: Position,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            current_pos: 0,
            current_ch: None,
            peek_ch: None,
            position: Position(1, 0),
        };
        lexer.read_char();
        lexer
    }

    /// Reads the next character from the input string and updates the lexer's state.
    /// Moves the lexer's position to the next character and updates the current and peek characters.
    fn read_char(&mut self) {
        if self.current_pos >= self.input.len() {
            self.current_ch = None;
            self.peek_ch = None;
        } else {
            self.current_ch = match self.peek_ch {
                Some(ch) => Some(ch),
                None => self.input.chars().nth(self.current_pos),
            };
            self.peek_ch = self.input.chars().nth(self.current_pos + 1);
        }

        if self.current_ch.is_some() {
            self.position.1 += 1;
            if self.current_ch.unwrap() == '\n' {
                self.position.0 += 1;
                self.position.1 = 0;
            }
        }

        self.current_pos += 1;
    }

    /// Skips over any whitespace characters in the input string. I'm not a snake.
    fn fuck_whitespaces(&mut self) {
        while self.current_ch.is_some() && self.current_ch.unwrap().is_whitespace() {
            self.read_char();
        }
    }

    /// Checks if the next character is the given character.
    fn peek_char_is(&self, ch: char) -> bool {
        self.peek_ch == Some(ch)
    }

    /// Reads an alphabetic token from the input string.
    /// If keyword, returns the corresponding keyword token.
    /// Otherwise, returns an identifier token.
    fn read_alphabetic_token(&mut self) -> Token {
        let token_position = self.position;

        let start_token = self.current_ch.unwrap();
        let mut buf = String::new();
        buf.push(start_token);

        self.read_char(); // consume start token

        while self.current_is_alphabetic() {
            buf.push(self.current_ch.unwrap());
            self.read_char();
        }

        Token::new(TokenKind::new_alphabetic(buf), token_position)
    }

    fn read_numeric_token(&mut self) -> Token {
        let token_position = self.position;

        let start_token = self.current_ch.unwrap();
        let mut buf = String::new();
        buf.push(start_token);

        self.read_char(); // consume start token

        while self.current_is_numeric() || self.current_ch == Some('.') {
            buf.push(self.current_ch.unwrap());
            self.read_char();
        }

        if buf.contains('.') {
            Token::new(TokenKind::Float(buf.parse().unwrap()), token_position)
        } else {
            Token::new(TokenKind::Integer(buf.parse().unwrap()), token_position)
        }
    }

    fn read_string(&mut self) -> Token {
        let token_position = self.position;

        let mut buf = String::new();
        self.read_char();

        while self.current_ch.is_some()
            && self.current_ch != Some('"')
            && self.current_ch != Some('\n')
        {
            buf.push(self.current_ch.unwrap());
            self.read_char();
        }

        if self.current_ch == Some('"') {
            self.read_char();
        }

        Token::new(TokenKind::String(buf), token_position)
    }

    /// Checks if the current character is an alphabetic character or an underscore.
    fn current_is_alphabetic(&self) -> bool {
        match self.current_ch {
            Some(c) => c.is_alphabetic() || c == '_',
            None => false,
        }
    }

    /// Checks if the current character is a numeric character.
    fn current_is_numeric(&self) -> bool {
        match self.current_ch {
            Some(c) => c.is_numeric(),
            None => false,
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.fuck_whitespaces();

        let ch = self.current_ch;
        if ch.is_none() {
            return None;
        }

        let token_position = self.position;

        let token = match ch.expect("cant be None") {
            '+' => Token::new(TokenKind::Plus, token_position),
            '-' => Token::new(TokenKind::Minus, token_position),
            '*' => Token::new(TokenKind::Asterisk, token_position),
            '/' => {
                if self.peek_char_is('/') {
                    while self.current_ch != Some('\n') || self.current_ch.is_none() {
                        self.read_char();
                    }
                    return self.next();
                } else {
                    Token::new(TokenKind::Slash, token_position)
                }
            }

            ';' => Token::new(TokenKind::SemiColon, token_position),
            ':' => Token::new(TokenKind::Colon, token_position),
            '(' => Token::new(TokenKind::LeftParen, token_position),
            ')' => Token::new(TokenKind::RightParen, token_position),
            '{' => Token::new(TokenKind::LeftCurly, token_position),
            '}' => Token::new(TokenKind::RightCurly, token_position),
            '[' => Token::new(TokenKind::LeftBracket, token_position),
            ']' => Token::new(TokenKind::RightBracket, token_position),
            ',' => Token::new(TokenKind::Comma, token_position),
            '.' => Token::new(TokenKind::Dot, token_position),

            '=' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    Token::new(TokenKind::EqualsEquals, token_position)
                } else {
                    Token::new(TokenKind::Equals, token_position)
                }
            }
            '!' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    Token::new(TokenKind::BangEquals, token_position)
                } else {
                    Token::new(TokenKind::Bang, token_position)
                }
            }
            '>' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    Token::new(TokenKind::GreaterThanEquals, token_position)
                } else {
                    Token::new(TokenKind::GreaterThan, token_position)
                }
            }
            '<' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    Token::new(TokenKind::LessThanEquals, token_position)
                } else {
                    Token::new(TokenKind::LessThan, token_position)
                }
            }

            '"' => return Some(self.read_string()),
            _ if self.current_is_alphabetic() => return Some(self.read_alphabetic_token()),
            _ if self.current_is_numeric() => return Some(self.read_numeric_token()),

            c => Token::new(TokenKind::ILLEGAL(c), token_position),
        };

        self.read_char();
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smth_that_makes_sense() {
        let input =
            String::from("if (true) { return 2 + 2; } else { return 3.25 - 0.25; } // who cares");
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::If, Position(1, 1)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::LeftParen, Position(1, 4)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::True, Position(1, 5)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::RightParen, Position(1, 9)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::LeftCurly, Position(1, 11)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Return, Position(1, 13)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Integer(2), Position(1, 20)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Plus, Position(1, 22)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Integer(2), Position(1, 24)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::SemiColon, Position(1, 25)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::RightCurly, Position(1, 27)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Else, Position(1, 29)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::LeftCurly, Position(1, 34)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Return, Position(1, 36)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Float(3.25), Position(1, 43)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Minus, Position(1, 48)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Float(0.25), Position(1, 50)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::SemiColon, Position(1, 54)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::RightCurly, Position(1, 56)))
        );
    }

    #[test]
    fn test_boolean_tokens() {
        let input = String::from("true false");
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::True, Position(1, 1)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::False, Position(1, 6)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_numeric_tokens() {
        let input = String::from("123 456.789 -123 -23.23");
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Integer(123), Position(1, 1)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Float(456.789), Position(1, 5)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Minus, Position(1, 13)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Integer(123), Position(1, 14)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Minus, Position(1, 18)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Float(23.23), Position(1, 19)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_punctuation_tokens() {
        let input = String::from(";:[]{}(),.");
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::SemiColon, Position(1, 1)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Colon, Position(1, 2)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::LeftBracket, Position(1, 3)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::RightBracket, Position(1, 4)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::LeftCurly, Position(1, 5)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::RightCurly, Position(1, 6)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::LeftParen, Position(1, 7)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::RightParen, Position(1, 8)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Comma, Position(1, 9)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Dot, Position(1, 10)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_alphabetic_token() {
        let input = String::from("let what_x return if else");
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Let, Position(1, 1)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(
                TokenKind::Identifier("what_x".to_string()),
                Position(1, 5)
            ))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Return, Position(1, 12)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::If, Position(1, 19)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Else, Position(1, 22)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_op_tokens() {
        let input = String::from("+-*/=!<>!= <= >= ==");
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Plus, Position(1, 1)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Minus, Position(1, 2)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Asterisk, Position(1, 3)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Slash, Position(1, 4)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Equals, Position(1, 5)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Bang, Position(1, 6)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::LessThan, Position(1, 7)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::GreaterThan, Position(1, 8)))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::BangEquals, Position(1, 9)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::LessThanEquals, Position(1, 12)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::GreaterThanEquals, Position(1, 15)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::EqualsEquals, Position(1, 18)))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_new() {
        let input = String::from("80085");
        let lexer = Lexer::new(input.clone());
        assert_eq!(lexer.input, input);
        assert_eq!(lexer.current_pos, 1);
        assert_eq!(lexer.current_ch, input.chars().nth(0));
        assert_eq!(lexer.peek_ch, input.chars().nth(1));
    }

    #[test]
    fn test_skip_whitespaces() {
        let input = String::from("  80085");
        let mut lexer = Lexer::new(input.clone());
        lexer.fuck_whitespaces();
        assert_eq!(lexer.current_pos, 3);
        assert_eq!(lexer.current_ch, input.chars().nth(2));
        assert_eq!(lexer.peek_ch, input.chars().nth(3));
    }

    #[test]
    fn test_read_char() {
        let input = String::from("ick");
        let mut lexer = Lexer::new(input.clone());
        lexer.read_char();
        assert_eq!(lexer.current_pos, 2);
        assert_eq!(lexer.current_ch, input.chars().nth(1));
        assert_eq!(lexer.peek_ch, input.chars().nth(2));
        lexer.read_char();
        assert_eq!(lexer.current_pos, 3);
        assert_eq!(lexer.current_ch, input.chars().nth(2));
        assert_eq!(lexer.peek_ch, input.chars().nth(3));
        lexer.read_char();
        assert_eq!(lexer.current_pos, 4);
        assert_eq!(lexer.current_ch, input.chars().nth(3));
        assert_eq!(lexer.peek_ch, None);
        lexer.read_char();
        assert_eq!(lexer.current_pos, 5);
        assert_eq!(lexer.current_ch, None);
        assert_eq!(lexer.peek_ch, None);
    }

    #[test]
    fn test_peek_char_is() {
        let input = String::from("ick");
        let mut lexer = Lexer::new(input.clone());
        assert!(lexer.peek_char_is('c'));
        lexer.read_char();
        assert!(lexer.peek_char_is('k'));
        lexer.read_char();
        assert!(!lexer.peek_char_is('k'));
    }

    #[test]
    fn test_position_with_newlines() {
        let input = String::from("if\ntrue");
        let mut lexer = Lexer::new(input);

        let token1 = lexer.next().unwrap();
        assert_eq!(token1, Token::new(TokenKind::If, Position(1, 1)));

        let token2 = lexer.next().unwrap();
        assert_eq!(token2, Token::new(TokenKind::True, Position(2, 1)));
    }

    #[test]
    fn test_multiline_position() {
        let input = String::from("let x = 10;\nif x > 5 {\n  return true;\n}");
        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Let, Position(1, 1)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(
                TokenKind::Identifier("x".to_string()),
                Position(1, 5)
            ))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Equals, Position(1, 7)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Integer(10), Position(1, 9)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::SemiColon, Position(1, 11)))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::If, Position(2, 1)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(
                TokenKind::Identifier("x".to_string()),
                Position(2, 4)
            ))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::GreaterThan, Position(2, 6)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Integer(5), Position(2, 8)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::LeftCurly, Position(2, 10)))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::Return, Position(3, 3)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::True, Position(3, 10)))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::SemiColon, Position(3, 14)))
        );

        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenKind::RightCurly, Position(4, 1)))
        );
    }
}

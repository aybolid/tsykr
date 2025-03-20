use super::Token;

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
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            current_pos: 0,
            current_ch: None,
            peek_ch: None,
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
        let start_token = self.current_ch.unwrap();
        let mut buf = String::new();
        buf.push(start_token);

        self.read_char(); // consume start token

        while self.current_is_alphabetic() {
            buf.push(self.current_ch.unwrap());
            self.read_char();
        }

        Token::new_alphabetic(buf)
    }

    fn read_numeric_token(&mut self) -> Token {
        let start_token = self.current_ch.unwrap();
        let mut buf = String::new();
        buf.push(start_token);

        self.read_char(); // consume start token

        while self.current_is_numeric() || self.current_ch == Some('.') {
            buf.push(self.current_ch.unwrap());
            self.read_char();
        }

        if buf.contains('.') {
            Token::Float(buf.parse().unwrap())
        } else {
            Token::Integer(buf.parse().unwrap())
        }
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

        let token = match ch.expect("cant be None") {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,

            ';' => Token::SemiColon,
            ':' => Token::Colon,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftCurly,
            '}' => Token::RightCurly,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ',' => Token::Comma,
            '.' => Token::Dot,

            '=' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    Token::EqualsEquals
                } else {
                    Token::Equals
                }
            }
            '!' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    Token::BangEquals
                } else {
                    Token::Bang
                }
            }
            '>' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    Token::GreaterThanEquals
                } else {
                    Token::GreaterThan
                }
            }
            '<' => {
                if self.peek_char_is('=') {
                    self.read_char();
                    Token::LessThanEquals
                } else {
                    Token::LessThan
                }
            }

            _ if self.current_is_alphabetic() => return Some(self.read_alphabetic_token()),
            _ if self.current_is_numeric() => return Some(self.read_numeric_token()),

            c => Token::ILLEGAL(c),
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
        let input = String::from("if (true) { return 2 + 2; } else { return 3.25 - 0.25; }");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token::If));
        assert_eq!(lexer.next(), Some(Token::LeftParen));
        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::RightParen));
        assert_eq!(lexer.next(), Some(Token::LeftCurly));
        assert_eq!(lexer.next(), Some(Token::Return));
        assert_eq!(lexer.next(), Some(Token::Integer(2)));
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Integer(2)));
        assert_eq!(lexer.next(), Some(Token::SemiColon));
        assert_eq!(lexer.next(), Some(Token::RightCurly));
        assert_eq!(lexer.next(), Some(Token::Else));
        assert_eq!(lexer.next(), Some(Token::LeftCurly));
        assert_eq!(lexer.next(), Some(Token::Return));
        assert_eq!(lexer.next(), Some(Token::Float(3.25)));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Float(0.25)));
        assert_eq!(lexer.next(), Some(Token::SemiColon));
        assert_eq!(lexer.next(), Some(Token::RightCurly));
    }

    #[test]
    fn test_boolean_tokens() {
        let input = String::from("true false");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token::True));
        assert_eq!(lexer.next(), Some(Token::False));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_numeric_tokens() {
        let input = String::from("123 456.789 -123 -23.23");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token::Integer(123)));
        assert_eq!(lexer.next(), Some(Token::Float(456.789)));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Integer(123)));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Float(23.23)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_punctuation_tokens() {
        let input = String::from(";:[]{}(),.");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token::SemiColon));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::LeftBracket));
        assert_eq!(lexer.next(), Some(Token::RightBracket));
        assert_eq!(lexer.next(), Some(Token::LeftCurly));
        assert_eq!(lexer.next(), Some(Token::RightCurly));
        assert_eq!(lexer.next(), Some(Token::LeftParen));
        assert_eq!(lexer.next(), Some(Token::RightParen));
        assert_eq!(lexer.next(), Some(Token::Comma));
        assert_eq!(lexer.next(), Some(Token::Dot));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_alphabetic_token() {
        let input = String::from("let what_x return if else");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token::Let));
        assert_eq!(lexer.next(), Some(Token::Identifier("what_x".to_string())));
        assert_eq!(lexer.next(), Some(Token::Return));
        assert_eq!(lexer.next(), Some(Token::If));
        assert_eq!(lexer.next(), Some(Token::Else));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_op_tokens() {
        let input = String::from("+-*/=!<>!= <= >= ==");
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Token::Plus));
        assert_eq!(lexer.next(), Some(Token::Minus));
        assert_eq!(lexer.next(), Some(Token::Asterisk));
        assert_eq!(lexer.next(), Some(Token::Slash));
        assert_eq!(lexer.next(), Some(Token::Equals));
        assert_eq!(lexer.next(), Some(Token::Bang));
        assert_eq!(lexer.next(), Some(Token::LessThan));
        assert_eq!(lexer.next(), Some(Token::GreaterThan));

        assert_eq!(lexer.next(), Some(Token::BangEquals));
        assert_eq!(lexer.next(), Some(Token::LessThanEquals));
        assert_eq!(lexer.next(), Some(Token::GreaterThanEquals));
        assert_eq!(lexer.next(), Some(Token::EqualsEquals));
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
    fn test_fuck_whitespaces() {
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
}

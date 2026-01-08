use crate::error::LexError;

/// Lexer module responsible for tokenizing input strings.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    Caret,
    Identifier(String),
    Exclamation,
    Comma,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        match self.current_char() {
            None => Ok(Token::EOF),
            Some('+') => {
                self.advance();
                Ok(Token::Plus)
            },
            Some('-') => {
                self.advance();
                Ok(Token::Minus)
            },
            Some('*') => {
                self.advance();
                Ok(Token::Asterisk)
            },
            Some('/') => {
                self.advance();
                Ok(Token::Slash)
            },
            Some('(') => {
                self.advance();
                Ok(Token::LParen)
            },
            Some(')') => {
                self.advance();
                Ok(Token::RParen)
            },
            Some('^') => {
                self.advance();
                Ok(Token::Caret)
            },
            Some('!') => {
                self.advance();
                Ok(Token::Exclamation)
            },
            Some(',') => {
                self.advance();
                Ok(Token::Comma)
            },
            Some(ch) if ch == '.' || ch.is_digit(10) => self.read_number(),
            Some(ch) if ch.is_alphabetic() => Ok(self.read_identifier()),
            Some(ch) => Err(LexError::UnexpectedCharacter(ch)),
        }
        

    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<Token, LexError> {
        let start = self.position;
        let mut has_dot = false;

        while let Some(ch) = self.current_char() {
            if ch.is_digit(10) {
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        let number_str = self.input[start..self.position].iter().collect::<String>();
        match number_str.parse::<f64>() {
            Ok(num) => Ok(Token::Number(num)),
            Err(_) => Err(LexError::InvalidNumber(number_str)), 
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident_str = self.input[start..self.position].iter().collect::<String>();
        Token::Identifier(ident_str)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic_tokens() {
        let mut lexer = Lexer::new("3 + 4 * (.1 - 1.2)");
        assert_eq!(lexer.next_token().unwrap(), Token::Number(3.0));
        assert_eq!(lexer.next_token().unwrap(), Token::Plus);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(4.0));
        assert_eq!(lexer.next_token().unwrap(), Token::Asterisk);
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(0.1));
        assert_eq!(lexer.next_token().unwrap(), Token::Minus);
        assert_eq!(lexer.next_token().unwrap(), Token::Number(1.2));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_function_and_identifier_tokens() {
        let mut lexer = Lexer::new("sin(x) + cos(y)");
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("sin".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Plus);
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("cos".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("y".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }
}
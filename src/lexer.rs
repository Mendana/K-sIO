/// Lexer module responsible for tokenizing input strings.

use crate::error::LexError;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithPos {
    pub token: Token,
    pub position: usize,
    pub length: usize,
}

impl TokenWithPos {
    pub fn new(token: Token, position: usize, length: usize) -> Self {
        TokenWithPos { token, position, length }
    }
}

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
    Equals,
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

    pub fn tokenize(&mut self) -> Result<Vec<TokenWithPos>, LexError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            tokens.push(token);
            if tokens.last().unwrap().token == Token::EOF {
                break;
            }
        }
        Ok(tokens)
    }
    
    pub fn next_token(&mut self) -> Result<TokenWithPos, LexError> {
        self.skip_whitespace();

        let start_pos = self.position;

        let token = match self.current_char() {
            None => Token::EOF,
            Some('+') => {
                self.advance();
                Token::Plus
            },
            Some('-') => {
                self.advance();
                Token::Minus
            },
            Some('*') => {
                self.advance();
                Token::Asterisk
            },
            Some('/') => {
                self.advance();
                Token::Slash
            },
            Some('(') => {
                self.advance();
                Token::LParen
            },
            Some(')') => {
                self.advance();
                Token::RParen
            },
            Some('^') => {
                self.advance();
                Token::Caret
            },
            Some('!') => {
                self.advance();
                Token::Exclamation
            },
            Some(',') => {
                self.advance();
                Token::Comma
            },
            Some('=') => {
                self.advance();
                Token::Equals
            },
            Some(ch) if ch == '.' || ch.is_digit(10) => {
                return self.read_number(start_pos);
            },
            Some(ch) if ch.is_alphabetic() => {
                return Ok(self.read_identifier(start_pos));
            },
            Some(ch) => {
                return Err(LexError::UnexpectedCharacter(ch, self.position));
            },
        };
        
        let length = self.position - start_pos;

        Ok(TokenWithPos::new(token, start_pos, length.max(1)))
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

    fn read_number(&mut self, start_pos: usize) -> Result<TokenWithPos, LexError> {
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
        let length = self.position - start_pos;
        match number_str.parse::<f64>() {
            Ok(num) => Ok(TokenWithPos::new(Token::Number(num), start_pos, length)),
            Err(_) => Err(LexError::InvalidNumber(number_str)), 
        }
    }

    fn read_identifier(&mut self, start_pos: usize) -> TokenWithPos {
        let start = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident_str = self.input[start..self.position].iter().collect::<String>();
        let length = self.position - start_pos;
        TokenWithPos::new(Token::Identifier(ident_str), start_pos, length)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic_tokens() {
        let mut lexer = Lexer::new("3 + 4 * (.1 - 1.2)");
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Number(3.0), position: 0, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Plus, position: 2, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Number(4.0), position: 4, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Asterisk, position: 6, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::LParen, position: 8, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Number(0.1), position: 9, length: 2 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Minus, position: 12, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Number(1.2), position: 14, length: 3 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::RParen, position: 17, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::EOF, position: 18, length: 1 });
    }

    #[test]
    fn test_function_and_identifier_tokens() {
        let mut lexer = Lexer::new("sin(x) + cos(y)");
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Identifier("sin".to_string()), position: 0, length: 3 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::LParen, position: 3, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Identifier("x".to_string()), position: 4, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::RParen, position: 5, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Plus, position: 7, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Identifier("cos".to_string()), position: 9, length: 3 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::LParen, position: 12, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::Identifier("y".to_string()), position: 13, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::RParen, position: 14, length: 1 });
        assert_eq!(lexer.next_token().unwrap(), TokenWithPos { token: Token::EOF, position: 15, length: 1 });
    }
}
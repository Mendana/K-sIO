/// Parser module for the application.
/// Uses a grammar to parse input data.
/// 
/// The grammar rules are defined as follows:
/// expression    → term ((PLUS | MINUS) term)*
/// term          → factor ((STAR | SLASH) factor)*
/// factor        → power
/// power         → unary (CARET unary)*
/// unary         → (PLUS | MINUS)? postfix
/// postfix       → primary EXCLAMATION?
/// primary       → NUMBER | IDENTIFIER | function_call | LPAREN expression RPAREN
/// function_call → IDENTIFIER LPAREN arguments RPAREN
/// arguments     → expression (COMMA expression)*


use crate::{ast::{BinOp, Expr, Function, UnOp}, error::ParseError, lexer::Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn peek_token(&self) -> &Token {
        &self.tokens.get(self.position + 1).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(format!("{:?}", self.current_token())))
        }
    }

    /// Main entry point for parsing.
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_expression()
    }

    /// Parses an expression according to the grammar rules.
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = match self.current_token() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Subtract,
                _ => unreachable!(),
            };

            self.advance();
            let right = self.parse_term()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses a term according to the grammar rules.
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;

        while matches!(self.current_token(), Token::Asterisk | Token::Slash) {
            let op = match self.current_token() {
                Token::Asterisk => BinOp::Multiply,
                Token::Slash => BinOp::Divide,
                _ => unreachable!(),
            };

            self.advance();
            let right = self.parse_factor()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Parses a factor according to the grammar rules.
    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        self.parse_power()
    }

    /// Parses a power according to the grammar rules.
    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        while matches!(self.current_token(), Token::Caret) {
            self.advance();
            let right = self.parse_power()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::Power,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// Parses a unary expression according to the grammar rules.
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = match self.current_token() {
                Token::Plus => UnOp::Positive,
                Token::Minus => UnOp::Negate,
                _ => unreachable!(),
            };
            self.advance();
            let expr = self.parse_postfix()?;
            Ok(Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            })
        } else {
            self.parse_postfix()
        }
    }

    /// Parses a postfix expression according to the grammar rules.
    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let primary = self.parse_primary()?;
        if matches!(self.current_token(), Token::Exclamation) {
            self.advance();
            Ok(Expr::PostfixOp {
                expr: Box::new(primary),
                op: UnOp::Factorial,
            })
        } else {
            Ok(primary)
        }
    }

    /// Parses a primary expression according to the grammar rules.
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.current_token() {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Number(val))
            }
            Token::Identifier(name) => {
                if matches!(self.peek_token(), Token::LParen) {
                    self.parse_function_call()
                } else {
                    let name = name.clone();
                    self.advance();
                    Ok(Expr::Variable(name))
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::UnexpectedToken(format!("{:?}", self.current_token()))),
        }
    }

    /// Parses a function call according to the grammar rules.
    fn parse_function_call(&mut self) -> Result<Expr, ParseError> {
        if let Token::Identifier(name) = self.current_token() {
            let func_name = name.clone();
            self.advance();
            self.expect(Token::LParen)?;
            let mut args = Vec::new();
            if !matches!(self.current_token(), Token::RParen) {
                loop {
                    let arg = self.parse_expression()?;
                    args.push(arg);
                    if matches!(self.current_token(), Token::RParen) {
                        break;
                    }
                    self.expect(Token::Comma)?;
                }
            }
            self.expect(Token::RParen)?;
            Ok(Expr::FunctionCall {
                func: Function::from_str(&func_name).ok_or_else(|| ParseError::InvalidExpression(func_name.clone()))?,
                args,
            })
        } else {
            Err(ParseError::UnexpectedToken(format!("{:?}", self.current_token())))
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_expr(input: &str) -> Result<Expr, ParseError> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token().unwrap();
            if token == Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_simple_addition() {
        let expr = parse_expr("2 + 3").unwrap();
        matches!(expr, Expr::BinaryOp { .. });
    }

    #[test]
    fn test_precedence() {
        let expr = parse_expr("2 + 3 * 4").unwrap();
        // 2 + (3 * 4), no (2 + 3) * 4
        if let Expr::BinaryOp { left, op, right } = expr {
            assert_eq!(op, BinOp::Add);
            matches!(*left, Expr::Number(2.0));
            matches!(*right, Expr::BinaryOp { .. });
        }
    }

    #[test]
    fn test_function_call() {
        let expr = parse_expr("sin(0)").unwrap();
        if let Expr::FunctionCall { func, args } = expr {
            assert_eq!(func, Function::Sin);
            assert_eq!(args.len(), 1);
            matches!(args[0], Expr::Number(0.0));
        }
    }
}
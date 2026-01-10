use std::{error::Error, fmt, usize};

/// Error types for the application.

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnexpectedCharacter(char, usize),
    InvalidNumber(String),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::UnexpectedCharacter(ch, usize) => {
                write!(f, "Unexpected character encountered: '{}' at position {}", ch, usize)
            },
            LexError::InvalidNumber(num_str) => {
                write!(f, "Invalid number format: '{}'", num_str)
            },
        }
    }
}

impl Error for LexError {}


#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: String,
        position: usize,
    },
    UnexpectedEOF {
        position: usize,
    },
    InvalidExpression {
        message: String,
        position: usize,
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, position } => {
                write!(f, "Unexpected token at position {}: expected {}, found {}", 
                    position, expected, found)
            },
            ParseError::UnexpectedEOF { position } => {
                write!(f, "Unexpected end of input at position {}", position)
            },
            ParseError::InvalidExpression { message, position } => {
                write!(f, "Invalid expression at position {}: {}", position, message)
            },
        }
    }
}

impl Error for ParseError {}


#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    UndefinedVariable(String),
    DivisionByZero,
    InvalidArguments(String),
    MathError(String)
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::UndefinedVariable(var) => {
                write!(f, "Undefined variable encountered: '{}'", var)
            },
            EvalError::DivisionByZero => {
                write!(f, "Division by zero error")
            },
            EvalError::InvalidArguments(func) => {
                write!(f, "Invalid arguments provided to function: '{}'", func)
            },
            EvalError::MathError(msg) => {
                write!(f, "Mathematical error: '{}'", msg)
            },
        }
    }
}

impl Error for EvalError {}
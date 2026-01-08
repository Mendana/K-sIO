use std::{error::Error, fmt};

/// Error types for the application.

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnexpectedCharacter(char),
    InvalidNumber(String),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::UnexpectedCharacter(ch) => {
                write!(f, "Unexpected character encountered: '{}'", ch)
            },
            LexError::InvalidNumber(num_str) => {
                write!(f, "Invalid number format: '{}'", num_str)
            },
        }
    }
}

impl Error for LexError {}
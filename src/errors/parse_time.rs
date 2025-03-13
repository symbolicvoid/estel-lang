use super::expr::ExpectType;
use super::token::{Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub enum LexError {
    InvalidTokenError,
    UnterminatedStringError,
}

impl LexError {
    pub fn get_message(&self) -> &str {
        match self {
            Self::InvalidTokenError => "Unrecognized token",
            Self::UnterminatedStringError => "Unterminated string",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprError {
    //ExpectedTokenError(expected, got)
    ExpectTokenError(ExpectType, Token),
    UnterminatedParenthesis(Token),
}

impl ExprError {
    pub fn get_message(&self) -> &str {
        match self {
            Self::ExpectTokenError(expect_type, _) => match expect_type {
                ExpectType::Operand => "Expected an operand",
                ExpectType::Operator => "Expected an operator",
            },
            Self::UnterminatedParenthesis(_) => "Unterminated parenthesis",
        }
    }

    pub fn get_position(&self) -> (u32, u32) {
        match self {
            Self::ExpectTokenError(_, token) => (token.line, token.start),
            Self::UnterminatedParenthesis(token) => (token.line, token.start),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StmtErrors {
    pub errors: Vec<StmtError>,
}

#[derive(Debug, PartialEq)]
pub enum StmtError {
    InvalidStartToken(Token),
    //ExpectToken(expected: TokenType, got: Token)
    ExpectToken(TokenType, Token),
    InvalidExpression(ExprError),
    ExpectedExpression(Token),
    IncompleteStatement(Token),
    UnterminatedParenthesis(Token),
    UnterminatedBlock(Token),
    UnexpectedBlockClose(Token),
}

impl StmtError {
    pub fn get_message(&self) -> String {
        match self {
            Self::InvalidStartToken(_) => String::from("Invalid start of statement"),
            Self::ExpectToken(expect_type, got_token) => {
                format!(
                    "Expected {}, got {} instead",
                    expect_type.to_string(),
                    got_token.class.to_string()
                )
            }
            Self::InvalidExpression(error) => error.get_message().to_string(),
            Self::ExpectedExpression(_) => String::from("Expected an expression"),
            Self::IncompleteStatement(_) => String::from("Incomplete statement"),
            Self::UnterminatedParenthesis(_) => String::from("Unterminated parenthesis"),
            Self::UnterminatedBlock(_) => String::from("Unterminated block"),
            Self::UnexpectedBlockClose(_) => String::from("Unexpected block termination"),
        }
    }

    pub fn get_position(&self) -> (u32, u32) {
        match self {
            Self::InvalidStartToken(token) => (token.line, token.start),
            Self::ExpectToken(_, token) => (token.line, token.start),
            Self::InvalidExpression(error) => error.get_position(),
            Self::ExpectedExpression(token) => (token.line, token.start),
            Self::IncompleteStatement(token) => (token.line, token.start),
            Self::UnterminatedParenthesis(token) => (token.line, token.start),
            Self::UnterminatedBlock(token) => (token.line, token.start),
            Self::UnexpectedBlockClose(token) => (token.line, token.start),
        }
    }
}

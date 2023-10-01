use super::token::{ Token, TokenType };

#[derive(Debug, PartialEq, Clone)]
pub enum LexError{
    InvalidTokenError,
    UnterminatedStringError,
}

impl LexError{
    pub fn get_message(&self) -> &str{
        match self{
            Self::InvalidTokenError => "Unrecognized token",
            Self::UnterminatedStringError => "Unterminated string",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprError{
    //ExpectedTokenError(expected, got)
    ExpectTokenError(ExpectType, Token),
}

impl ExprError{
    pub fn get_message(&self) -> &str{
        match self{
            Self::ExpectTokenError(expect_type, _) => {
                match expect_type{
                    ExpectType::Operand => "Expected an operand",
                    ExpectType::Expression => "Expected an expression",
                }
            },
        }
    }

    pub fn get_position(&self) -> (u32, u32){
        match self{
            Self::ExpectTokenError(_, token) => (token.line, token.start),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpectType{
    Operand,
    Expression
}

#[derive(Debug, PartialEq)]
pub struct StmtErrors{
    pub errors: Vec<StmtError>,
}

#[derive(Debug, PartialEq)]
pub enum StmtError{
    InvalidStartToken(Token),
    //ExpectToken(expected: TokenType, got: Token)
    ExpectToken(TokenType, Token),
    InvalidExpression(ExprError),
}

impl StmtError{
    pub fn get_message(&self) -> String{
        match self{
            Self::InvalidStartToken(_) => String::from("Invalid start of statement"),
            Self::ExpectToken(expect_type, got_token) => {
                format!(
                    "Expected {}, got {} instead", expect_type.to_string(), got_token.class.to_string()
                )
            },
            Self::InvalidExpression(error) => 
            format!(
                "{}",
                error.get_message()
            ),
        }
    }

    pub fn get_position(&self) -> (u32, u32){
        match self{
            Self::InvalidStartToken(token) => (token.line, token.start),
            Self::ExpectToken(_, token) => (token.line, token.start),
            Self::InvalidExpression(error) => error.get_position(),
        }
    }
}
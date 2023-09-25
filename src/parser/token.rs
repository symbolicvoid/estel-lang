#[derive(Debug, PartialEq, Clone)]
pub struct Token{
    pub class: TokenType,
    pub start: u32,
    pub line: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType{
    Literal(Literal),
    Operator(Operator),
    Error(TokenErrorType),
    Eof,
}

impl TokenType{
    pub fn new_number_literal(text: &str) -> TokenType{
        let number = Literal::Number(text.parse().unwrap());
        Self::Literal(number)
    }

    pub fn new_string_literal(text: &str) -> TokenType{
        Self::Literal(Literal::String(text.to_owned()))
    }

    pub fn new_operator(text: char) -> TokenType{
        match text{
            '+' => TokenType::Operator(Operator::Add),
            '-' => TokenType::Operator(Operator::Sub),
            '*' => TokenType::Operator(Operator::Mul),
            '/' => TokenType::Operator(Operator::Div),
            _ => TokenType::Error(TokenErrorType::InvalidTokenError),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenErrorType{
    InvalidTokenError,
    UnterminatedStringError,
}

impl TokenErrorType{
    pub fn get_message(&self) -> &str{
        match self{
            Self::InvalidTokenError => "Unrecognized character",
            Self::UnterminatedStringError => "Unterminated string",
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal{
    Number(i32),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator{
    Add,
    Sub,
    Mul,
    Div
}

#[cfg(test)]
mod tests{
    use super::*;

   #[test]
   fn parse_number(){
        assert_eq!(TokenType::Literal(Literal::Number(17)), TokenType::new_number_literal("17"));
   }
}

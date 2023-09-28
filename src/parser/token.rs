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
    //Keywords
    Keyword(Keyword),
    Lparen,
    Rparen,
    //Semicolon or blank line used to terminate statements
    StmtEnd,
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

    pub fn get_literal(&self) -> Option<&Literal>{
        match self{
            Self::Literal(lit) => Some(lit),
            _ => None,
        }
    }

    pub fn get_operator(&self) -> Option<&Operator>{
        match self{
            Self::Operator(opr) => Some(opr),
            _ => None,
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

impl Literal{
    pub fn get_number(&self) -> Option<i32>{
        match self{
            Self::Number(num) => Some(*num),
            _ => None,
        }
    }

    pub fn get_string(&self) -> Option<&String>{
        match self{
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String{
        match self{
            Self::Number(num) => num.to_string(),
            Self::String(string) => string.to_owned(),
        }
    }

    pub fn add(self, other: Literal) -> Result<Literal, LiteralOpError>{
        match (self, other){
            (Self::Number(num1), Self::Number(num2)) => 
                Ok(Self::Number(num1 + num2)),
            (Self::String(string1), Self::String(string2)) => 
                Ok(Self::String(string1 + &string2)),
            (Self::String(string), Self::Number(num)) => 
                Ok(Self::String(string + &num.to_string())),
            (Self::Number(num), Self::String(string)) => 
                Ok(Self::String(num.to_string() + &string)),
            _ => return Err(LiteralOpError::InvalidTypeError),
        }
    }

    pub fn sub(self, other: Literal) -> Result<Literal, LiteralOpError>{
        match (self, other){
            (Self::Number(num1), Self::Number(num2)) => Ok(Self::Number(num1 - num2)),
            _ => return Err(LiteralOpError::InvalidTypeError),
        }
    }

    pub fn mul(self, other: Literal) -> Result<Literal, LiteralOpError>{
        match (self, other){
            (Self::Number(num1), Self::Number(num2)) => Ok(Self::Number(num1 * num2)),
            (Self::String(string), Self::Number(num)) => {
                let mut result = String::new();
                for _ in 0..num{
                    result += &string;
                }
                Ok(Self::String(result))
            }
            (Self::Number(num), Self::String(string)) => {
                let mut result = String::new();
                for _ in 0..num{
                    result += &string;
                }
                Ok(Self::String(result))
            }
            _ => return Err(LiteralOpError::InvalidTypeError),
        }
    }

    pub fn div(self, other: Literal) -> Result<Literal, LiteralOpError>{
        match (self, other){
            (Self::Number(num1), Self::Number(num2)) => {
                if num2 == 0{
                    return Err(LiteralOpError::DivByZeroError);
                }
                Ok(Self::Number(num1 / num2))
            }
            _ => return Err(LiteralOpError::InvalidTypeError),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralOpError{
    InvalidTypeError,
    DivByZeroError,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator{
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword{
    Print,
}

impl Keyword{
    pub fn new_keyword(text: &str) -> Option<Self>{
        match text{
            "print" => Some(Self::Print),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

   #[test]
   fn parse_number(){
        assert_eq!(TokenType::Literal(Literal::Number(17)), TokenType::new_number_literal("17"));
   }
}

use super::errors::{ LexError, LiteralOpError };

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
    Error(LexError),
    //Keywords
    Keyword(Keyword),
    //Identifier with name
    Ident(String),
    Lparen,
    Rparen,
    // = for assignment
    Assign,
    //Semicolon or newline used to terminate statements
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
            _ => TokenType::Error(LexError::InvalidTokenError),
        }
    }

    //Useful for error messages
    pub fn to_string(&self) -> &str{
        match self{
            Self::Literal(_) => "a literal",
            Self::Operator(_) => "an operator",
            Self::Error(_) => "error",
            Self::Keyword(_) => "a keyword",
            Self::Ident(_) => "an identifier",
            Self::Lparen => "(",
            Self::Rparen => ")",
            Self::Assign => "=",
            Self::StmtEnd => "the end of statement",
            Self::Eof => "the end of file",
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal{
    Number(i32),
    String(String),
}

impl Literal{
    pub fn to_string(&self) -> String{
        match self{
            Self::Number(num) => num.to_string(),
            Self::String(string) => 
            string.to_owned()
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
            (Self::String(string), Self::Number(num)) | 
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
pub enum Operator{
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword{
    Print,
    //Keyword to declare identifier
    Let,
}

impl Keyword{
    pub fn new_keyword(text: &str) -> Option<Self>{
        match text{
            "print" => Some(Self::Print),
            "let" => Some(Self::Let),
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

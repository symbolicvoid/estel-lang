#[derive(Debug, PartialEq, Clone)]
pub struct Token{
    pub class: TokenType,
    pub start: u32,
    pub span: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType{
    Literal(Literal),
    Operator(Operator),
    Error,
    EOF,
}

impl TokenType{
    pub fn new_number_literal(text: &str) -> TokenType{
        let number = Literal::Number(text.parse().unwrap());
        Self::Literal(number)
    }

    pub fn new_operator(text: char) -> TokenType{
        match text{
            '+' => TokenType::Operator(Operator::Add),
            '-' => TokenType::Operator(Operator::Sub),
            '*' => TokenType::Operator(Operator::Mul),
            '/' => TokenType::Operator(Operator::Div),
            _ => TokenType::Error,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

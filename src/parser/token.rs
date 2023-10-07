use super::errors::{LexError, LiteralOpError};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub class: TokenType,
    pub start: u32,
    pub line: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Literal(Literal),
    Operator(Operator),
    Unary(Unary),
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

impl TokenType {
    pub fn new_number_literal(text: &str) -> TokenType {
        let number = Literal::Number(text.parse().unwrap());
        Self::Literal(number)
    }

    pub fn new_float_literal(text: &str) -> TokenType {
        let float = Literal::Float(text.parse().unwrap());
        Self::Literal(float)
    }

    pub fn new_string_literal(text: &str) -> TokenType {
        Self::Literal(Literal::String(text.to_owned()))
    }

    pub fn new_operator(text: &str) -> TokenType {
        match text {
            "+" => Self::Operator(Operator::Add),
            "-" => Self::Operator(Operator::Sub),
            "*" => Self::Operator(Operator::Mul),
            "/" => Self::Operator(Operator::Div),
            ">" => Self::Operator(Operator::Greater),
            "<" => Self::Operator(Operator::Less),
            ">=" => Self::Operator(Operator::GreaterEqual),
            "<=" => Self::Operator(Operator::LessEqual),
            "==" => Self::Operator(Operator::Equal),
            "!=" => Self::Operator(Operator::NotEqual),
            "or" => Self::Operator(Operator::Or),
            "and" => Self::Operator(Operator::And),
            _ => panic!("Invalid operator"),
        }
    }

    pub fn new_unary(text: char) -> TokenType {
        match text {
            '-' => Self::Unary(Unary::Neg),
            '!' => Self::Unary(Unary::Not),
            _ => panic!("Invalid unary operator"),
        }
    }

    //Useful for error messages
    pub fn to_string(&self) -> &str {
        match self {
            Self::Literal(_) => "a literal",
            Self::Operator(_) => "an operator",
            Self::Unary(_) => "a unary operator",
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
pub enum Literal {
    Number(i32),
    String(String),
    Float(f32),
    Bool(bool),
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Self::Number(num) => num.to_string(),
            Self::String(string) => string.to_owned(),
            Self::Float(float) => float.to_string(),
            Self::Bool(boolean) => boolean.to_string(),
        }
    }

    pub fn add(self, other: Literal) -> Result<Literal, LiteralOpError> {
        match self {
            //Number can add other numbers, strings and floats
            Literal::Number(num1) => match other {
                Literal::Number(num2) => Ok(Self::Number(num1 + num2)),
                Literal::String(str) => Ok(Self::String(num1.to_string() + &str)),
                Literal::Float(num2) => Ok(Self::Float(num1 as f32 + num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            //Strings can be added to anything
            Literal::String(str1) => match other {
                Literal::Number(num) => Ok(Self::String(str1 + &num.to_string())),
                Literal::String(str2) => Ok(Self::String(str1 + &str2)),
                Literal::Float(num) => Ok(Self::String(str1 + &num.to_string())),
                Literal::Bool(boolean) => Ok(Self::String(str1 + &boolean.to_string())),
            },
            //Floats are similar to numbers and can be added to strings, numbers and other floats
            Literal::Float(num1) => match other {
                Literal::Number(num2) => Ok(Self::Float(num1 + num2 as f32)),
                Literal::String(str) => Ok(Self::String(num1.to_string() + &str)),
                Literal::Float(num2) => Ok(Self::Float(num1 + num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            //Booleans can only be added to a string
            Literal::Bool(boolean) => match other {
                Literal::String(str) => Ok(Self::String(boolean.to_string() + &str)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
        }
    }

    pub fn sub(self, other: Literal) -> Result<Literal, LiteralOpError> {
        //can only substract numbers and floats
        match self {
            Literal::Number(num1) => match other {
                Literal::Number(num2) => Ok(Literal::Number(num1 - num2)),
                Literal::Float(num2) => Ok(Literal::Float(num1 as f32 - num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            Literal::Float(num1) => match other {
                Literal::Number(num2) => Ok(Literal::Float(num1 - num2 as f32)),
                Literal::Float(num2) => Ok(Literal::Float(num1 - num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            _ => Err(LiteralOpError::InvalidTypeError),
        }
    }

    pub fn mul(self, other: Literal) -> Result<Literal, LiteralOpError> {
        match self {
            //Number can be multiplied to numbers, floats and strings
            Literal::Number(num1) => match other {
                Literal::Number(num2) => Ok(Self::Number(num1 * num2)),
                Literal::String(str) => {
                    let mut new_string = String::new();
                    for _ in 0..num1 {
                        new_string.push_str(&str);
                    }
                    Ok(Literal::String(new_string))
                }
                Literal::Float(num2) => Ok(Self::Float(num1 as f32 * num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            //String can only be multiplied to a number
            Literal::String(str) => match other {
                Literal::Number(num) => {
                    let mut new_string = String::new();
                    for _ in 0..num {
                        new_string.push_str(&str);
                    }
                    Ok(Literal::String(new_string))
                }
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            //Floats can be multiplied to numbers and floats
            Literal::Float(num1) => match other {
                Literal::Number(num2) => Ok(Self::Float(num1 * num2 as f32)),
                Literal::Float(num2) => Ok(Self::Float(num1 * num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            _ => Err(LiteralOpError::InvalidTypeError),
        }
    }

    pub fn div(self, other: Literal) -> Result<Literal, LiteralOpError> {
        //can only divide numbers and floats
        match self {
            Literal::Number(num1) => {
                match other {
                    //Change integers to float for accurate division
                    Literal::Number(num2) => Ok(Literal::Float(num1 as f32 / num2 as f32)),
                    Literal::Float(num2) => Ok(Literal::Float(num1 as f32 / num2)),
                    _ => Err(LiteralOpError::InvalidTypeError),
                }
            }
            Literal::Float(num1) => match other {
                Literal::Number(num2) => Ok(Literal::Float(num1 / num2 as f32)),
                Literal::Float(num2) => Ok(Literal::Float(num1 / num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            _ => Err(LiteralOpError::InvalidTypeError),
        }
    }

    pub fn greater(self, other: Literal) -> Result<Literal, LiteralOpError> {
        match self {
            Literal::Number(num1) => match other {
                Literal::Number(num2) => Ok(Literal::Bool(num1 > num2)),
                Literal::Float(num2) => Ok(Literal::Bool(num1 as f32 > num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            Literal::Float(num1) => match other {
                Literal::Number(num2) => Ok(Literal::Bool(num1 > num2 as f32)),
                Literal::Float(num2) => Ok(Literal::Bool(num1 > num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            _ => Err(LiteralOpError::InvalidTypeError),
        }
    }

    pub fn less(self, other: Literal) -> Result<Literal, LiteralOpError> {
        match self {
            Literal::Number(num1) => match other {
                Literal::Number(num2) => Ok(Literal::Bool(num1 < num2)),
                Literal::Float(num2) => Ok(Literal::Bool((num1 as f32) < num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            Literal::Float(num1) => match other {
                Literal::Number(num2) => Ok(Literal::Bool(num1 < num2 as f32)),
                Literal::Float(num2) => Ok(Literal::Bool(num1 < num2)),
                _ => Err(LiteralOpError::InvalidTypeError),
            },
            _ => Err(LiteralOpError::InvalidTypeError),
        }
    }

    pub fn equal(self, other: Literal) -> Literal {
        Literal::Bool(self == other)
    }

    pub fn greater_equal(self, other: Literal) -> Result<Literal, LiteralOpError> {
        Ok(self.clone().greater(other.clone())?.or(self.equal(other)))
    }

    pub fn less_equal(self, other: Literal) -> Result<Literal, LiteralOpError> {
        Ok(self.clone().less(other.clone())?.or(self.equal(other)))
    }

    pub fn not_equal(self, other: Literal) -> Literal {
        self.equal(other).not()
    }

    pub fn and(self, other: Literal) -> Literal {
        Literal::Bool(self.is_truthy() && other.is_truthy())
    }

    pub fn or(self, other: Literal) -> Literal {
        Literal::Bool(self.is_truthy() || other.is_truthy())
    }

    pub fn not(self) -> Literal {
        Literal::Bool(!self.is_truthy())
    }

    pub fn negate(self) -> Result<Literal, LiteralOpError> {
        match self {
            Literal::Number(num) => Ok(Literal::Number(-num)),
            Literal::Float(num) => Ok(Literal::Float(-num)),
            _ => Err(LiteralOpError::InvalidTypeError),
        }
    }

    pub fn is_truthy(&self) -> bool {
        //Numbers and floats are false if they are 0
        //Empty string are false
        match self {
            Literal::Number(num) => *num != 0,
            Literal::String(str) => !str.is_empty(),
            Literal::Float(num) => *num != 0.0,
            Literal::Bool(boolean) => boolean.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Sub,
    Add,
    Mul,
    Div,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
    Or,
    And,
}

impl Operator {
    pub fn precedence(&self) -> u8{
        match self {
            Self::Or => 1,
            Self::And => 2,
            Self::Equal | Self::NotEqual => 3,
            Self::Greater | Self::Less | Self::GreaterEqual | Self::LessEqual => 4,
            Self::Add | Self::Sub => 5,
            Self::Mul | Self::Div => 6,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unary {
    Neg,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Print,
    //Keyword to declare identifier
    Let,
}

impl Keyword {
    pub fn new_keyword(text: &str) -> Option<Self> {
        match text {
            "print" => Some(Self::Print),
            "let" => Some(Self::Let),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(
            TokenType::Literal(Literal::Number(17)),
            TokenType::new_number_literal("17")
        );
    }
}

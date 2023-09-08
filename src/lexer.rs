use crate::token::*;

pub struct Lexer{
    source: Vec<char>,
    line: u32,
    pos: u32,
    current_char: Option<char>,
}

#[allow(dead_code)]
impl Lexer{
    pub fn new(source: &str) -> Lexer{
        let source: Vec<char> = source.chars().collect();

        //If the source is empty, current character is to be set to None
        let current_char = if source.len() != 0 { Some(source[0]) } else { None };

        Self { 
            source,
            line: 1, 
            pos: 0,
            current_char
        }
    }

    pub fn lex(&mut self) -> Vec<Token>{
        let mut tokens = Vec::new();

        //continue as long as we get some character, advance() sets current character to None at the end of string
        while let Some(ch) = self.current_char{

            //save the start of the next token
            let token_start = self.pos;

            let token_type: Option<TokenType> = match ch{
                //not call advance() when another function is called to parse the characters
                //as they call advance() on their own
                '0'..='9' => Some(self.parse_number()),
                '"' | '\'' => Some(self.parse_string()),
                '+' | '-' | '/' | '*' => {
                    let ch = ch;
                    self.advance();
                    Some(TokenType::new_operator(ch))
                }
                '\r' => {
                    self.advance();
                    None
                },
                //handle newline character by incrementing the line and advancing the lexer
                '\n' => {
                    self.line += 1;
                    self.advance();
                    None
                }
                //do nothing for newlines
                ' ' => {
                    self.advance();
                    None
                }
                //error for unrecognized characters
                _ => {
                    Some(TokenType::Error)
                }
            };
            if let Some(token_type) = token_type{
                //synchronize to the next token after whitespace when error occurs
                if token_type == TokenType::Error{
                    self.synchronize_position();
                }

                tokens.push(
                    Token { 
                        class: token_type, 
                        start: token_start, 
                        line: self.line,
                    }
                )
            }
        }

        //add an EOF token at the end of the file
        tokens.push(Token { class: TokenType::EOF, start: self.pos, line: 0 });
        tokens
    }

    fn parse_number(&mut self) -> TokenType{
        let mut number = String::new();
        while let Some(ch) = self.current_char{
            match ch{
                '0'..='9' =>{ 
                    self.advance();
                    number.push(ch);
                },
                _ => return TokenType::new_number_literal(number.as_str()),
            };
        }

        //return the number when we reach EOF
        return TokenType::new_number_literal(number.as_str());
    }

    fn parse_string(&mut self) -> TokenType{
        let mut string: String = String::new();
        let start_char = self.current_char.unwrap();
        self.advance();
        while let Some(ch) = self.current_char{
            if ch == start_char{
                //advance before returning to consume the ending character
                self.advance();
                return TokenType::new_string_literal(string.as_str());
            } else {
                self.advance();
                string.push(ch);
            }
        }
        //return an error for unterminated string
        TokenType::Error
    }

    //function to advance the pos attribute and update the current character
    fn advance(&mut self) {
        self.pos += 1;
        if self.pos as usize >= self.source.len(){
            self.current_char = None;
        }
        else {
            self.current_char = Some(self.source[self.pos as usize]);
        }
    }

    //Incase of a lexical error, move the position of the lexer to the next whitespace character to continue lexing
    //this prevents a large cascade of errors from one error
    fn synchronize_position(&mut self){
        while let Some(ch) = self.current_char{
            match ch{
                ' ' | '\n' => return,
                _ => self.advance(),
            }
        }
    }

    //returns the next character without incrementing the index
    fn peek(&mut self) -> Option<char>{
        let pos = (self.pos + 1) as usize;
        if pos >= self.source.len(){
            return None
        }
        Some(self.source[pos])
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    //test the parse_number function
    #[test]
    fn num_parse(){
        //Parse a valid number
        let mut lexer = Lexer::new("45");
        assert_eq!(TokenType::Literal(Literal::Number(45)), lexer.parse_number());

        //Parse an invalid number
        lexer = Lexer::new("45p");
        assert_eq!(TokenType::Error, lexer.parse_number());
    }

    //test the parse_string function
    #[test]
    fn str_parse(){
        //parse valid strings
        let mut lexer = Lexer::new("\"Hello\"");
        assert_eq!(TokenType::new_string_literal("Hello"), lexer.parse_string());
        lexer = Lexer::new("\'Hello\'");
        assert_eq!(TokenType::new_string_literal("Hello"), lexer.parse_string());
        lexer = Lexer::new("\'Hello\"\'");
        assert_eq!(TokenType::new_string_literal("Hello\""), lexer.parse_string());

        //parse invalid strings
        lexer = Lexer::new("\'Hello");
        assert_eq!(TokenType::Error, lexer.parse_string());
        lexer = Lexer::new("\'Hello\"");
        assert_eq!(TokenType::Error, lexer.parse_string());
    }

    //compare the expected and resulted vectors one element at a time
    pub fn compare_lexer_outputs(expected: Vec<Token>, result: Vec<Token>) -> bool{
        if expected.len() == result.len(){
            let combined = expected.iter().zip(result.iter());
            for(expect, got) in combined{
                if expect != got{
                    println!("Token test case failed!");
                    println!("expect: {:?}, got: {:?}", expect, got);
                    return false;
                }
            }
            true
        } else {
            println!("expected length: {}, got: {}", expected.len(), result.len());
            false
        }
    }

    //tests for lexing basic numeric operations
    #[test]
    fn lex_basic_number_ops(){
        let mut lexer = Lexer::new("25");
        let expected = [
            Token{
                class: TokenType::new_number_literal("25"),
                start: 0,
                line: 0,
            }, 
            Token{
                class: TokenType::EOF,
                start: 2,
                line: 0,
            }, 
        ];
        assert!(compare_lexer_outputs(expected.to_vec(), lexer.lex()));

        let mut lexer = Lexer::new("25+42");
        let expected = [
            Token{
                class: TokenType::new_number_literal("25"),
                start: 0,
                line: 0,
            },
            Token{
                class: TokenType::Operator(Operator::Add),
                start: 2,
                line: 0,
            },
            Token{
                class: TokenType::new_number_literal("42"),
                start: 3,
                line: 0,
            }, 
            Token{
                class: TokenType::EOF,
                start: 5,
                line: 0,
            }, 
        ];
        assert!(compare_lexer_outputs(expected.to_vec(), lexer.lex()));
    }

    //test if the lexer can parse whitespaces correctly
    #[test]
    fn test_whitespace_skips(){
        let mut lexer = Lexer::new("       25 \n");
        let expected = [
            Token{
                class: TokenType::new_number_literal("25"),
                start: 7,
                line: 0,
            }, 
            Token{
                class: TokenType::EOF,
                start: 11,
                line: 0,
            }, 
        ];
        assert!(compare_lexer_outputs(expected.to_vec(), lexer.lex()));

        let mut lexer = Lexer::new("   8   -4");
        let expected = [
            Token{
                class: TokenType::new_number_literal("8"),
                start: 3,
                line: 0,
            },
            Token{
                class: TokenType::Operator(Operator::Sub),
                start: 7,
                line: 0,
            },
            Token{
                class: TokenType::new_number_literal("4"),
                start: 8,
                line: 0,
            },
            Token{
                class: TokenType::EOF,
                start: 9,
                line: 0,
            }, 
        ];
        assert!(compare_lexer_outputs(expected.to_vec(), lexer.lex()));
    }
}
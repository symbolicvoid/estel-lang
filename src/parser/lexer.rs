use super::token::*;

//source: The source code as a vector of characters
//line: The line number the lexer is currently at
//pos: The position of the character the lexer is currently at
//token_start: Store the start for the next token
//current_char: The character at the current position of the lexer, set to None once the source ends
pub struct Lexer{
    source: Vec<char>,
    line: u32,
    pos: u32,
    token_start: u32,
    current_char: Option<char>,
}

impl Lexer{
    pub fn new(source: &str) -> Lexer{
        let source: Vec<char> = source.chars().collect();

        //If the source is empty, current character is to be set to None
        let current_char = if source.len() != 0 { Some(source[0]) } else { None };

        Self { 
            source,
            line: 1, 
            pos: 0,
            token_start: 0,
            current_char
        }
    }

    pub fn lex(&mut self) -> Vec<Token>{
        let mut tokens: Vec<Token> = Vec::new();

        //continue as long as we get some character, advance() sets current character to None at the end of string
        while let Some(ch) = self.current_char{

            //save the start of the next token
            let token_start = self.token_start;

            let token_type: Option<TokenType> = match ch{
                //not call advance() when another function is called to lex the characters
                //as they call advance() on their own
                '0'..='9' => Some(self.lex_number()),
                'a'..='z' | 'A'..='Z' => Some(self.lex_keyword_or_identifier()),
                '"' | '\'' => Some(self.lex_string()),
                '+' | '-' | '/' | '*' => {
                    let ch = ch;
                    self.advance();
                    Some(TokenType::new_operator(ch))
                }
                '=' => {
                    self.advance();
                    Some(TokenType::Assign)
                }
                '(' => {
                    self.advance();
                    Some(TokenType::Lparen)
                }
                ')' => {
                    self.advance();
                    Some(TokenType::Rparen)
                }
                '\r' => {
                    self.advance();
                    None
                }
                //Semicolon or blank line ends statement
                ';' => {
                    self.advance();
                    Some(TokenType::StmtEnd)
                }
                //handle newline character by incrementing the line and advancing the lexer
                '\n' => {
                    self.line += 1;
                    //reset the start of the token relative to the line
                    self.token_start = 0;
                    //if the last token added was an StmtEnd, then don't add another
                    //else add an StmtEnd token
                    let token_type = if let Some(token) = tokens.last(){
                        if token.class == TokenType::StmtEnd{
                            None
                        } else {
                            Some(TokenType::StmtEnd)
                        }
                    } else {
                        Some(TokenType::StmtEnd)
                    };
                    self.advance();
                    token_type
                }
                //do nothing for whitespaces
                ' ' => {
                    self.advance();
                    None
                }
                //error for unrecognized characters
                _ => {
                    Some(TokenType::Error(TokenErrorType::InvalidTokenError))
                }
            };
            if let Some(token_type) = token_type{
                //synchronize to the next token after whitespace when error occurs
                match token_type{
                    TokenType::Error(_) => self.synchronize_position(),
                    _ => {}
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
        tokens.push(Token { class: TokenType::Eof, start: self.pos, line: self.line });
        tokens
    }

    fn lex_number(&mut self) -> TokenType{
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

    fn lex_string(&mut self) -> TokenType{
        let mut string: String = String::new();
        let start_char = self.current_char.unwrap();
        self.advance();
        while let Some(ch) = self.current_char{
            if ch == start_char{
                //advance before returning to consume the ending character
                self.advance();
                return TokenType::new_string_literal(string.as_str());
            } else if ch == '\\'{
                //handle escape characters

                //consume the backslash
                self.advance();
                //push the next character
                if let Some(ch) = self.current_char{
                    match ch{
                        'n' => string.push('\n'),
                        'r' => string.push('\r'),
                        't' => string.push('\t'),
                        '\\' => string.push('\\'),
                        '\'' => string.push('\''),
                        '"' => string.push('"'),
                        _ => {},
                    }
                }
                //consume the next character
                self.advance();
            } else {
                self.advance();
                string.push(ch);
            }
        }
        //return an error for unterminated string
        TokenType::Error(TokenErrorType::UnterminatedStringError)
    }

    //Generate keyword or identifier token
    fn lex_keyword_or_identifier(&mut self) -> TokenType{
        let mut word = String::new();
        while let Some(ch) = self.current_char{
            match ch{
                //valid identifier names can contain letters, numbers and underscores
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' =>{ 
                    self.advance();
                    word.push(ch);
                },
                ' ' | '\r' | '\n' | ';' | '(' | ')' | '+' | '-' | '*' | '/' | '=' => break,
                _ => return TokenType::Error(TokenErrorType::InvalidTokenError),
            };
        };
        
        //check if the word is a keyword else return an identifier
        if let Some(keyword) = Keyword::new_keyword(&word){
            TokenType::Keyword(keyword)
        } else {
            TokenType::Ident(word)
        }
    }

    //function to advance the pos attribute and update the current character
    fn advance(&mut self) {
        self.pos += 1;
        //advance token start whenever the position is advanced
        self.token_start += 1;
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

    //test the lex_number function
    #[test]
    fn num_lex(){
        //lex a valid number
        let mut lexer = Lexer::new("45");
        assert_eq!(TokenType::Literal(Literal::Number(45)), lexer.lex_number());
    }

    //test the lex_string function
    #[test]
    fn str_lex(){
        //lex valid strings
        let mut lexer = Lexer::new("\"Hello\"");
        assert_eq!(TokenType::new_string_literal("Hello"), lexer.lex_string());
        lexer = Lexer::new("\'Hello\'");
        assert_eq!(TokenType::new_string_literal("Hello"), lexer.lex_string());
        lexer = Lexer::new("\'Hello\"\'");
        assert_eq!(TokenType::new_string_literal("Hello\""), lexer.lex_string());

        //lex invalid strings
        lexer = Lexer::new("\'Hello");
        assert_eq!(TokenType::Error(TokenErrorType::UnterminatedStringError), lexer.lex_string());
        lexer = Lexer::new("\'Hello\"");
        assert_eq!(TokenType::Error(TokenErrorType::UnterminatedStringError), lexer.lex_string());
    }

    #[test]
    fn keyword_lex(){
        //lex valid keywords
        let mut lexer = Lexer::new("print");
        assert_eq!(TokenType::Keyword(Keyword::Print), lexer.lex_keyword_or_identifier());
        lexer = Lexer::new("print 25");
        assert_eq!(TokenType::Keyword(Keyword::Print), lexer.lex_keyword_or_identifier());
        lexer = Lexer::new("print\n");
        assert_eq!(TokenType::Keyword(Keyword::Print), lexer.lex_keyword_or_identifier());
        lexer = Lexer::new("print;");
        assert_eq!(TokenType::Keyword(Keyword::Print), lexer.lex_keyword_or_identifier());
        lexer = Lexer::new("print 25;");
        assert_eq!(TokenType::Keyword(Keyword::Print), lexer.lex_keyword_or_identifier());

        //lex valid identifiers
        lexer = Lexer::new("hello");
        assert_eq!(TokenType::Ident("hello".to_string()), lexer.lex_keyword_or_identifier());
        lexer = Lexer::new("hello 25");
        assert_eq!(TokenType::Ident("hello".to_string()), lexer.lex_keyword_or_identifier());
        lexer = Lexer::new("hello_\n");
        assert_eq!(TokenType::Ident("hello_".to_string()), lexer.lex_keyword_or_identifier());
        lexer = Lexer::new("hello123;");
        assert_eq!(TokenType::Ident("hello123".to_string()), lexer.lex_keyword_or_identifier());

        //lex invalid identifiers
        lexer = Lexer::new("h$llo");
        assert_eq!(TokenType::Error(TokenErrorType::InvalidTokenError), lexer.lex_keyword_or_identifier());
    }

    //compare the expected and resulted vectors one element at a time
    //prints all failed token comparisons
    fn compare_lexer_outputs(expected: Vec<Token>, result: Vec<Token>) -> bool{
        let mut pass = true;
        if expected.len() == result.len(){
            let combined = expected.iter().zip(result.iter());
            for(expect, got) in combined{
                if expect != got{
                    println!("Token test case failed!");
                    println!("expect: {:?}, got: {:?}", expect, got);
                    pass = false;
                }
            }
        } else {
            println!("expected length: {}, got: {}", expected.len(), result.len());
            pass = false;
        }
        pass
    }

    //tests for lexing basic numeric operations
    #[test]
    fn lex_basic_number_ops(){
        let mut lexer = Lexer::new("25");
        let expected = [
            Token{
                class: TokenType::new_number_literal("25"),
                start: 0,
                line: 1,
            }, 
            Token{
                class: TokenType::Eof,
                start: 2,
                line: 1,
            }, 
        ];
        assert!(compare_lexer_outputs(expected.to_vec(), lexer.lex()));

        let mut lexer = Lexer::new("25+42");
        let expected = [
            Token{
                class: TokenType::new_number_literal("25"),
                start: 0,
                line: 1,
            },
            Token{
                class: TokenType::new_operator('+'),
                start: 2,
                line: 1,
            },
            Token{
                class: TokenType::new_number_literal("42"),
                start: 3,
                line: 1,
            }, 
            Token{
                class: TokenType::Eof,
                start: 5,
                line: 1,
            }, 
        ];
        assert!(compare_lexer_outputs(expected.to_vec(), lexer.lex()));
    }

    //test if the lexer can skip whitespaces correctly
    #[test]
    fn test_whitespace_skips(){
        let mut lexer = Lexer::new("       25 \n");
        let expected = [
            Token{
                class: TokenType::new_number_literal("25"),
                start: 7,
                line: 1,
            },
            Token{
                class: TokenType::StmtEnd,
                start: 10,
                line: 2,
            },
            Token{
                class: TokenType::Eof,
                start: 11,
                line: 2,
            }, 
        ];
        assert!(compare_lexer_outputs(expected.to_vec(), lexer.lex()));

        let mut lexer = Lexer::new("   8   -4");
        let expected = [
            Token{
                class: TokenType::new_number_literal("8"),
                start: 3,
                line: 1,
            },
            Token{
                class: TokenType::new_operator('-'),
                start: 7,
                line: 1,
            },
            Token{
                class: TokenType::new_number_literal("4"),
                start: 8,
                line: 1,
            },
            Token{
                class: TokenType::Eof,
                start: 9,
                line: 1,
            }, 
        ];
        assert!(compare_lexer_outputs(expected.to_vec(), lexer.lex()));
    }
}
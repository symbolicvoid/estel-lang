use crate::token::{Token, TokenType};
use crate::lexer::Lexer;

pub struct Interpreter{
    source: String,
    tokens: Vec<Token>,
}

impl Interpreter{
    pub fn new(source: String) -> Interpreter{
        Self { 
            source,
            tokens: Vec::new(), 
        }
    }

    pub fn interpret(&mut self){
        let mut lexer = Lexer::new(&self.source);
        self.tokens = lexer.lex();
        self.check_lex_errors();
        println!("Out:");
        for token in self.tokens.iter(){
            println!("{:?}", token);
        }
    }


    //Check for lexical errors in the program
    //If an error occured return true and print the error message
    fn check_lex_errors(&self) -> bool{
        let mut error = false;
        for token in self.tokens.iter(){
            if let TokenType::Error(err_type) = &token.class {
                eprintln!("\x1b[0;31mError: {} at line {} position {}\x1b[0m", 
                    err_type.get_message(), token.line, token.start
                );
                error = true;
            }
        }
        error
    }
}
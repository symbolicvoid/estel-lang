use std::io::{self, Write};
use colored::Colorize;
use crate::parser::parser::Parser;
use crate::parser::stmt::Block;
use crate::token::{Token, TokenType};
use crate::lexer::Lexer;

pub struct Interpreter{
    source: String,
    tokens: Vec<Token>,
}

impl Interpreter{
    pub fn new() -> Interpreter{
        let source = String::from("");
        Self { 
            source,
            tokens: Vec::new(), 
        }
    }

    pub fn run_prompt(&mut self){
        //create a single block for a prompt session
        let mut prompt_block: Block = Block::new(Vec::new(), None);
        println!(
            "{}", 
            "Entering prompt mode, use !q or !quit to exit. To run a file, use estel [filename]".green()
        );
        loop{
            self.source.clear();
            print!(">>");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut self.source)
                .expect(format!("{}", "Failed to read input!".red()).as_str());

            if self.source == "!q\r\n" || self.source == "!quit\r\n"{
                break;
            }

            self.tokens = Lexer::new(&self.source).lex();
            if self.check_lex_errors(){
                //Stop interpreting if a lexical error occured
                continue;
            }
            //add new variables to the block
            let block = Parser::new(&self.tokens).parse(None);
            match block{
                Err(_) => continue,
                Ok(block) => {
                    //copy the statements from the new block to the prompt block
                    prompt_block.stmts = block.stmts;
                    //show Expr result in prompt
                    prompt_block.execute(true);
                }
            }
        }
    }

    pub fn interpret(&mut self, source: String){
        self.source = source;
        let mut lexer = Lexer::new(&self.source);
        self.tokens = lexer.lex();
        if self.check_lex_errors(){
            //Stop interpreting if a lexical error occured
            return;
        }
        println!("Tokens:");
        for token in self.tokens.iter(){
            println!("{:?}", token);
        }

        //Parser
        let mut parser = Parser::new(&self.tokens);
        let block = parser.parse(None);
        println!("Block: {:?}", block);
        match block{
            Err(_) => {},
            Ok(mut block) => {
                block.execute(false);
            }
        }
    }


    //Check for lexical errors in the program
    //If an error occured return true and print the error message
    fn check_lex_errors(&self) -> bool{
        let mut error = false;
        for token in self.tokens.iter(){
            if let TokenType::Error(err_type) = &token.class {
                eprintln!( "{}",
                    format!("Error: {} at line {} position {}", 
                    err_type.get_message(), token.line, token.start).red()
                );
                error = true;
            }
        }
        error
    }
}
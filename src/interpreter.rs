use crate::errors::ErrorHandler;
use crate::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::parser::stmt::Block;
use crate::token::Token;
use colored::Colorize;
use std::io::{self, Write};

pub struct Interpreter {
    source: String,
    tokens: Vec<Token>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let source = String::from("");
        Self {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn run_prompt(&mut self) {
        //create a single block for a prompt session
        let mut prompt_block: Block = Block::new(Vec::new(), None);
        println!(
            "{}",
            "Entering prompt mode, use !q or !quit to exit. To run a file, use estel [filename]"
                .green()
        );
        loop {
            self.source.clear();

            print!(">>");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut self.source)
                .expect(format!("{}", "Failed to read input!".red()).as_str());

            if self.source == "!q\r\n" || self.source == "!quit\r\n" {
                break;
            }

            let mut error_handler = ErrorHandler::new(&self.source);

            self.tokens = Lexer::new(&self.source).lex();
            //Print lexical errors
            if error_handler.find_lexical_errors(&self.tokens) {
                error_handler.print_lexical_errors();
                continue;
            }

            //add new variables to the block
            let block = Parser::new(&self.tokens).parse(None);
            match block {
                Err(errors) => {
                    //handle errors using error handler
                    error_handler.print_stmt_errors(&errors);
                }
                Ok(block) => {
                    //copy the statements from the new block to the prompt block
                    prompt_block.stmts = block.stmts;
                    //show Expr result in prompt
                    prompt_block.execute(true);
                }
            }
        }
    }

    pub fn interpret(&mut self, source: String) {
        self.source = source;

        let mut error_handler = ErrorHandler::new(&self.source);
        let mut lexer = Lexer::new(&self.source);
        self.tokens = lexer.lex();

        //Stop interpreting if a lexical error occured
        if error_handler.find_lexical_errors(&self.tokens) {
            error_handler.print_lexical_errors();
            return;
        }

        println!("Tokens:");
        for token in self.tokens.iter() {
            println!("{:?}", token);
        }

        //Parser
        let mut parser = Parser::new(&self.tokens);
        let block = parser.parse(None);
        println!("Block: {:?}", block);
        match block {
            Err(errors) => {
                error_handler.print_stmt_errors(&errors);
            }
            Ok(mut block) => {
                block.execute(false);
            }
        }
    }
}

use crate::errors::ErrorHandler;
use crate::lexer::Lexer;
use crate::parser::executor::{Executor, Scope};
use crate::parser::parser::Parser;
use crate::token::Token;
use colored::Colorize;
use std::io::{self, Write};

pub struct Interpreter {
    source: String,
    tokens: Vec<Token>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
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
        //create am executor that prints expressions for prompt session
        let mut executor = Executor::new(true, Scope::new());
        println!(
            "{}",
            "Entering prompt mode, use !q or !quit to exit. To run a file, use estel [filename]"
                .green()
        );
        loop {
            self.source.clear();

            print!(">>>>");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut self.source)
                .unwrap_or_else(|_| panic!("{}", "Failed to read input!".red()));

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
            let block = Parser::new(&self.tokens).parse();
            match block {
                Err(errors) => {
                    //handle errors using error handler
                    error_handler.print_stmt_errors(&errors);
                }
                Ok(block) => {
                    executor.execute_code(block);
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

        //Parser
        let mut parser = Parser::new(&self.tokens);

        //Executor
        let mut executor = Executor::new(false, Scope::new());

        let program = parser.parse();

        match program {
            Err(errors) => {
                error_handler.print_stmt_errors(&errors);
            }
            Ok(program) => {
                executor.execute_code(program);
            }
        }
    }
}

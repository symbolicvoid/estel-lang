use super::{
    token::{Token, TokenType},
    StmtErrors,
};
use colored::Colorize;

pub struct ErrorHandler<'a> {
    source: &'a str,
    lex_errors: Vec<&'a Token>,
}

impl<'a> ErrorHandler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lex_errors: Vec::new(),
        }
    }

    //Searches the tokens for Error tokens, return true if error is found and saves it in a vector
    pub fn find_lexical_errors(&mut self, tokens: &'a Vec<Token>) -> bool {
        let mut had_error = false;
        for token in tokens {
            match &token.class {
                TokenType::Error(_) => {
                    self.lex_errors.push(token);
                    had_error = true;
                }
                _ => (),
            }
        }
        had_error
    }

    pub fn print_lexical_errors(&self) {
        for token in &self.lex_errors {
            if let TokenType::Error(err_type) = &token.class {
                eprintln!(
                    "{}",
                    format!(
                        "Error: {} at line {} position {}",
                        err_type.get_message(),
                        token.line,
                        token.start
                    )
                    .bright_red()
                );
                self.print_code_snippet(token.line, token.start, 1)
            }
        }
    }

    pub fn print_stmt_errors(&self, errors: &'a StmtErrors) {
        for error in errors.errors.iter() {
            let error_position = error.get_position();
            eprintln!(
                "{}",
                format!(
                    "Error: {} at line {} position {}",
                    error.get_message(),
                    error_position.0,
                    error_position.1
                )
                .bright_red()
            );
            self.print_code_snippet(error_position.0, error_position.1, 1)
        }
    }

    //prints a code snippet around the line where the error occured and point at the error
    fn print_code_snippet(&self, line: u32, pos: u32, surround_lines: u32) {
        let mut current_line: u32 = 1;
        eprintln!();
        //prevent overflow
        let start_line = {
            if line > surround_lines {
                line - surround_lines
            } else {
                1
            }
        };

        let end_line = line + surround_lines;
        //Calculate the number of characters taken by the line number
        let gap = line.to_string().len() as u32;

        for code_line in self.source.lines() {
            if current_line == line {
                eprintln!(
                    "{}{}",
                    (current_line.to_string() + " | ").bright_cyan(),
                    code_line
                );
                //make an arrow to the position
                for _ in 0..gap + pos + 3 {
                    eprint!(" ");
                }
                eprintln!("{}", "^".bright_red());
            } else if current_line >= start_line && current_line <= end_line {
                //equalize the gap with the line with line number
                for _ in 0..gap {
                    eprint!(" ");
                }
                eprintln!(
                    "{}{}",
                    " | ".bright_cyan(),
                    code_line.truecolor(150, 150, 150)
                );
            }
            current_line += 1;
        }
        eprintln!("\n")
    }
}

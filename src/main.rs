use std::{io, fs, env};
use estel::{lexer::*, token::Token};

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut tokens;
    if args.len() == 1{
        //run the prompt version for no arguments
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read input");
            tokens = run_prompt(&input);
            println!("Out:");
            for token in tokens{
                println!("{:?}", token)
            }
        }
    }
    else {
        tokens = run_file(&args[1]);
        println!("Out:");
        for token in tokens{
            println!("{:?}", token)
        }
    }
}

fn run_file(file: &str) -> Vec<Token>{
    let contents = fs::read_to_string(file)
                                .expect("Error occured when reading the file");
    run_code(contents.as_str())
}

fn run_code(code: &str) -> Vec<Token>{
    Lexer::new(code).lex()
}

fn run_prompt(input: &str) -> Vec<Token>{
    run_code(input)
}
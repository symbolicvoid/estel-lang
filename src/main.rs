use std::{io, fs, env};
use estel::interpreter::Interpreter;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        //run the prompt version for no arguments
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read input");
            match input.as_str(){
                //q or quit to close the shell
                "q\r\n" | "quit\r\n" => return,
                _ => {
                    run_prompt(input);
                }
            }
        }
    }
    else {
        run_file(&args[1]);
    }
}

fn run_file(file: &str){
    let contents = fs::read_to_string(file)
                                .expect("Error occured when reading the file");
    run_code(contents);
}

fn run_code(code: String){
   let mut interpreter = Interpreter::new(code);
   interpreter.interpret();
}

fn run_prompt(input: String){
    run_code(input);
}
use estel::interpreter::Interpreter;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();
    if args.len() == 1 {
        interpreter.run_prompt();
    } else {
        interpreter.interpret(open_file(&args[1]));
    }
}

fn open_file(file: &str) -> String {
    fs::read_to_string(file).expect("Failed to read file")
}

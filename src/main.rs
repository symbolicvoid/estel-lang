use std::{ fs, env };
use estel::interpreter::Interpreter;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();
    if args.len() == 1{
        interpreter.run_prompt();
    }
    else {
        interpreter.interpret(open_file(&args[1]));
    }
}

fn open_file(file: &str) -> String{
    let contents = fs::read_to_string(file)
                                .expect("Error occured when reading the file");
    contents
}
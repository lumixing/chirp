use std::{env, fs};

use lexer::Lexer;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args.get(1).expect("expected input path");
    let source = fs::read_to_string(input_path).expect("invalid input path");
    let lexer = Lexer::new(&source);

    for (token, _) in lexer {
        println!("{:?}", token);
    }
}

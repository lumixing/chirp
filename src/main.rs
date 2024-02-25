use std::{env, fs};

use interp::interp;
use lexer::Lexer;
use parser::parse;

mod interp;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args.get(1).expect("expected input path");
    let source = fs::read_to_string(input_path).expect("invalid input path");

    let lexer = Lexer::new(&source);
    // for (token, _) in lexer {
    // println!("{:?}", token);
    // }

    let program = parse(lexer).expect("error while parsing");
    // for st in program.statements {
    // println!("{:?}", st.node);
    // }

    let ins = interp(&program);
    println!("{:#04X?}", ins);
}

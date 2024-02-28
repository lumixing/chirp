use interp::interp;
use lexer::Lexer;
use parser::parse;
use std::{env, fs};

use crate::diagnostic::Diagnostic;

mod diagnostic;
mod interp;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args.get(1).expect("expected input path");
    let output_path = &args.get(2).expect("expected output path");

    let source = fs::read_to_string(input_path).expect("could not read input");
    let diagnostic = Diagnostic::from_source(&source);
    let lexer = Lexer::new(&source);

    let program = match parse(lexer) {
        Ok(p) => p,
        Err(err) => {
            let (token, span) = err.0.unwrap();
            diagnostic.error(span, format!("{} but got {:?}", err.1, token));
            return;
        }
    };

    let ins = interp(&program, &diagnostic);
    fs::write(output_path, ins.clone()).expect("could not write output");

    println!(
        "successfully compiled {} bytes to {}",
        ins.len(),
        output_path
    );
}

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
    let output_path = &args.get(2).expect("expected output path");
    let source = fs::read_to_string(input_path).expect("could not read input");

    let lexer = Lexer::new(&source);
    // for (token, _) in lexer {
    // println!("{:?}", token);
    // }

    let program = parse(lexer).expect("error while parsing");
    // for st in program.statements {
    // println!("{:?}", st.node);
    // }

    let ins = interp(&program);
    // for (pc, i) in ins.chunks(2).enumerate() {
    //     if i.len() == 1 {
    //         println!("{:03X?} {:02X?}", 0x200 + (pc * 2), i[0]);
    //     } else {
    //         println!("{:03X?} {:02X?}{:02X?}", 0x200 + (pc * 2), i[0], i[1]);
    //     }
    // }

    fs::write(output_path, ins.clone()).expect("could not write output");
    println!(
        "successfully compiled {} bytes to {}",
        ins.len(),
        output_path
    );
}

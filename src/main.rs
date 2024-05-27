use lex::Lexer;
use parse::Parser;
use std::{env, fs, io::Read};

mod lex;
mod parse;

fn main() {
    println!("Teeny Tiny Compiler - Rust edition");

    let mut source = String::new();

    if env::args().len() != 2 {
        panic!("Error: Compiler needs source file as argument");
    }

    if let Some(file_path) = env::args().nth(1) {
        //open file provided in args
        let mut file = fs::File::open(file_path).unwrap();
        file.read_to_string(&mut source).unwrap();
    }

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    parser.program();

    println!("Parsing completed")
}

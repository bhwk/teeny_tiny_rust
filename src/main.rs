use lex::{Lexer, TokenType};

mod lex;

fn main() {
    let source = "+- */ > >= = !=";
    let mut lexer = Lexer::new(source.into());

    let mut token = lexer.get_token().unwrap();

    while token.kind != TokenType::EOF {
        println!("{:?}", token.kind);
        token = lexer.get_token().unwrap();
    }
}

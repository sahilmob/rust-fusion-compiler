use ast::lexer::{Lexer, Token};

mod ast;
fn main() {
    let input = "7";
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = vec![];

    while let Some(t) = lexer.next_token() {
        tokens.push(t)
    }

    println!("{:#?}", tokens);
}

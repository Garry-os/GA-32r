use arch::asm::lexer::{Token, tokenize};

fn main() {
    let str = String::from("Hello World from a parser 123");

    println!("Original string: {}", str);

    let tokens: Vec<Token> = tokenize(&str);
    for i in 0..tokens.len() {
        println!("{}, number: {}", tokens[i].content, tokens[i].number);
    }
}

use arch::asm::lexer::{Token, tokenize};

fn main() {
    let str = String::from("Hello World from a parser");

    println!("Original string: {}", str);

    let tokens: Vec<Token> = tokenize(&str);
    for i in 0..tokens.len() {
        println!("{},", tokens[i].content);
    }
}

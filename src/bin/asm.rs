use arch::asm::lexer::{Token, tokenize};
use arch::asm::asm::Context;

fn main() {
    // Create an assembler context
    let mut context = Context {
        tokens: Vec::new(),
        buffer: Vec::new()
    };

    let test = String::from("MOV A, 123");

    println!("Original string: {}", test);

    let tokens: Vec<Token> = tokenize(&test, "test.asm");
    context.tokens = tokens;
    for i in 0..context.tokens.len() {
        println!("{}, number: {}", context.tokens[i].content, context.tokens[i].number);
    }
}

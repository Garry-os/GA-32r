use arch::asm::lexer::{Token, tokenize};
use arch::asm::asm::Context;

fn main() {
    // Create an assembler context
    let mut context = Context::default();

    let test = String::from("MOV A, 123");

    println!("Original string: {}", test);

    let tokens: Vec<Token> = tokenize(&test, "test.asm");
    context.tokens = tokens;
    for token in context.tokens {
        println!("{}, number: {}", token.content, token.number);
    }
}

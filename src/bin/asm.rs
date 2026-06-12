use std::io::Write;

use arch::asm::lexer::{Token, tokenize};
use arch::asm::asm::Context;
use arch::asm::parser::parse;

fn main() {
    // Create an assembler context
    let mut context = Context::default();

    let test = String::from("PUSH 123");

    // Tokenize
    let tokens: Vec<Token> = tokenize(&test, "test.asm");
    context.tokens = tokens;

    // Parse
    match parse(&mut context) {
        Ok(_) => println!("Parsed successfully!"),
        Err(_) => println!("Failed to parse!")
    };

    // Print out the buffer
    for byte in context.buffer {
        print!("{:X} ", byte);
        std::io::stdout().flush().unwrap();
    }
    println!("");
}

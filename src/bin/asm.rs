use std::env;
use std::fs;

use arch::asm::lexer::{Token, tokenize};
use arch::asm::asm::{Context, err_to_msg};
use arch::asm::parser::parse;

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Syntax: ./asm [asm file] [OPTIONAL output file]");
        return;
    }

    let src = fs::read_to_string(&args[1]).expect("Failed to read file");
    let mut out_path = "a.out";
    if args.len() > 2 {
        out_path = &args[2];
    }

    // Create an assembler context
    let mut context = Context::default();

    // Tokenize
    let tokens: Vec<Token> = tokenize(&src, "test.asm");
    context.tokens = tokens;

    // Parse
    match parse(&mut context) {
        Ok(_) => println!("Parsed successfully!"),
        Err(err) => println!("Failed to parse! Error: {}", err_to_msg(&err))
    };

    // Output into a file
    fs::write(out_path, context.buffer).expect("Failed to write");
}

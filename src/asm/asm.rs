use crate::asm::lexer::Token;

pub struct Context {
    pub tokens: Vec<Token>,
    pub buffer: Vec<u8>
}

// Should be called when having a compiling error in the source file
pub fn asm_err(token: &Token, reason: &str) {
    println!("{} {}:{}: {}", token.o_file, token.o_line, token.o_index, reason);
}

// This is an alternative to asm_err(),
// where token isn't avaliable
pub fn asm_err_info(file: &str, line: &usize, index:&usize, reason: &str) {
    println!("{} {}:{}: {}", file, line, index, reason);
}


use crate::asm::lexer::Token;
use crate::arch::arch::*;

#[derive(Default)]
pub struct Context {
    pub tokens: Vec<Token>,
    pub buffer: Vec<u8>
}

// Opcode
pub struct Op {
    pub name: &'static str,
    pub num_args: usize,
    pub opcode: u8
}

// Opcode lookup table
pub const OP_TABLE: &[Op] = &[
    Op {
        name: "MOV",
        num_args: 2,
        opcode: Opcode::Mov as u8
    },
    Op {
        name: "LOAD",
        num_args: 2,
        opcode: Opcode::Load as u8
    },
    Op {
        name: "STORE",
        num_args: 2,
        opcode: Opcode::Store as u8
    },
    Op {
        name: "PUSH",
        num_args: 1,
        opcode: Opcode::Push as u8
    },
    Op {
        name: "POP",
        num_args: 1,
        opcode: Opcode::Pop as u8
    },
    Op {
        name: "JNZ",
        num_args: 1,
        opcode: Opcode::Jnz as u8
    },
    Op {
        name: "ADD",
        num_args: 2,
        opcode: Opcode::Add as u8
    },
    Op {
        name: "SUB",
        num_args: 2,
        opcode: Opcode::Sub as u8
    },
    Op {
        name: "CALL",
        num_args: 1,
        opcode: Opcode::Call as u8
    },
    Op {
        name: "RET",
        num_args: 0,
        opcode: Opcode::Ret as u8
    },
    Op {
        name: "HLT",
        num_args: 0,
        opcode: Opcode::Hlt as u8
    },
    Op {
        name: "NOP",
        num_args: 0,
        opcode: Opcode::Nop as u8
    },
    Op {
        name: "MOVU",
        num_args: 2,
        opcode: Opcode::Movu as u8
    }
];

// Should be called when having a compiling error in the source file
pub fn asm_err(token: &Token, reason: &str) {
    println!("{} {}:{}: {}", token.o_file, token.o_line, token.o_index, reason);
}

// This is an alternative to asm_err(),
// where token isn't avaliable
pub fn asm_err_info(file: &str, line: usize, index: usize, reason: &str) {
    println!("{} {}:{}: {}", file, line, index, reason);
}


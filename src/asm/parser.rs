use crate::asm::asm::*;
use crate::arch::arch::*;
use crate::asm::lexer::{Token, TokenType};

fn parse_op(token: &Token) -> Option<&'static Op> {
    for op in OP_TABLE {
        if op.name == token.content {
            return Some(op);
        }
    }

    None
}

fn parse_register(token: &Token) -> Option<u8> {
    // Check if it's a number
    if matches!(token.t_type, TokenType::Number) {
        return Some(Register::IMM as u8);
    }

    match &*token.content {
        "A" => Some(Register::A as u8),
        "B" => Some(Register::B as u8),
        "C" => Some(Register::C as u8),
        "D" => Some(Register::D as u8),
        "E" => Some(Register::E as u8),
        "SP" => Some(Register::SP as u8),
        "PC" => Some(Register::PC as u8),
        _ => None
    }
}

pub fn parse(context: &mut Context) -> Result<(), AsmError> {
    let tokens = &mut context.tokens;
    let buffer = &mut context.buffer;

    let mut tokens = tokens.iter();
    while let Some(token) = tokens.next() {
        // Registers' values
        let mut src_reg: u8 = 0;
        let mut dst_reg: u8 = 0;
        let mut imm: u16 = 0;

        // Parse opcode
        let op = parse_op(token).ok_or(AsmError::InvalidToken)?;
        buff_push8(buffer, op.opcode);
        let token = tokens.next().ok_or(AsmError::UnexpectedEOF)?;

        // Parse registers
        if op.num_args == 1 {
            // [OP] [SRC]
            src_reg = parse_register(token).ok_or(AsmError::InvalidToken)?;
            if src_reg == Register::IMM as u8 {
                imm = token.number as u16;
            }
        }
        else if op.num_args == 2 {
            // [OP] [DST], [SRC]
            dst_reg = parse_register(token).ok_or(AsmError::InvalidToken)?;
            if dst_reg == Register::IMM as u8 {
                imm = token.number as u16;
            }

            let token = tokens.next().ok_or(AsmError::UnexpectedEOF)?;

            // Expects a comma
            if !matches!(token.t_type, TokenType::Comma) {
                return asm_err(token, AsmError::InvalidToken);
            }

            let token = tokens.next().ok_or(AsmError::UnexpectedEOF)?;

            src_reg = parse_register(token).ok_or(AsmError::InvalidToken)?;
            if src_reg == Register::IMM as u8 {
                imm = token.number as u16;
            }
        } 

        // Emit the instruction
        buff_push8(buffer, ((src_reg << 4) | dst_reg) as u8);
        buff_push16(buffer, imm);
    }

    Ok(())
}


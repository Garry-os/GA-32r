// Instructions opcode
#[repr(u8)]
#[derive(Debug, Default, Copy, Clone)]
pub enum Opcode {
    #[default]
    Mov   = 0x01,
    Load  = 0x02,
    Store = 0x03,
    Push  = 0x04,
    Pop   = 0x05,
    Jnz   = 0x06,
    Add   = 0x07,
    Sub   = 0x08,
    Call  = 0x09,
    Ret   = 0x0A,
    Hlt   = 0x0B,
    Nop   = 0x0C,
    Movu  = 0x0D
}

impl Opcode {
    pub fn from_u8(opcode: u8) -> Option<Self> {
        match opcode {
            0x01 => Some(Opcode::Mov),
            0x02 => Some(Opcode::Load),
            0x03 => Some(Opcode::Store),
            0x04 => Some(Opcode::Push),
            0x05 => Some(Opcode::Pop),
            0x06 => Some(Opcode::Jnz),
            0x07 => Some(Opcode::Add),
            0x08 => Some(Opcode::Sub),
            0x09 => Some(Opcode::Call),
            0x0A => Some(Opcode::Ret),
            0x0B => Some(Opcode::Hlt),
            0x0C => Some(Opcode::Nop),
            0x0D => Some(Opcode::Movu),
            _ => None
        }
    }
}

// Registers ID
#[repr(u8)]
#[derive(Debug, Default, Copy, Clone)]
pub enum Register {
    #[default]
    A   = 0x01,
    B   = 0x02,
    C   = 0x03,
    D   = 0x04,
    E   = 0x05,
    PC  = 0x06,
    SP  = 0x07,
    IMM = 0x08,
    Z   = 0x09
}

impl Register {
    pub fn from_u8(id: u8) -> Option<Self> {
        match id {
            0x01 => Some(Register::A),
            0x02 => Some(Register::B),
            0x03 => Some(Register::C),
            0x04 => Some(Register::D),
            0x05 => Some(Register::E),
            0x06 => Some(Register::PC),
            0x07 => Some(Register::SP),
            0x08 => Some(Register::IMM),
            0x09 => Some(Register::Z),
            _ => None
        }
    }
}

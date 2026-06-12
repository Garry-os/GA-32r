// Instructions opcode
#[repr(u8)]
pub enum Opcode {
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

// Registers ID
#[repr(u8)]
pub enum Register {
    A   = 0x01,
    B   = 0x02,
    C   = 0x03,
    D   = 0x04,
    E   = 0x05,
    PC  = 0x06,
    SP  = 0x07,
    IMM = 0x08
}


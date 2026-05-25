// Instructions opcode
const OP_MOV:   u8 = 0x01;
const OP_LOAD:  u8 = 0x02;
const OP_STORE: u8 = 0x03;
const OP_PUSH:  u8 = 0x04;
const OP_POP:   u8 = 0x05;
const OP_JNZ:   u8 = 0x06;
const OP_ADD:   u8 = 0x07;
const OP_SUB:   u8 = 0x08;
const OP_CALL:  u8 = 0x09;
const OP_RET:   u8 = 0x0A;
const OP_HLT:   u8 = 0x0B;
const OP_NOP:   u8 = 0x0C;
const OP_MOVU:  u8 = 0x0D;

// Registers
const REG_A:    u8 = 0x01;
const REG_B:    u8 = 0x02;
const REG_C:    u8 = 0x03;
const REG_D:    u8 = 0x04;
const REG_E:    u8 = 0x05;
const REG_PC:   u8 = 0x06;
const REG_SP:   u8 = 0x07;
const REG_IMM:  u8 = 0x08;


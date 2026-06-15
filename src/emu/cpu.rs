use crate::emu::bus::bus::Bus;
use crate::arch::arch::*;
use crate::emu::emu::*;

#[derive(Default, Debug)]
struct InstructionLayout {
    opcode: Opcode,
    src_reg: Register,
    dst_reg: Register,
    immediate: u16
}

impl InstructionLayout {
    fn decode(&mut self, instruction: u32) -> Result<(), EmuError> {
        self.opcode = Opcode::from_u8(((instruction >> 24) & 0xFF) as u8).ok_or(EmuError::InvalidOpcode)?;
        self.src_reg = Register::from_u8(((instruction >> 20) & 0x0F) as u8).ok_or(EmuError::InvalidRegister)?;
        self.dst_reg = Register::from_u8(((instruction >> 16) & 0x0F) as u8).ok_or(EmuError::InvalidRegister)?;
        self.immediate = (instruction & 0xFFFF) as u16;

        Ok(())
    }
}


#[derive(Default, Debug)]
pub struct Cpu {
    // General purpose registers
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
    // Jumper register
    pub e: u32,
    pub pc: u32, // Program counter
    pub sp: u32, // Stack pointer

    pub z: u8, // Cpu status (4-bit)
    status: EmuStatus
}

impl Cpu {
    pub fn reset(&mut self, bus: &mut Bus) {
        // Reset the registers
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.d = 0;

        self.e = 0;
        self.sp = 0x8FFFFFFF;

        self.z = 0;

        // Read the reset address
        self.pc = bus.read32(0xFFFFFFF0).unwrap_or(0x00);
    }

    fn read_register(&self, id: u8) -> Result<u32, EmuError> {
        // Get the actual enum
        let register = match Register::from_u8(id) {
            Some(reg) => reg,
            None => {
                eprintln!("Invalid register ID: {id}");
                return Err(EmuError::InvalidRegister);
            }
        };

        match register {
            Register::A => Ok(self.a),
            Register::B => Ok(self.b),
            Register::C => Ok(self.c),
            Register::D => Ok(self.d),
            Register::E => Ok(self.e),
            Register::PC => Ok(self.pc),
            Register::SP => Ok(self.sp),
            Register::Z => Ok(self.z as u32),
            Register::IMM => Err(EmuError::AttemptToAccessImm)
        }
    }

    fn write_register(&mut self, id: u8, value: u32) -> Result<(), EmuError> {
        // Get the actual enum
        let register = match Register::from_u8(id) {
            Some(reg) => reg,
            None => {
                eprintln!("Invalid register ID: {id}");
                return Err(EmuError::InvalidRegister);
            }
        };

        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::SP => self.sp = value,

            // Read only registers
            Register::PC => return Err(EmuError::ReadOnlyRegister),
            Register::Z => return Err(EmuError::ReadOnlyRegister),

            Register::IMM => return Err(EmuError::AttemptToAccessImm)
        };

        Ok(())
    }


    fn fetch8(&mut self, bus: &Bus) -> Option<u8> {
        let data: u8 = bus.read8(self.pc)?;
        self.pc += 1;

        Some(data)
    }

    fn fetch16(&mut self, bus: &Bus) -> Option<u16> {
        let high = self.fetch8(bus)?;
        let low = self.fetch8(bus)?;

        Some(((high as u16) << 8) | (low as u16))
    }

    fn fetch32(&mut self, bus: &Bus) -> Option<u32> {
        let high = self.fetch16(bus)?;
        let low = self.fetch16(bus)?;

        Some(((high as u32) << 16) | (low as u32))
    }

    pub fn step(&mut self, bus: &mut Bus) -> Result<(), EmuError> {
        let instruction_bytes: u32 = self.fetch32(bus).unwrap();

        // Decode instruction
        let mut instruction = InstructionLayout::default();
        instruction.decode(instruction_bytes)?;

        match instruction.opcode {
            Opcode::Mov => {
                if matches!(instruction.src_reg, Register::IMM) {
                    // Immediate register
                    self.write_register(instruction.dst_reg as u8, instruction.immediate as u32)?;
                }
                else {
                    // Normal register
                    self.write_register(instruction.dst_reg as u8, instruction.src_reg as u32)?;
                }
            },
            Opcode::Load => {
                todo!();
            }
            _ => ()
        };

        Ok(())
    }
}

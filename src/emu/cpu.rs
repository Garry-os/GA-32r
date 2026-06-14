use crate::emu::bus::bus::Bus;

#[derive(Default, Debug)]
struct InstructionLayout {
    opcode: u8,
    src_reg: u8,
    dst_reg: u8,
    immediate: u16
}

impl InstructionLayout {
    fn decode(&mut self, instruction: u32) {
        self.opcode = (instruction >> 24) as u8;
        self.src_reg = (instruction >> 20) as u8;
        self.dst_reg = (instruction >> 16) as u8;
        self.immediate = instruction as u16;
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

        // Read the reset address
        self.pc = bus.read32(0xFFFFFFF0).unwrap_or(0x00);
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

    pub fn step(&mut self, bus: &mut Bus) {
        let instruction_bytes: u32 = self.fetch32(bus).unwrap();

        // Decode instruction
        let mut instruction = InstructionLayout::default();
        instruction.decode(instruction_bytes);

        // Print out the layout
        println!("{:#?}", instruction);
    }
}

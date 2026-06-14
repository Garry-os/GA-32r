use crate::emu::bus::bus::Bus;

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
}

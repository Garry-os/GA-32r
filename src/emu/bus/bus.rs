use crate::emu::bus::ram::{Ram, RAM_SIZE};

pub struct Bus {
    pub ram: Ram
}

impl Bus {
    pub fn read8(self, address: u32) -> Option<u8> {
        if address <= RAM_SIZE as u32 {
            return Some(self.ram.data[address as usize]);
        }

        None
    }

    pub fn write8(&mut self, address: u32, value: u8) {
        if address <= RAM_SIZE as u32 {
            self.ram.data[address as usize] = value;
        }
    }
}


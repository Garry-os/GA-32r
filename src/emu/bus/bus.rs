use crate::emu::bus::ram::{Ram, RAM_SIZE};

#[derive(Default)]
pub struct Bus {
    pub ram: Ram,
    pub reset_vec: Box<[u8; 3]>
}

pub enum MemErr {
    InvalidAddress
}

impl Bus {
    pub fn read8(&self, address: u32) -> Option<u8> {
        match address as usize {
            0x0..RAM_SIZE => Some(self.ram.data[address as usize]),
            0xFFFFFFF0..0xFFFFFFF3 => Some(self.reset_vec[address as usize]),
            _ => None
        }
    }

    pub fn read16(&self, address: u32) -> Option<u16> {
        let high = self.read8(address)?;
        let low = self.read8(address + 1)?;

        Some(((high << 8) | low) as u16)
    }

    pub fn read32(&self, address: u32) -> Option<u32> {
        let high = self.read16(address)?;
        let low = self.read16(address + 2)?;

        Some(((high << 16) | low) as u32)
    }

    pub fn write8(&mut self, address: u32, value: u8) -> Result<(), MemErr> {
        match address as usize {
            0x0..RAM_SIZE => self.ram.data[address as usize] = value,
            0xFFFFFFF0..0xFFFFFFF3 => self.reset_vec[address as usize] = value,
            _ => return Err(MemErr::InvalidAddress)
        };

        Ok(())
    }

    pub fn write16(&mut self, address: u32, value: u16) -> Result<(), MemErr> {
        let high: u8 = (value >> 8) as u8;
        let low: u8 = value as u8;

        self.write8(address, high)?;
        self.write8(address + 1, low)?;

        Ok(())
    }

    pub fn write32(&mut self, address: u32, value: u32) -> Result<(), MemErr> {
        let high: u16 = (value >> 16) as u16;
        let low: u16 = value as u16;

        self.write16(address, high)?;
        self.write16(address + 2, low)?;

        Ok(())
    }
}


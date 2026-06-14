pub const RAM_SIZE: usize = 64 * 1024 * 1024; // 64 Mb

pub struct Ram {
    pub data: Vec<u8>
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            data: vec![0; RAM_SIZE]
        }
    }
}

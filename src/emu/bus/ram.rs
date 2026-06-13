pub const RAM_SIZE: usize = 64 * 1024 * 1024; // 64 Mb

pub struct Ram {
    pub data: Box<[u8; RAM_SIZE]>
}

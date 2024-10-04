#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Block {
    Air = 0,
    Grass = 1,
    Dirt = 2,
    Stone = 3,
}

impl Block {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

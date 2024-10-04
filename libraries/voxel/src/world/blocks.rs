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

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Air,
            1 => Self::Grass,
            2 => Self::Dirt,
            3 => Self::Stone,
            _ => Self::Air,
        }
    }
}

use crate::block::Block;

pub struct Chunk {
    blocks: [[[Block; 16]; 16]; 16],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[Block::Air; 16]; 16]; 16],
        }
    }
}

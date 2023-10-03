use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Block {
    Air,
    Dirt,
    Stone,
}

impl Block {
    pub fn uvs(&self) -> [Vec2; 4] {
        let (x, y) = match self {
            Block::Air => (0, 0),
            Block::Dirt => (2, 0),
            Block::Stone => (3, 0),
        };

        let offset = Vec2::new(x as f32, y as f32) / 16.;

        [
            offset + Vec2::new(0., 1. / 16.),
            offset + Vec2::new(0., 0.),
            offset + Vec2::new(1. / 16., 0.),
            offset + Vec2::new(1. / 16., 1. / 16.),
        ]
    }
}

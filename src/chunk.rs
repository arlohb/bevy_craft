use bevy::prelude::*;

use crate::{
    block::Block,
    mesh::{Direction, IncompleteMesh},
};

pub struct Chunk {
    blocks: [[[Block; 16]; 16]; 16],
}

impl Chunk {
    pub fn new() -> Self {
        let mut this = Self {
            blocks: [[[Block::Air; 16]; 16]; 16],
        };

        this.regenerate();

        this
    }

    pub fn regenerate(&mut self) {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let height = ((x + z) % 2) + 7;
                    let block = match y {
                        y if y < height => Block::Stone,
                        y if y == height => Block::Dirt,
                        _ => Block::Air,
                    };

                    self.blocks[x][y][z] = block;
                }
            }
        }
    }

    /// Try and get a block, returning None if outside bounds
    pub fn try_get(&self, x: usize, y: usize, z: usize) -> Option<Block> {
        self.blocks
            .get(x)
            .and_then(|arr| arr.get(y))
            .and_then(|arr| arr.get(z))
            .copied()
    }

    /// Tries to get a block, returning `Block::Air` if outside bounds
    pub fn get_or_air(&self, x: usize, y: usize, z: usize) -> Block {
        self.try_get(x, y, z).unwrap_or(Block::Air)
    }

    pub fn build_mesh(&self) -> Mesh {
        let mut incomplete_mesh = IncompleteMesh::default();

        for i in 0..16 {
            for j in 0..16 {
                incomplete_mesh.maybe_add_face(
                    Vec3::new(0., i as f32, j as f32),
                    Direction::Nx,
                    self.blocks[0][i][j],
                    Block::Air,
                );

                incomplete_mesh.maybe_add_face(
                    Vec3::new(i as f32, 0., j as f32),
                    Direction::Ny,
                    self.blocks[i][0][j],
                    Block::Air,
                );

                incomplete_mesh.maybe_add_face(
                    Vec3::new(i as f32, j as f32, 0.),
                    Direction::Nz,
                    self.blocks[i][j][0],
                    Block::Air,
                );
            }
        }

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let a = self.blocks[x][y][z];

                    let px = self.get_or_air(x + 1, y, z);
                    let py = self.get_or_air(x, y + 1, z);
                    let pz = self.get_or_air(x, y, z + 1);

                    let pos = Vec3::new(x as f32, y as f32, z as f32);
                    incomplete_mesh.maybe_add_face(pos, Direction::Px, a, px);
                    incomplete_mesh.maybe_add_face(pos, Direction::Py, a, py);
                    incomplete_mesh.maybe_add_face(pos, Direction::Pz, a, pz);
                }
            }
        }

        incomplete_mesh.complete()
    }
}

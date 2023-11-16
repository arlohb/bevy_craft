use bevy::prelude::*;
use noise::{Fbm, NoiseFn, SuperSimplex};

use crate::{
    block::Block,
    mesh::{Direction, IncompleteMesh},
};

#[derive(Resource)]
pub struct TerrainGen {
    pub height: Fbm<SuperSimplex>,
}

impl Default for TerrainGen {
    fn default() -> Self {
        let mut height = Fbm::new(0);
        height.octaves = 4;
        height.frequency = 0.04;
        height.lacunarity = 1.95;
        height.persistence = 0.40;

        Self { height }
    }
}

pub struct Chunk {
    id: IVec3,
    blocks: [[[Block; 16]; 16]; 16],
}

impl Chunk {
    /// This does not generate the chunk
    pub fn new(id: IVec3) -> Self {
        Self {
            id,
            blocks: [[[Block::Air; 16]; 16]; 16],
        }
    }

    pub fn generate(&mut self, terrain_gen: &TerrainGen) {
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let global_x = x as i32 + self.id.x * 16;
                    let global_y = y as i32 + self.id.y * 16;
                    let global_z = z as i32 + self.id.z * 16;

                    let height = (terrain_gen
                        .height
                        .get([global_x as f64, global_z as f64, 0.])
                        * 4.
                        + 8.) as i32;

                    let block = match global_y {
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
    pub fn try_get(&self, x: i32, y: i32, z: i32) -> Option<Block> {
        // If less than 0 make outside of bounds on other side
        let x = usize::try_from(x).unwrap_or(16);
        let y = usize::try_from(y).unwrap_or(16);
        let z = usize::try_from(z).unwrap_or(16);

        self.blocks
            .get(x)
            .and_then(|arr| arr.get(y))
            .and_then(|arr| arr.get(z))
            .copied()
    }

    /// Tries to get a block, returning `Block::Air` if outside bounds
    pub fn get_or_air(&self, x: i32, y: i32, z: i32) -> Block {
        self.try_get(x, y, z).unwrap_or(Block::Air)
    }

    /// Mutable gets a block
    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut Block {
        &mut self.blocks[x][y][z]
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

                    let px = self.get_or_air(x as i32 + 1, y as i32, z as i32);
                    let py = self.get_or_air(x as i32, y as i32 + 1, z as i32);
                    let pz = self.get_or_air(x as i32, y as i32, z as i32 + 1);

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

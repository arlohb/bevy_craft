use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::block::Block;

pub enum Direction {
    Px,
    Py,
    Pz,
    Nx,
    Ny,
    Nz,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [Px, Py, Pz, Nx, Ny, Nz].into_iter()
    }

    pub fn face_verts(&self) -> [(Vec3, Vec3); 4] {
        match self {
            Direction::Px => [
                (Vec3::new(1., 0., 1.), Vec3::new(1., 0., 0.)),
                (Vec3::new(1., 1., 1.), Vec3::new(1., 0., 0.)),
                (Vec3::new(1., 1., 0.), Vec3::new(1., 0., 0.)),
                (Vec3::new(1., 0., 0.), Vec3::new(1., 0., 0.)),
            ],
            Direction::Py => [
                (Vec3::new(0., 1., 0.), Vec3::new(0., 1., 0.)),
                (Vec3::new(1., 1., 0.), Vec3::new(0., 1., 0.)),
                (Vec3::new(1., 1., 1.), Vec3::new(0., 1., 0.)),
                (Vec3::new(0., 1., 1.), Vec3::new(0., 1., 0.)),
            ],
            Direction::Pz => [
                (Vec3::new(0., 0., 1.), Vec3::new(0., 0., 1.)),
                (Vec3::new(0., 1., 1.), Vec3::new(0., 0., 1.)),
                (Vec3::new(1., 1., 1.), Vec3::new(0., 0., 1.)),
                (Vec3::new(1., 0., 1.), Vec3::new(0., 0., 1.)),
            ],
            Direction::Nx => [
                (Vec3::new(0., 0., 0.), Vec3::new(-1., 0., 0.)),
                (Vec3::new(0., 1., 0.), Vec3::new(-1., 0., 0.)),
                (Vec3::new(0., 1., 1.), Vec3::new(-1., 0., 0.)),
                (Vec3::new(0., 0., 1.), Vec3::new(-1., 0., 0.)),
            ],
            Direction::Ny => [
                (Vec3::new(0., 0., 1.), Vec3::new(0., -1., 0.)),
                (Vec3::new(1., 0., 1.), Vec3::new(0., -1., 0.)),
                (Vec3::new(1., 0., 0.), Vec3::new(0., -1., 0.)),
                (Vec3::new(0., 0., 0.), Vec3::new(0., -1., 0.)),
            ],
            Direction::Nz => [
                (Vec3::new(1., 0., 0.), Vec3::new(0., 0., -1.)),
                (Vec3::new(1., 1., 0.), Vec3::new(0., 0., -1.)),
                (Vec3::new(0., 1., 0.), Vec3::new(0., 0., -1.)),
                (Vec3::new(0., 0., 0.), Vec3::new(0., 0., -1.)),
            ],
        }
    }
}

#[derive(Default)]
pub struct IncompleteMesh {
    // Splitting the pos, uv, and normals into separate vectors
    // avoids calls to map when inserting the attrs,
    // should benchmark in the future to verify
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,

    indices: Vec<u16>,
}

impl IncompleteMesh {
    pub fn add_face(&mut self, dir: Direction, block: Block) {
        for i in [0, 3, 1, 3, 2, 1] {
            self.indices.push(i + self.vertices.len() as u16);
        }

        for (v, n) in dir.face_verts() {
            self.vertices.push(v);
            self.normals.push(n);
        }

        for uv in block.uvs() {
            self.uvs.push(uv);
        }
    }

    pub fn complete(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.set_indices(Some(Indices::U16(self.indices)));
        mesh
    }
}

pub fn test_cube() -> Mesh {
    let mut incomplete_mesh = IncompleteMesh::default();

    for dir in Direction::iter() {
        incomplete_mesh.add_face(dir, Block::Dirt);
    }

    incomplete_mesh.complete()
}

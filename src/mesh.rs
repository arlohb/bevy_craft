use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use parry3d::na;

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
    pub fn add_face(&mut self, pos: Vec3, dir: Direction, invert: bool, block: Block) {
        let mut indices = [0, 3, 1, 3, 2, 1];
        if invert {
            indices.reverse();
        }
        for i in indices {
            self.indices.push(i + self.vertices.len() as u16);
        }

        for (v, n) in dir.face_verts() {
            self.vertices.push(v + pos);
            self.normals.push(n * if invert { -1. } else { 1. });
        }

        for uv in block.uvs() {
            self.uvs.push(uv);
        }
    }

    pub fn maybe_add_face(&mut self, pos: Vec3, dir: Direction, a: Block, b: Block) {
        let (block, invert) = match (a, b) {
            (Block::Air, Block::Air) => return,
            (Block::Air, b) => (b, true),
            (a, Block::Air) => (a, false),
            _ => return,
        };

        self.add_face(pos, dir, invert, block);
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

/// This may be able to be made faster.
/// `TriMesh` is really `GenericTriMesh<DefaultStorage>`,
/// so bevy's `Mesh` could be wrapped as a storage type instead.
pub fn mesh_to_tri_mesh(mesh: &Mesh) -> parry3d::shape::TriMesh {
    parry3d::shape::TriMesh::new(
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Mesh has not vertex positions")
            .as_float3()
            .expect("Mesh vertex positions are not float3s")
            .iter()
            .map(|[x, y, z]| na::Point3::new(*x, *y, *z))
            .collect::<Vec<_>>(),
        {
            let mut indices = mesh.indices().expect("Mesh has no indices").iter();
            let mut result = vec![];
            while let Some(first) = indices.next() {
                result.push([
                    first as u32,
                    indices.next().expect("Indices not a multiple of 3") as u32,
                    indices.next().expect("Indices not a multiple of 3") as u32,
                ]);
            }
            result
        },
    )
}

#[allow(dead_code)]
pub fn test_cube() -> Mesh {
    let mut incomplete_mesh = IncompleteMesh::default();

    for dir in Direction::iter() {
        incomplete_mesh.add_face(Vec3::ZERO, dir, false, Block::Dirt);
    }

    incomplete_mesh.complete()
}

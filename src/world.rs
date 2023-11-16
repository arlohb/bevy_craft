use bevy::{prelude::*, utils::HashMap};
use parry3d::{na, query::RayCast};

use crate::{
    block::Block,
    chunk::Chunk,
    mesh::{mesh_to_tri_mesh, Direction},
};

#[derive(Clone)]
pub struct WorldHit {
    pub chunk_id: IVec3,
    pub hit_pos: Vec3,
    pub dst: f32,
}

pub struct WorldTarget {
    pub local_pos: IVec3,
    pub block: Block,
}

#[derive(Resource)]
pub struct World {
    pub chunks: HashMap<IVec3, Chunk>,
    pub colliders: HashMap<IVec3, parry3d::shape::TriMesh>,
    pub meshes: HashMap<IVec3, Entity>,

    pub invalid_meshes: Vec<IVec3>,

    pub material: Handle<StandardMaterial>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::default(),
            colliders: HashMap::default(),
            meshes: HashMap::default(),

            invalid_meshes: vec![],

            material: Handle::default(),
        }
    }

    /// Returns the chunk pos and the hit distance
    pub fn cast_ray(&self, ray: &parry3d::query::Ray) -> Option<WorldHit> {
        self.colliders
            .iter()
            .map(|(chunk_id, tri_mesh)| {
                (
                    *chunk_id,
                    tri_mesh.cast_ray(
                        &na::Isometry3::from_parts(
                            na::Translation3::new(
                                16. * chunk_id.x as f32,
                                16. * chunk_id.y as f32,
                                16. * chunk_id.z as f32,
                            ),
                            na::UnitQuaternion::from_euler_angles(0., 0., 0.),
                        ),
                        ray,
                        10_000.,
                        true,
                    ),
                )
            })
            .filter_map(|(pos, maybe_dst)| Some(pos).zip(maybe_dst))
            .min_by(|(_, dst1), (_, dst2)| dst1.partial_cmp(dst2).expect("Invalid ray dst"))
            .map(|(chunk_id, dst)| WorldHit {
                chunk_id,
                dst,
                hit_pos: {
                    let p = ray.point_at(dst);
                    Vec3::new(p.x, p.y, p.z)
                },
            })
    }

    pub fn target_from_hit(&self, hit: WorldHit) -> Option<WorldTarget> {
        let x = hit.hit_pos.x - 16. * hit.chunk_id.x as f32;
        let y = hit.hit_pos.y - 16. * hit.chunk_id.y as f32;
        let z = hit.hit_pos.z - 16. * hit.chunk_id.z as f32;

        let dir = if (x + 0.0005) % 1. <= 0.001 {
            Direction::Px
        } else if (y + 0.0005) % 1. <= 0.001 {
            Direction::Py
        } else {
            Direction::Pz
        };

        let x = x.floor() as i32;
        let y = y.floor() as i32;
        let z = z.floor() as i32;

        let chunk = &self.chunks[&hit.chunk_id];

        let block1 = chunk.get_or_air(x, y, z);
        let (block2, block2pos) = match dir {
            Direction::Px => (chunk.get_or_air(x - 1, y, z), IVec3::new(x - 1, y, z)),
            Direction::Py => (chunk.get_or_air(x, y - 1, z), IVec3::new(x, y - 1, z)),
            _ => (chunk.get_or_air(x, y, z - 1), IVec3::new(x, y, z - 1)),
        };

        match (block1, block2) {
            (Block::Air, Block::Air) => None,
            (Block::Air, block2) => Some(WorldTarget {
                local_pos: block2pos,
                block: block2,
            }),
            (block1, Block::Air) => Some(WorldTarget {
                local_pos: IVec3::new(x, y, z),
                block: block1,
            }),
            _ => None,
        }
    }
}

pub fn world_mesh_gen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut world: ResMut<World>,
) {
    println!(
        "Regenerating {} meshes this frame",
        world.invalid_meshes.len()
    );

    for chunk_id in std::mem::take(&mut world.invalid_meshes) {
        // TODO For now this doesn't delete the mesh,
        // need to test whether it's auto removed
        if let Some(&chunk) = world.meshes.get(&chunk_id) {
            commands.entity(chunk).despawn();
        }

        let mesh = world.chunks[&chunk_id].build_mesh();
        world.colliders.insert(chunk_id, mesh_to_tri_mesh(&mesh));

        let mesh_handle = meshes.add(mesh);
        let entity = commands
            .spawn(PbrBundle {
                mesh: mesh_handle.clone(),
                material: world.material.clone(),
                transform: Transform::from_translation(16. * chunk_id.as_vec3()),
                ..default()
            })
            .id();

        world.meshes.insert(chunk_id, entity);
    }
}

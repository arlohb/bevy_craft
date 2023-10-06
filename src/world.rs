use bevy::{prelude::*, utils::HashMap};

use crate::chunk::Chunk;

#[derive(Resource)]
pub struct World {
    pub chunks: HashMap<IVec3, Chunk>,
    pub material: Handle<StandardMaterial>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::default(),
            material: Handle::default(),
        }
    }
}

#[derive(Component)]
pub struct ChunkMesh {
    mesh_handle: Handle<Mesh>,
}

pub fn mesh_cleanup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &ChunkMesh)>,
) {
    for (entity, chunk_mesh) in &query {
        meshes.remove(chunk_mesh.mesh_handle.clone());
        commands.entity(entity).despawn();
    }
}

pub fn world_mesh_gen(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, world: Res<World>) {
    for (pos, chunk) in &world.chunks {
        let mesh = chunk.build_mesh();
        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            PbrBundle {
                mesh: mesh_handle.clone(),
                material: world.material.clone(),
                transform: Transform::from_translation(16. * pos.as_vec3()),
                ..default()
            },
            ChunkMesh { mesh_handle },
        ));
    }
}

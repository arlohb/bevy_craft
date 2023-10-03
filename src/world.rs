use bevy::{prelude::*, utils::HashMap};

use crate::chunk::Chunk;

#[derive(Resource)]
pub struct World {
    chunks: HashMap<IVec3, Chunk>,
    pub mesh: Handle<Mesh>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::default(),
            mesh: Handle::default(),
        }
    }
}

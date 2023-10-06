mod block;
mod camera;
mod chunk;
mod mesh;
mod world;

use crate::world::World;
use bevy::prelude::*;
use chunk::Chunk;
use world::{mesh_cleanup, world_mesh_gen};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Mesh>::default())
        .add_plugins(camera::FlyCamPlugin)
        .add_systems(Startup, create_axis)
        .add_systems(Startup, setup)
        .add_systems(Update, entities_count)
        .add_systems(Update, world_mesh_gen)
        .add_systems(Update, mesh_cleanup.before(world_mesh_gen))
        .insert_resource(World::new())
        .run();
}

fn entities_count(world: &bevy::prelude::World, meshes: Res<Assets<Mesh>>) {
    println!(
        "Entities: {}, Meshes: {}",
        world.entities().len(),
        meshes.len()
    );
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world: ResMut<World>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 20_000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1., 2., 0.2).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });

    let world_tex: Handle<Image> = asset_server.load("./Texture.png");
    world.material = materials.add(StandardMaterial {
        base_color_texture: Some(world_tex),
        ..default()
    });

    for x in -5..=5 {
        for z in -5..=5 {
            world.chunks.insert(IVec3::new(x, 0, z), Chunk::new());
        }
    }
}

fn create_axis(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::new(1.).into()),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(5., 0., 0.).with_scale(Vec3::new(10., 1., 1.)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::new(1.).into()),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0., 5., 0.).with_scale(Vec3::new(1., 10., 1.)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::new(1.).into()),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(0., 0., 5.).with_scale(Vec3::new(1., 1., 10.)),
        ..default()
    });
}

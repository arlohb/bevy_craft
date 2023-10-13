use crate::{block::Block, world::World};
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use parry3d::na;

#[derive(Component)]
pub struct FlyCam {
    pub move_speed: f32,
    pub sprint_mod: f32,
    pub look_speed: f32,
}

impl Default for FlyCam {
    fn default() -> Self {
        Self {
            move_speed: 1.,
            sprint_mod: 3.,
            look_speed: 0.003,
        }
    }
}

#[derive(Default)]
pub struct FlyCamPlugin;

impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::movement)
            .add_systems(Update, Self::rotate)
            .add_systems(Update, Self::pointer);
    }
}

impl FlyCamPlugin {
    fn setup(mut commands: Commands) {
        commands.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(120., 40., 120.)
                    .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
                ..default()
            },
            FlyCam::default(),
        ));
    }

    fn create_ray(transform: &Transform) -> parry3d::query::Ray {
        let origin = transform.translation;
        let direction = transform.forward();

        parry3d::query::Ray::new(
            na::Point3::new(origin.x, origin.y, origin.z),
            na::Vector3::new(direction.x, direction.y, direction.z),
        )
    }

    pub fn pointer(
        query: Query<&Transform, With<FlyCam>>,
        mouse_btns: Res<Input<MouseButton>>,
        mut world: ResMut<World>,
    ) {
        let transform = query
            .get_single()
            .expect("None / more than 1 camera present");

        if mouse_btns.pressed(MouseButton::Left) {
            let ray = Self::create_ray(transform);

            if let Some((hit, target)) = world
                .cast_ray(&ray)
                .and_then(|world_hit| Some(world_hit.clone()).zip(world.target_from_hit(world_hit)))
            {
                let block = world.chunks.get_mut(&hit.chunk_id).unwrap().get_mut(
                    target.local_pos.x as usize,
                    target.local_pos.y as usize,
                    target.local_pos.z as usize,
                );

                *block = Block::Air;
            }
        }
    }

    fn movement(mut query: Query<(&mut Transform, &FlyCam)>, keys: Res<Input<KeyCode>>) {
        for (mut transform, fly_cam) in &mut query {
            let move_speed = if keys.pressed(KeyCode::ShiftLeft) {
                fly_cam.move_speed * fly_cam.sprint_mod
            } else {
                fly_cam.move_speed
            };

            if keys.pressed(KeyCode::W) {
                let forward = transform.forward();
                transform.translation += forward * move_speed;
            }

            if keys.pressed(KeyCode::S) {
                let forward = transform.forward();
                transform.translation -= forward * move_speed;
            }

            if keys.pressed(KeyCode::D) {
                let right = transform.right();
                transform.translation += right * move_speed;
            }

            if keys.pressed(KeyCode::A) {
                let right = transform.right();
                transform.translation -= right * move_speed;
            }

            if keys.pressed(KeyCode::E) {
                let up = transform.up();
                transform.translation += up * move_speed;
            }

            if keys.pressed(KeyCode::Q) {
                let up = transform.up();
                transform.translation -= up * move_speed;
            }
        }
    }

    fn rotate(
        mut query: Query<(&mut Transform, &FlyCam)>,
        mut windows: Query<&mut Window, With<PrimaryWindow>>,
        mouse_btns: Res<Input<MouseButton>>,
        mut mouse_motion: EventReader<MouseMotion>,
    ) {
        let Ok(mut window) = windows.get_single_mut() else { return; };

        for (mut transform, fly_cam) in &mut query {
            if !mouse_btns.pressed(MouseButton::Right) {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;

                mouse_motion.clear();
                continue;
            }

            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;

            for &MouseMotion {
                delta: Vec2 { x, y },
            } in mouse_motion.iter()
            {
                let right = transform.right();
                transform.rotate_axis(right, -y * fly_cam.look_speed);

                transform.rotate_y(-x * fly_cam.look_speed);
            }
        }
    }
}

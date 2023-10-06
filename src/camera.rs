use bevy::{input::mouse::MouseMotion, prelude::*};

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
            look_speed: 0.01,
        }
    }
}

#[derive(Default)]
pub struct FlyCamPlugin;

impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .add_systems(Update, Self::movement)
            .add_systems(Update, Self::rotate);
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

    fn movement(
        mut query: Query<(&mut Transform, &FlyCam), With<Camera3d>>,
        keys: Res<Input<KeyCode>>,
    ) {
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
        mut query: Query<(&mut Transform, &FlyCam), With<Camera3d>>,
        mouse_btns: Res<Input<MouseButton>>,
        mut mouse_motion: EventReader<MouseMotion>,
    ) {
        for (mut transform, fly_cam) in &mut query {
            if !mouse_btns.pressed(MouseButton::Right) {
                mouse_motion.clear();
                continue;
            }

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

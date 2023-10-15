use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_egui::{egui, EguiContexts};
use noise::{Fbm, NoiseFn, SuperSimplex};

pub struct NoiseDebugPlugin;

impl Plugin for NoiseDebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NoiseDebugState::default())
            .add_systems(Update, create_image_system)
            .add_systems(Update, noise_debug_system.after(create_image_system));
    }
}

#[derive(Resource)]
pub struct NoiseDebugState {
    noise_image: Handle<Image>,
    width: usize,
    height: usize,
    fbm: Fbm<SuperSimplex>,
}

impl Default for NoiseDebugState {
    fn default() -> Self {
        Self {
            noise_image: Handle::default(),
            width: 512,
            height: 512,
            fbm: Fbm::new(0),
        }
    }
}

pub fn create_image_system(mut state: ResMut<NoiseDebugState>, mut images: ResMut<Assets<Image>>) {
    if !images.contains(&state.noise_image) {
        let pixel = [1., 0., 0., 1.];

        state.noise_image = images.add(Image::new_fill(
            Extent3d {
                width: state.width as u32,
                height: state.height as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &pixel
                .into_iter()
                .flat_map(f32::to_le_bytes)
                .collect::<Vec<_>>(),
            TextureFormat::Rgba32Float,
        ));
    }
}

pub fn noise_debug_system(
    mut ctx: EguiContexts,
    mut state: ResMut<NoiseDebugState>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = images.get_mut(&state.noise_image).unwrap();

    let mut write_pixel = |x: usize, y: usize, r: f32, g: f32, b: f32| {
        let pixel_i = y * state.width + x;
        let i = pixel_i * 16;

        let r = r.to_le_bytes();
        image.data[i] = r[0];
        image.data[i + 1] = r[1];
        image.data[i + 2] = r[2];
        image.data[i + 3] = r[3];

        let g = g.to_le_bytes();
        image.data[i + 4] = g[0];
        image.data[i + 5] = g[1];
        image.data[i + 6] = g[2];
        image.data[i + 7] = g[3];

        let b = b.to_le_bytes();
        image.data[i + 8] = b[0];
        image.data[i + 9] = b[1];
        image.data[i + 10] = b[2];
        image.data[i + 11] = b[3];

        let a = 1f32.to_le_bytes();
        image.data[i + 12] = a[0];
        image.data[i + 13] = a[1];
        image.data[i + 14] = a[2];
        image.data[i + 15] = a[3];
    };

    for x in 0..state.width {
        for y in 0..state.height {
            let v = state.fbm.get([x as f64, y as f64]);
            let v = v as f32 / 2. + 0.5;

            write_pixel(x, y, v, v, v);
        }
    }

    let tex = ctx.add_image(state.noise_image.clone());

    egui::Window::new("Noise Debug").show(ctx.ctx_mut(), |ui| {
        egui::Grid::new("noise_debug_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Octaves");
                ui.add(
                    egui::DragValue::new(&mut state.fbm.octaves)
                        .speed(0.5)
                        .clamp_range(1..=6),
                );
                ui.end_row();

                ui.label("Frequency");
                ui.add(
                    egui::DragValue::new(&mut state.fbm.frequency)
                        .speed(0.01)
                        .clamp_range(0..=1),
                );
                ui.end_row();

                ui.label("Lacunarity");
                ui.add(egui::DragValue::new(&mut state.fbm.lacunarity).speed(0.05));
                ui.end_row();

                ui.label("Persistence");
                ui.add(egui::DragValue::new(&mut state.fbm.persistence).speed(0.05));
                ui.end_row();
            });

        ui.image(egui::load::SizedTexture::new(tex, [256., 256.]));
    });
}

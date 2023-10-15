use bevy::diagnostic::{Diagnostic, DiagnosticId, Diagnostics, DiagnosticsStore};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct CustomDiagnosticsPlugin;

impl Plugin for CustomDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_system)
            .add_systems(Update, Self::diagnostic_system);
    }
}

impl CustomDiagnosticsPlugin {
    /// Gets unique id of this diagnostic.
    ///
    /// The diagnostic id is the type uuid of `T`.
    pub fn diagnostic_id() -> DiagnosticId {
        DiagnosticId::from_u128(311626136719528359392517333749239739456)
    }

    /// Registers the asset count diagnostic for the current application.
    pub fn setup_system(mut diagnostics: ResMut<DiagnosticsStore>) {
        diagnostics.add(Diagnostic::new(Self::diagnostic_id(), "entity_count", 20));
    }

    /// Updates the asset count of `T` assets.
    pub fn diagnostic_system(mut diagnostics: Diagnostics, world: &World) {
        diagnostics.add_measurement(Self::diagnostic_id(), || world.entities().len() as f64);
    }
}

pub fn egui_diagnostics(mut ctx: EguiContexts, diagnostics: Res<DiagnosticsStore>) {
    egui::Window::new("Diagnostics").show(ctx.ctx_mut(), |ui| {
        egui::Grid::new("diagnostics_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                for diagnostic in diagnostics.iter() {
                    ui.label(diagnostic.name.to_string());

                    let value = diagnostic
                        .smoothed()
                        .unwrap_or_else(|| diagnostic.value().unwrap_or(-1.));
                    ui.label(format!("{:.2} {}", value, diagnostic.suffix));

                    ui.end_row();
                }
            });
    });
}

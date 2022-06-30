use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.insert_resource(WorldInspectorParams {
                enabled: false,
                ..Default::default()
            });
            app.add_system(toggle_inspector);
            app.add_plugin(WorldInspectorPlugin::new());
        }
    }
}

pub fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled;
    }
}

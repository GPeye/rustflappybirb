use bevy::prelude::*;

mod window_config;
use window_config::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let window_config = WindowConfig::new();

        app.insert_resource(WindowDescriptor {
            title: window_config.title,
            width: window_config.width,
            height: window_config.height,
            resizable: window_config.resizable,
            ..default()
        });
    }
}

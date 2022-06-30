use bevy::asset::AssetServerSettings;
use bevy::prelude::*;

mod window_config;
use window_config::*;

pub mod game_config;
pub use game_config::*;

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

        if window_config.hot_reload {
            app.insert_resource(AssetServerSettings {
                watch_for_changes: true,
                ..default()
            });
        }

        app.add_plugins(DefaultPlugins);
        app.add_plugin(GameConfigPlugin);
    }
}

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_asset_ron::RonAssetPlugin;

const GAME_CONFIG: &str = "assets/configs/game_config.ron";

#[derive(serde::Deserialize, TypeUuid, Debug, Default, Copy, Clone)]
#[uuid = "ab1355b1-3412-4038-8555-d72a46bfbad0"]
pub struct GameConfigAsset {
    pub gravity: f32,
    pub speed: f32,
    pub jump_velocity: f32,
}

impl GameConfigAsset {
    pub fn new() -> Self {
        let ron_option = &std::fs::read_to_string(GAME_CONFIG);
        let ron_str = match ron_option {
            Ok(v) => v,
            Err(_) => "",
        };

        let config = if ron_str == "" {
            Self::default()
        } else {
            match ron::from_str::<GameConfigAsset>(ron_str) {
                Ok(v) => v,
                Err(_) => {
                    error!("Failed to load window_config.ron");
                    Self::default()
                }
            }
        };
        config
    }

    fn default() -> Self {
        Self {
            gravity: -300.,
            speed: 2.,
            jump_velocity: 300.,
        }
    }
}

#[derive(Default)]
struct GameConfigHandle(Handle<GameConfigAsset>);

#[derive(Default, Debug)]
pub struct GameConfig(pub GameConfigAsset);

pub struct GameConfigPlugin;

impl Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(GameConfigHandle::default());
        app.add_plugin(RonAssetPlugin::<GameConfigAsset>::new(&["ron"]));
        app.add_startup_system(setup);
        app.add_system(listen_for_config_changes);
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let config_ron: Handle<GameConfigAsset> = asset_server.load("configs/game_config.ron");
    commands.insert_resource(GameConfigHandle(config_ron.clone()));
    commands.insert_resource(GameConfig(GameConfigAsset::new()));
}

fn listen_for_config_changes(
    mut ev_asset: EventReader<AssetEvent<GameConfigAsset>>,
    assets: Res<Assets<GameConfigAsset>>,
    game_config_handle: Res<GameConfigHandle>,
    mut game_config: ResMut<GameConfig>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                if *handle == game_config_handle.0 {
                    info!("modified");
                    let gc = assets.get(handle).unwrap();
                    info!("{:?}", gc);
                    game_config.0 = *gc;
                }
            }
            _ => {}
        }
    }
}

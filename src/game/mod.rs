mod debug_config;
use debug_config::DebugPlugin;

mod level;
use level::LevelPlugin;

mod player;
pub use player::*;

mod physics;
pub use physics::*;

mod state;
pub use state::*;

mod score;
pub use score::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)));

        //#[cfg(debug_assertions)]
        app.add_plugin(DebugPlugin);

        app.add_plugin(ScorePlugin);

        app.add_plugin(LevelPlugin);

        app.add_plugin(PlayerPlugin);
    }
}

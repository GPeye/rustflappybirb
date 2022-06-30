use bevy::prelude::*;

mod configs;
use configs::*;

mod game;
pub use game::GamePlugin;

fn main() {
    let mut app = App::new();

    app.add_plugin(ConfigPlugin);
    app.add_plugins(DefaultPlugins);

    app.add_plugin(GamePlugin);

    app.run();
}

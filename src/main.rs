use bevy::prelude::*;

pub mod configs;
pub use configs::*;

mod game;
pub use game::GamePlugin;

fn main() {
    let mut app = App::new();

    app.add_plugin(ConfigPlugin);

    app.add_plugin(GamePlugin);

    app.run();
}

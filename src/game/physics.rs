use crate::game::score::ScoreEvent;
use crate::game::state::AppState;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(PhysicsLayer, Debug)]
pub enum Layer {
    World,
    Player,
    Goal,
}

pub fn detect_collisions(
    mut events: EventReader<CollisionEvent>,
    mut app_state: ResMut<State<AppState>>,
    mut score_writer: EventWriter<ScoreEvent>,
    audio: Res<Audio>,
    asset_server: ResMut<AssetServer>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {
                if d1.collision_layers().contains_group(Layer::Goal)
                    || d2.collision_layers().contains_group(Layer::Goal)
                {
                    score_writer.send(ScoreEvent);
                    let sound = asset_server.load("sound/point.ogg");
                    audio.play(sound);
                } else {
                    app_state.push(AppState::GameOver).unwrap();
                    let sound = asset_server.load("sound/die.ogg");
                    audio.play(sound);
                    let sound2 = asset_server.load("sound/hit.ogg");
                    audio.play(sound2);
                }
            }
            CollisionEvent::Stopped(d1, d2) => {
                println!("Collision stopped between {:?} and {:?}", d1, d2);
            }
        }
    }
}

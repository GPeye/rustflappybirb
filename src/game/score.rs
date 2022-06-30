use crate::game::state::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

pub struct Score(pub i32);
pub struct ScorePlugin;
pub struct ScoreEvent;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.add_event::<ScoreEvent>();
        app.add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(clean.before(setup))
                .with_system(setup),
        );
        app.add_system_set(SystemSet::on_update(AppState::Playing).with_system(update_score));
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    setup_score_ui(&mut commands, &asset_server);
}

fn clean(mut commands: Commands, mut q: Query<Entity, With<ScoreText>>, mut score: ResMut<Score>) {
    for score_text in q.iter_mut() {
        commands.entity(score_text).despawn_recursive();
    }
    score.0 = 0;
}

fn setup_score_ui(commands: &mut Commands, asset_server: &ResMut<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 50.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(ScoreText);
}

fn update_score(
    mut reader: EventReader<ScoreEvent>,
    mut score: ResMut<Score>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
) {
    if reader.iter().next().is_some() {
        score.0 += 1;
        text_query.single_mut().sections[1].value = format!("{:?}", score.0);
    }
}

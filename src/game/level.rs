use bevy::audio::AudioSink;
use bevy::prelude::*;
use heron::prelude::*;
use rand::prelude::random;

use crate::game::physics::*;
use crate::game::state::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity::from(Vec3::new(0.0, -360.1, 0.0)));
        app.insert_resource(MusicController::default());

        app.add_plugin(PhysicsPlugin::default());
        app.add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(clean_level.before(setup_level))
                .with_system(setup_level),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(update_pipes)
                .with_system(update_background)
                .with_system(detect_collisions),
        );
        app.add_system_set(SystemSet::on_update(AppState::GameOver).with_system(reset_state));
        app.add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(setup_gameover));
        //app.add_system(reset_state);

        app.add_state(AppState::Playing);
    }
}

#[derive(Default)]
struct MusicController(Handle<AudioSink>);

fn setup_level(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let music = asset_server.load("sound/theme.ogg");
    let handle = audio_sinks.get_handle(audio.play(music));
    commands.insert_resource(MusicController(handle));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    spawn_background(&mut commands, &asset_server);
    spawn_pipe_pair(&mut commands, &asset_server);
}

fn clean_level(
    mut commands: Commands,
    q: Query<Entity>,
    audio_sinks: ResMut<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
) {
    if let Some(sink) = audio_sinks.get(&music_controller.0) {
        sink.stop();
    }
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
struct Background;

fn spawn_background(commands: &mut Commands, asset_server: &ResMut<AssetServer>) {
    let background_image = asset_server.load("pictures/background.png");
    let starting_x = -420.;
    let increment = 420.;

    for n in 0..5 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: background_image.clone(),
                transform: Transform {
                    translation: Vec3::new(starting_x + (n as f32 * increment), 0.0, 0.0),
                    scale: Vec3::new(1.5, 1.5, 0.0),
                    ..default()
                },
                ..Default::default()
            })
            .insert(Background);
    }
}

fn update_background(mut backgrounds: Query<&mut Transform, With<Background>>) {
    for mut transform in backgrounds.iter_mut() {
        transform.translation.x -= 1.;

        if transform.translation.x < -840. {
            transform.translation.x = 840.
        }
    }
}

#[derive(Component)]
pub struct PipePair;

fn spawn_pipe_pair(commands: &mut Commands, asset_server: &ResMut<AssetServer>) {
    let width = 25.;
    let offset = 300.;
    let spacing = 342.;

    let pipe = asset_server.load("pictures/pipe-green.png");

    for n in 0..3 {
        commands
            .spawn()
            .insert(PipePair)
            .insert(Transform::from_translation(Vec3::new(
                offset + (n as f32 * (spacing + width)),
                pipe_random_y() - 250.,
                1.0,
            )))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                // Top Pipe
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: pipe.clone(),
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(50.0, 600.0)),
                            flip_y: true,
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 375.0, 0.0)),
                        ..Default::default()
                    })
                    .insert(RigidBody::Static)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec2::new(50.0, 600.0).extend(0.0) / 2.0,
                        border_radius: None,
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_group(Layer::World)
                            .with_mask(Layer::Player),
                    );

                // Center Goal
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(50.0, 100.0)),
                            ..Default::default()
                        },
                        //visibility: Visibility { is_visible: false },
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                        visibility: Visibility { is_visible: false },
                        ..Default::default()
                    })
                    .insert(RigidBody::Sensor)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec2::new(50.0, 100.0).extend(0.0) / 2.0,
                        border_radius: None,
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_group(Layer::Goal)
                            .with_mask(Layer::Player),
                    );

                // Bottom Pipe
                parent
                    .spawn_bundle(SpriteBundle {
                        texture: pipe.clone(),
                        sprite: Sprite {
                            color: Color::GREEN,
                            custom_size: Some(Vec2::new(50.0, 600.0)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, -375.0, 0.0)),
                        ..Default::default()
                    })
                    .insert(RigidBody::Static)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec2::new(50.0, 600.0).extend(0.0) / 2.0,
                        border_radius: None,
                    })
                    .insert(
                        CollisionLayers::none()
                            .with_group(Layer::World)
                            .with_mask(Layer::Player),
                    );
            });
    }
}

fn pipe_random_y() -> f32 {
    random::<f32>() * 500.
}

fn update_pipes(mut pipes: Query<&mut Transform, With<PipePair>>) {
    for mut transform in pipes.iter_mut() {
        transform.translation.x -= 2.;

        if transform.translation.x < -537. {
            transform.translation.x = 537.;
            transform.translation.y = pipe_random_y() - 250.;
        }
    }
}

fn setup_gameover(mut commands: Commands, asset_server: Res<AssetServer>) {
    let gameover_image = asset_server.load("pictures/gameover.png");
    commands
        .spawn_bundle(SpriteBundle {
            texture: gameover_image.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 4.0),
                scale: Vec3::new(1.5, 1.5, 0.0),
                ..default()
            },
            ..Default::default()
        })
        .insert(Background);
}

use crate::game::physics::*;
use crate::game::state::*;
use bevy::prelude::*;
use heron::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<PlayerSpriteHandles>(PlayerSpriteHandles::default());
        app.add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(clean.before(setup))
                .with_system(setup),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_system(jump)
                .with_system(animate),
        );
        app.add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Default)]
struct PlayerSpriteHandles {
    upflap: Handle<Image>,
    midflap: Handle<Image>,
    downflap: Handle<Image>,
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut handles: ResMut<PlayerSpriteHandles>,
) {
    spawn_player(&mut commands, &asset_server, &mut handles);
}

fn clean(mut commands: Commands, mut q: Query<Entity, With<Player>>) {
    for player in q.iter_mut() {
        commands.entity(player).despawn_recursive();
    }
}

fn spawn_player(
    commands: &mut Commands,
    asset_server: &ResMut<AssetServer>,
    handles: &mut ResMut<PlayerSpriteHandles>,
) {
    handles.upflap = asset_server.load("pictures/yellowbird-upflap.png");
    handles.midflap = asset_server.load("pictures/yellowbird-midflap.png");
    handles.downflap = asset_server.load("pictures/yellowbird-downflap.png");
    commands
        .spawn_bundle(SpriteBundle {
            texture: handles.upflap.clone(),
            transform: Transform {
                translation: Vec3::new(-250.0, 0.0, 2.0),
                ..default()
            },
            global_transform: GlobalTransform::default(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(30.0, 30.0).extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(RotationConstraints::lock())
        .insert(Velocity::default())
        .insert(Player)
        .insert(
            CollisionLayers::none()
                .with_group(Layer::Player)
                .with_mask(Layer::World)
                .with_mask(Layer::Goal),
        );

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-250.0, -410.0, 0.0)),
            ..Default::default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec2::new(50.0, 50.0).extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(Layer::World)
                .with_mask(Layer::Player),
        );
}

fn jump(
    mouse_button_input: ResMut<Input<MouseButton>>,
    mut players: Query<&mut Velocity, With<Player>>,
    audio: Res<Audio>,
    asset_server: ResMut<AssetServer>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut velocity = players.single_mut();
        velocity.linear = Vec2::new(0.0, 1.0).normalize_or_zero().extend(0.0) * 300.0;
        let sound = asset_server.load("sound/wing.ogg");
        audio.play(sound);
    }
}

fn animate(
    mut player_velocity: Query<&mut Velocity, With<Player>>,
    mut player_image: Query<&mut Handle<Image>, With<Player>>,
    handles: Res<PlayerSpriteHandles>,
) {
    let velocity = player_velocity.single_mut();
    let mut image = player_image.single_mut();
    if velocity.linear.y > 10. {
        *image = handles.downflap.clone();
    } else if velocity.linear.y < 10. && velocity.linear.y > -10. {
        *image = handles.midflap.clone();
    } else {
        *image = handles.upflap.clone();
    }
}

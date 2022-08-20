use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

pub const SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Player;

#[derive(Default, Deref, DerefMut)]
pub struct InputDirection(Vec2);

pub fn init(mut cmd: Commands) {
    cmd.spawn_bundle((
        Player,
        RigidBody::KinematicVelocityBased,
        Velocity::default(),
        Collider::ball(16.0),
    ))
    .insert_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::from_array([32.0, 64.0])),
            anchor: Anchor::Custom(Vec2::from_array([0.0, -0.25])),
            ..default()
        },
        ..default()
    });
}

pub fn movement(
    mut q_player: Query<&mut Velocity, With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut input_direction: ResMut<InputDirection>,
) {
    let mut player_vel = q_player.single_mut();

    // Prevent stopping on SOCD
    if keys.just_pressed(KeyCode::A) {
        input_direction.x = -1.0;
    }
    if keys.just_pressed(KeyCode::D) {
        input_direction.x = 1.0;
    }
    if keys.just_pressed(KeyCode::W) {
        input_direction.y = 1.0;
    }
    if keys.just_pressed(KeyCode::S) {
        input_direction.y = -1.0;
    }

    if keys.just_released(KeyCode::A) {
        if keys.pressed(KeyCode::D) {
            input_direction.x = 1.0;
        } else {
            input_direction.x = 0.0;
        }
    }
    if keys.just_released(KeyCode::D) {
        if keys.pressed(KeyCode::A) {
            input_direction.x = -1.0;
        } else {
            input_direction.x = 0.0;
        }
    }
    if keys.just_released(KeyCode::W) {
        if keys.pressed(KeyCode::S) {
            input_direction.y = -1.0;
        } else {
            input_direction.y = 0.0;
        }
    }
    if keys.just_released(KeyCode::S) {
        if keys.pressed(KeyCode::W) {
            input_direction.y = 1.0;
        } else {
            input_direction.y = 0.0;
        }
    }

    player_vel.linvel = input_direction.normalize_or_zero() * SPEED;
}

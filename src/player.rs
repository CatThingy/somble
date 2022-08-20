use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

pub const SPEED: f32 = 300.0;
const IDLE_ANIM_OFFSET: usize = 0;
const WALK_ANIM_OFFSET: usize = 4;
const WALK_ANIM_FRAMES: usize = 2;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Default, Deref, DerefMut)]
pub struct InputDirection(Vec2);

#[derive(Default, Deref, DerefMut)]
pub struct PlayerDirection(IVec2);

pub fn init(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player.png");
    let atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 32.0), 12, 1);
    let texture_atlas = texture_atlases.add(atlas);
    cmd.spawn_bundle((
        Player,
        RigidBody::KinematicVelocityBased,
        Velocity::default(),
        Collider::ball(16.0),
        AnimationTimer(Timer::from_seconds(0.25, true)),
    ))
    .insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(Vec2::from_array([32.0, 64.0])),
            anchor: Anchor::Custom(Vec2::from_array([0.0, -0.25])),
            ..default()
        },
        texture_atlas,
        ..default()
    });
}

pub fn movement(
    mut q_player: Query<&mut Velocity, With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut input_direction: ResMut<InputDirection>,
    mut player_direction: ResMut<PlayerDirection>,
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

    if **input_direction != Vec2::ZERO {
        **player_direction = input_direction.as_ivec2();
    }
}

pub fn update_player_animation(
    mut q_player: Query<(&mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>,
    time: Res<Time>,
    player_direction: Res<PlayerDirection>,
    input_direction: Res<InputDirection>,
) {
    let (mut timer, mut sprite) = q_player.single_mut();

    if input_direction.is_changed() {
        if **input_direction != Vec2::ZERO {
            match **player_direction {
                IVec2 { x: _, y: 1 } => sprite.index = WALK_ANIM_OFFSET + WALK_ANIM_FRAMES * 0,
                IVec2 { x: 1, y: _ } => sprite.index = WALK_ANIM_OFFSET + WALK_ANIM_FRAMES * 1,
                IVec2 { x: -1, y: _ } => sprite.index = WALK_ANIM_OFFSET + WALK_ANIM_FRAMES * 2,
                IVec2 { x: _, y: -1 } => sprite.index = WALK_ANIM_OFFSET + WALK_ANIM_FRAMES * 3,
                _ => (),
            }
        } else {
            match **player_direction {
                IVec2 { x: _, y: 1 } => sprite.index = IDLE_ANIM_OFFSET + 0,
                IVec2 { x: 1, y: _ } => sprite.index = IDLE_ANIM_OFFSET + 1,
                IVec2 { x: -1, y: _ } => sprite.index = IDLE_ANIM_OFFSET + 2,
                IVec2 { x: _, y: -1 } => sprite.index = IDLE_ANIM_OFFSET + 3,
                _ => (),
            }
        }
    }
    if **input_direction != Vec2::ZERO {
        timer.tick(time.delta());

        if timer.just_finished() {
            match **player_direction {
                IVec2 { x: _, y: 1 } => {
                    sprite.index = WALK_ANIM_OFFSET
                        + WALK_ANIM_FRAMES * 0
                        + ((sprite.index - WALK_ANIM_OFFSET + 1) % WALK_ANIM_FRAMES);
                }
                IVec2 { x: 1, y: _ } => {
                    sprite.index = WALK_ANIM_OFFSET
                        + WALK_ANIM_FRAMES * 1
                        + ((sprite.index - WALK_ANIM_OFFSET + 1) % WALK_ANIM_FRAMES);
                }
                IVec2 { x: -1, y: _ } => {
                    sprite.index = WALK_ANIM_OFFSET
                        + WALK_ANIM_FRAMES * 2
                        + ((sprite.index - WALK_ANIM_OFFSET + 1) % WALK_ANIM_FRAMES);
                }
                IVec2 { x: _, y: -1 } => {
                    sprite.index = WALK_ANIM_OFFSET
                        + WALK_ANIM_FRAMES * 3
                        + ((sprite.index - WALK_ANIM_OFFSET + 1) % WALK_ANIM_FRAMES);
                }
                _ => (),
            }
        }
    }
}

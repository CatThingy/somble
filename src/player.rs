use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

use crate::{
    utils::MousePosition, ENEMY_COLLISION_GROUP, PLAYER_COLLISION_GROUP, WALL_COLLISION_GROUP,
};

pub const SPEED: f32 = 300.0;
pub const PLAYER_KICK_RANGE: f32 = 48.0;

const IDLE_ANIM_OFFSET: usize = 0;
const WALK_ANIM_OFFSET: usize = 4;
const WALK_ANIM_FRAMES: usize = 2;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Default, Deref, DerefMut)]
pub struct InputDirection(Vec2);

#[derive(Default, Deref, DerefMut)]
pub struct PlayerDirection(IVec2);

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    rigidbody: RigidBody,
    velocity: Velocity,
    collider: Collider,
    anim_timer: AnimationTimer,
    #[bundle]
    spritesheet: SpriteSheetBundle,
    collision_group: CollisionGroups,
    locked: LockedAxes
}

pub struct Plugin;

impl Plugin {
    fn init(
        mut cmd: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let texture_handle = asset_server.load("player.png");
        let atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 32.0), 12, 1);
        let texture_atlas = texture_atlases.add(atlas);
        cmd.spawn_bundle(PlayerBundle {
            player: Player,
            rigidbody: RigidBody::KinematicVelocityBased,
            velocity: Velocity::default(),
            collider: Collider::ball(16.0),
            anim_timer: AnimationTimer(Timer::from_seconds(0.25, true)),
            spritesheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::from_array([32.0, 64.0])),
                    anchor: Anchor::Custom(Vec2::from_array([0.0, -0.25])),
                    ..default()
                },
                texture_atlas,
                ..default()
            },
            collision_group: CollisionGroups {
                memberships: PLAYER_COLLISION_GROUP,
                filters: ENEMY_COLLISION_GROUP | WALL_COLLISION_GROUP,
            },
            locked: LockedAxes::ROTATION_LOCKED,
        });
    }

    fn movement(
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

    fn animate(
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

    fn attack(
        rapier_ctx: Res<RapierContext>,
        mouse_pos: Res<MousePosition>,
        q_player: Query<&Transform, With<Player>>,
        mouse_buttons: Res<Input<MouseButton>>,
    ) {
        if mouse_buttons.just_pressed(MouseButton::Left) {
            let pos = q_player.single().translation.truncate();
            let cast_dir = (mouse_pos.truncate() - pos).normalize_or_zero();
            let filter = QueryFilter::new().groups(InteractionGroups {
                memberships: PLAYER_COLLISION_GROUP,
                filter: ENEMY_COLLISION_GROUP,
            });
            if let Some((entity, _)) =
                rapier_ctx.cast_ray(pos, cast_dir, PLAYER_KICK_RANGE, true, filter)
            {
                info!("hit {entity:?}!");
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::init)
            .add_system(Self::movement)
            .add_system(Self::animate)
            .add_system(Self::attack)
            .init_resource::<InputDirection>()
            .init_resource::<PlayerDirection>();
    }
}

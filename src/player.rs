use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::utils::{MousePosition, Spritesheets};
use crate::{consts::*, GameState};

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
    locked: LockedAxes,
}

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, spritesheets: Res<Spritesheets>) {
        let texture_atlas = spritesheets.get("Player").unwrap().clone_weak();
        cmd.spawn_bundle(PlayerBundle {
            player: Player,
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::default(),
            collider: Collider::ball(8.0),
            anim_timer: AnimationTimer(Timer::from_seconds(0.25, true)),
            spritesheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::from_array([16.0, 32.0])),
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

        player_vel.linvel = input_direction.normalize_or_zero() * PLAYER_SPEED;

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
                    IVec2 { x: _, y: 1 } => {
                        sprite.index = PLAYER_WALK_ANIM_OFFSET + PLAYER_WALK_ANIM_FRAMES * 0
                    }
                    IVec2 { x: 1, y: _ } => {
                        sprite.index = PLAYER_WALK_ANIM_OFFSET + PLAYER_WALK_ANIM_FRAMES * 1
                    }
                    IVec2 { x: -1, y: _ } => {
                        sprite.index = PLAYER_WALK_ANIM_OFFSET + PLAYER_WALK_ANIM_FRAMES * 2
                    }
                    IVec2 { x: _, y: -1 } => {
                        sprite.index = PLAYER_WALK_ANIM_OFFSET + PLAYER_WALK_ANIM_FRAMES * 3
                    }
                    _ => (),
                }
            } else {
                match **player_direction {
                    IVec2 { x: _, y: 1 } => sprite.index = PLAYER_IDLE_ANIM_OFFSET + 0,
                    IVec2 { x: 1, y: _ } => sprite.index = PLAYER_IDLE_ANIM_OFFSET + 1,
                    IVec2 { x: -1, y: _ } => sprite.index = PLAYER_IDLE_ANIM_OFFSET + 2,
                    IVec2 { x: _, y: -1 } => sprite.index = PLAYER_IDLE_ANIM_OFFSET + 3,
                    _ => (),
                }
            }
        }
        if **input_direction != Vec2::ZERO {
            timer.tick(time.delta());

            if timer.just_finished() {
                match **player_direction {
                    IVec2 { x: _, y: 1 } => {
                        sprite.index = PLAYER_WALK_ANIM_OFFSET
                            + PLAYER_WALK_ANIM_FRAMES * 0
                            + ((sprite.index - PLAYER_WALK_ANIM_OFFSET + 1)
                                % PLAYER_WALK_ANIM_FRAMES);
                    }
                    IVec2 { x: 1, y: _ } => {
                        sprite.index = PLAYER_WALK_ANIM_OFFSET
                            + PLAYER_WALK_ANIM_FRAMES * 1
                            + ((sprite.index - PLAYER_WALK_ANIM_OFFSET + 1)
                                % PLAYER_WALK_ANIM_FRAMES);
                    }
                    IVec2 { x: -1, y: _ } => {
                        sprite.index = PLAYER_WALK_ANIM_OFFSET
                            + PLAYER_WALK_ANIM_FRAMES * 2
                            + ((sprite.index - PLAYER_WALK_ANIM_OFFSET + 1)
                                % PLAYER_WALK_ANIM_FRAMES);
                    }
                    IVec2 { x: _, y: -1 } => {
                        sprite.index = PLAYER_WALK_ANIM_OFFSET
                            + PLAYER_WALK_ANIM_FRAMES * 3
                            + ((sprite.index - PLAYER_WALK_ANIM_OFFSET + 1)
                                % PLAYER_WALK_ANIM_FRAMES);
                    }
                    _ => (),
                }
            }
        }
    }

    fn attack(
        mut cmd: Commands,
        rapier_ctx: Res<RapierContext>,
        mouse_pos: Res<MousePosition>,
        q_player: Query<&Transform, With<Player>>,
        mouse_buttons: Res<Input<MouseButton>>,
    ) {
        if mouse_buttons.just_pressed(MouseButton::Left) {
            let pos = q_player.single().translation.truncate();
            let cast_dir = (mouse_pos.truncate() - pos).normalize_or_zero();
            let filter = QueryFilter::new().groups(InteractionGroups {
                memberships: PLAYER_ATTACK_COLLISION_GROUP,
                filter: ENEMY_COLLISION_GROUP | WALL_COLLISION_GROUP | ENEMY_ATTACK_COLLISION_GROUP,
            });
            if let Some((entity, _)) =
                rapier_ctx.cast_ray(pos, cast_dir, PLAYER_KICK_RANGE, true, filter)
            {
                info!("hit {entity:?}!");
                //TODO: do something instead of instakill
                cmd.entity(entity).despawn_recursive();
            }
        }
    }

    fn init_throw(
        // mut cmd: Commands,
        mouse_pos: Res<MousePosition>,
        mouse_buttons: Res<Input<MouseButton>>,
        q_player: Query<&Transform, With<Player>>,
    ) {
        if mouse_buttons.just_pressed(MouseButton::Right) {
            let pos = q_player.single().translation.truncate();
            let throw_dir = (mouse_pos.truncate() - pos).normalize_or_zero();
            info!("AAAH I'VE BEEN THROWN IN THE DIRECTION {throw_dir}");
            // cmd.spawn_bundle()
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init)
            .add_system(Self::movement.run_in_state(GameState::InGame))
            .add_system(Self::animate.run_in_state(GameState::InGame))
            .add_system(Self::attack.run_in_state(GameState::InGame))
            .add_system(Self::init_throw.run_in_state(GameState::InGame))
            .init_resource::<InputDirection>()
            .init_resource::<PlayerDirection>();
    }
}

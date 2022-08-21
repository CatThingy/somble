use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::enemy::HitstunTimer;
use crate::potion::{PotionBrewData, PotionBrewState, PotionBrewUi};
use crate::utils::{MousePosition, TimeScale};
use crate::{consts::*, Enemy, GameState};

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Default, Deref, DerefMut)]
pub struct InputDirection(Vec2);

#[derive(Default, Deref, DerefMut)]
pub struct PlayerDirection(IVec2);

struct Kicked {
    target: Entity,
    direction: Vec2,
}

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

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let player_texture = asset_server.load("player.png");
        let player_atlas = TextureAtlas::from_grid(player_texture, Vec2::new(16.0, 32.0), 12, 1);
        let texture_atlas = texture_atlases.add(player_atlas);
        PlayerBundle {
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
                transform: Transform {
                    translation: Vec3::Z,
                    ..default()
                },
                ..default()
            },
            collision_group: CollisionGroups {
                memberships: PLAYER_COLLISION_GROUP,
                filters: ENEMY_COLLISION_GROUP | WALL_COLLISION_GROUP,
            },
            locked: LockedAxes::ROTATION_LOCKED,
        }
    }
}

pub struct Plugin;

impl Plugin {
    fn movement(
        mut q_player: Query<&mut Velocity, With<Player>>,
        keys: Res<Input<KeyCode>>,
        mut input_direction: ResMut<InputDirection>,
        mut player_direction: ResMut<PlayerDirection>,
    ) {
        let mut player_vel = match q_player.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };

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
        time_scale: Res<TimeScale>,
        player_direction: Res<PlayerDirection>,
        input_direction: Res<InputDirection>,
    ) {
        let (mut timer, mut sprite) = match q_player.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };

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
            timer.tick(time.delta().mul_f32(**time_scale));

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

    fn kick(
        rapier_ctx: Res<RapierContext>,
        mouse_pos: Res<MousePosition>,
        q_player: Query<&Transform, With<Player>>,
        mouse_buttons: Res<Input<MouseButton>>,
        mut event_writer: EventWriter<Kicked>,
    ) {
        let player = match q_player.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };

        if mouse_buttons.just_pressed(MouseButton::Left) {
            let pos = player.translation.truncate();
            let cast_dir = (mouse_pos.truncate() - pos).normalize_or_zero();
            let filter = QueryFilter::new().groups(InteractionGroups {
                memberships: PLAYER_ATTACK_COLLISION_GROUP,
                filter: ENEMY_COLLISION_GROUP | WALL_COLLISION_GROUP | ENEMY_ATTACK_COLLISION_GROUP,
            });
            if let Some((entity, _)) =
                rapier_ctx.cast_ray(pos, cast_dir, PLAYER_KICK_RANGE, true, filter)
            {
                event_writer.send(Kicked {
                    target: entity,
                    direction: cast_dir,
                });
            }
        }
    }

    fn init_throw(
        mouse_pos: Res<MousePosition>,
        mouse_buttons: Res<Input<MouseButton>>,
        q_player: Query<&Transform, (With<Player>, Without<PotionBrewUi>)>,
        mut q_brew_ui: Query<&mut Transform, (Without<Player>, With<PotionBrewUi>)>,

        mut brew_data: ResMut<PotionBrewData>,
        mut brew_state: ResMut<PotionBrewState>,
    ) {
        let player = match q_player.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };
        if mouse_buttons.just_pressed(MouseButton::Right) {
            let pos = player.translation.truncate();
            let throw_dir = (mouse_pos.truncate() - pos).normalize_or_zero();
            let mut brew_ui_transform = q_brew_ui.single_mut();

            brew_ui_transform.translation.x = mouse_pos.x;
            brew_ui_transform.translation.y = mouse_pos.y;

            brew_data.direction = throw_dir;
            *brew_state = PotionBrewState::Active;
        }
    }

    fn handle_kick(
        mut cmd: Commands,
        mut event_reader: EventReader<Kicked>,
        mut q_enemy: Query<&mut HitstunTimer, With<Enemy>>,
    ) {
        for event in event_reader.iter() {
            if let Ok(mut hitstun_timer) = q_enemy.get_mut(event.target) {
                cmd.entity(event.target).insert(ExternalImpulse {
                    impulse: event.direction * PLAYER_KICK_FORCE,
                    torque_impulse: 0.0,
                });
                hitstun_timer.set_duration(Duration::from_secs_f32(PLAYER_KICK_HITSTUN_SECS));
                hitstun_timer.reset();
            }
        }
    }
}
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::movement.run_in_state(GameState::InGame))
            .add_system(Self::animate.run_in_state(GameState::InGame))
            .add_system(Self::kick.run_in_state(GameState::InGame))
            .add_system(Self::init_throw.run_in_state(GameState::InGame))
            .add_system(Self::handle_kick.run_in_state(GameState::InGame))
            .init_resource::<InputDirection>()
            .init_resource::<PlayerDirection>()
            .add_event::<Kicked>()
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

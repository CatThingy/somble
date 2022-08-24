use bevy::{prelude::*, sprite::Anchor};

use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::grid_coords_to_translation_centered;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use ordered_float::OrderedFloat;
use pathfinding::directed::astar::astar;

use crate::health::Health;
use crate::level::WalkableTiles;
use crate::status::Blinded;
use crate::status::Slowed;
use crate::utils::TimeScale;
use crate::utils::UniformAnim;
use crate::{consts::*, player::Player, Enemy, GameState};

#[derive(Component, Deref, DerefMut, Debug)]
pub struct HitstunTimer(pub Timer);

#[derive(Component, Deref, DerefMut, Debug)]
pub struct AttackTimer(Timer);

#[derive(Component, PartialEq)]
pub enum EnemyState {
    Idle,
    Chase,
    Attack,
}

#[derive(Bundle)]
pub struct ElementalBundle {
    enemy: Enemy,
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    groups: CollisionGroups,
    locked: LockedAxes,
    damping: Damping,
    hitstun: HitstunTimer,
    anim: UniformAnim,
    health: Health,
    state: EnemyState,
    #[bundle]
    spritesheet: SpriteSheetBundle,
}

impl LdtkEntity for ElementalBundle {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let elemental_texture = asset_server.load("elemental.png");
        let elemental_atlas =
            TextureAtlas::from_grid(elemental_texture, Vec2::new(16.0, 32.0), 3, 1);
        let texture_atlas = texture_atlases.add(elemental_atlas);
        ElementalBundle {
            enemy: Enemy,
            body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            collider: Collider::ball(5.0),
            groups: CollisionGroups {
                memberships: ENEMY_COLLISION_GROUP,
                filters: PLAYER_COLLISION_GROUP
                    | ENEMY_COLLISION_GROUP
                    | WALL_COLLISION_GROUP
                    | PLAYER_ATTACK_COLLISION_GROUP,
            },
            locked: LockedAxes::ROTATION_LOCKED,
            damping: Damping {
                linear_damping: 20.0,
                angular_damping: 0.0,
            },
            anim: UniformAnim(Timer::from_seconds(0.1, true)),
            hitstun: HitstunTimer(Timer::from_seconds(0.0, false)),
            health: Health::new(ELEMENTAL_HEALTH),
            state: EnemyState::Idle,
            spritesheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    anchor: Anchor::Custom(Vec2::from_array([0.0, -0.25])),
                    index: 0,
                    color: Color::LIME_GREEN,
                    ..default()
                },
                texture_atlas,
                ..default()
            },
        }
    }
}

pub struct Plugin;

impl Plugin {
    fn update_state(
        mut q_enemy: Query<
            (&Transform, &mut EnemyState, &TextureAtlasSprite),
            (With<Enemy>, Without<Player>),
        >,
        rapier_ctx: Res<RapierContext>,
        q_player: Query<(Entity, &Transform), (Without<Enemy>, With<Player>)>,
    ) {
        let (player, player_transform) = match q_player.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };
        let player_pos = player_transform.translation.truncate();

        for (enemy_transform, mut enemy_state, sprite) in &mut q_enemy {
            let enemy_pos = enemy_transform.translation.truncate();
            let direction = player_pos - enemy_pos;
            let distance = direction.length();
            match *enemy_state {
                EnemyState::Idle => {
                    if distance < ELEMENTAL_AGGRO_RANGE {
                        *enemy_state = EnemyState::Chase;
                    }
                }
                EnemyState::Chase => {
                    let sight_filter = QueryFilter {
                        groups: Some(InteractionGroups {
                            memberships: ENEMY_COLLISION_GROUP,
                            filter: PLAYER_COLLISION_GROUP | WALL_COLLISION_GROUP,
                        }),
                        ..default()
                    };

                    if distance > ELEMENTAL_FORGET_RANGE {
                        *enemy_state = EnemyState::Idle;
                    } else if matches!(
                        rapier_ctx.cast_ray(enemy_pos, direction, f32::MAX, true, sight_filter),
                        Some((hit, _)) if hit == player,
                    ) && distance < ELEMENTAL_ATTACK_RANGE
                    {
                        *enemy_state = EnemyState::Attack;
                    }
                }
                EnemyState::Attack => {
                    if sprite.index == ELEMENTAL_ATTACK_ANIM_OFFSET + ELEMENTAL_ATTACK_ANIM_FRAMES {
                        *enemy_state = EnemyState::Chase;
                    }
                }
            }
        }
    }
    fn movement(
        mut q_enemy: Query<
            (
                &Transform,
                &mut Velocity,
                &HitstunTimer,
                &mut TextureAtlasSprite,
                &Collider,
                &EnemyState,
                Option<&Slowed>,
                Option<&Blinded>,
            ),
            (With<Enemy>, Without<Player>),
        >,
        q_player: Query<(&Transform, Entity), (Without<Enemy>, With<Player>)>,
        rapier_ctx: Res<RapierContext>,
        walkables: Res<WalkableTiles>,
    ) {
        let (player_transform, player) = match q_player.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };

        let player_pos = player_transform.translation.truncate();
        let player_tile_pos: IVec2 =
            translation_to_grid_coords(player_pos, IVec2::splat(GRID_SIZE)).into();

        let sight_filter = QueryFilter {
            groups: Some(InteractionGroups {
                memberships: ENEMY_COLLISION_GROUP,
                filter: PLAYER_COLLISION_GROUP | WALL_COLLISION_GROUP,
            }),
            ..default()
        };

        for (transform, mut vel, hitstun, mut sprite, collider, state, slowed, blinded) in
            &mut q_enemy
        {
            if !hitstun.finished() || state != &EnemyState::Chase {
                continue;
            }
            let pos = transform.translation.truncate();
            let direction = player_pos - pos;

            let speed = ELEMENTAL_SPEED
                * match slowed {
                    Some(_) => 0.7,
                    None => 1.0,
                };
            if let Some((hit, _)) =
                rapier_ctx.cast_shape(pos, 0.0, direction, collider, f32::MAX, sight_filter)
            {
                if hit == player {
                    vel.linvel = direction.normalize() * speed;
                } else if blinded.is_none() {
                    let enemy_tile_pos: IVec2 =
                        translation_to_grid_coords(pos, IVec2::splat(GRID_SIZE)).into();
                    if let Some((path, _)) = astar(
                        &enemy_tile_pos,
                        |&node| {
                            const SQRT_2: OrderedFloat<f32> =
                                OrderedFloat(std::f32::consts::SQRT_2);
                            const ONE: OrderedFloat<f32> = OrderedFloat(1.0);

                            const CARDINAL_NEIGHBOURS: [(IVec2, OrderedFloat<f32>); 4] = [
                                (IVec2::from_array([-1, 0]), ONE),
                                (IVec2::from_array([0, 1]), ONE),
                                (IVec2::from_array([1, 0]), ONE),
                                (IVec2::from_array([0, -1]), ONE),
                            ];
                            const DIAGONAL_NEIGHBOURS: [(IVec2, OrderedFloat<f32>); 4] = [
                                (IVec2::from_array([-1, -1]), SQRT_2),
                                (IVec2::from_array([-1, 1]), SQRT_2),
                                (IVec2::from_array([1, 1]), SQRT_2),
                                (IVec2::from_array([1, -1]), SQRT_2),
                            ];

                            let mut output: Vec<(IVec2, OrderedFloat<f32>)> = vec![];

                            for (neighbour, dist) in CARDINAL_NEIGHBOURS {
                                let next = node + neighbour;
                                if walkables.contains(&next) {
                                    output.push((next, dist));
                                }
                            }

                            for (neighbour, dist) in DIAGONAL_NEIGHBOURS {
                                let next = node + neighbour;
                                let x_adj = node + IVec2::new(neighbour.x, 0);
                                let y_adj = node + IVec2::new(0, neighbour.y);
                                if walkables.contains(&next)
                                    && walkables.contains(&x_adj)
                                    && walkables.contains(&y_adj)
                                {
                                    output.push((next, dist));
                                }
                            }

                            output
                        },
                        |&node| OrderedFloat((player_tile_pos - node).as_vec2().length()),
                        |&node| player_tile_pos == node,
                    ) {
                        if path.len() <= 1 {
                            vel.linvel = direction.normalize() * speed;
                        } else {
                            let mut path = path.iter().map(|v| {
                                grid_coords_to_translation_centered(
                                    v.to_owned().into(),
                                    IVec2::splat(GRID_SIZE),
                                )
                            });
                            let first = path.next().unwrap();
                            let second = path.next().unwrap();
                            let target = if pos.dot(first).signum() * pos.dot(second).signum() < 0.0
                            {
                                first
                            } else {
                                second
                            };

                            let direction = target - pos;

                            vel.linvel = direction.normalize_or_zero() * speed;
                        }
                    }
                }
            }
            sprite.flip_x = vel.linvel.x < 0.0;
        }
    }

    fn tick_hitstun(
        mut q_enemy: Query<(&mut HitstunTimer, &mut UniformAnim), With<Enemy>>,
        time: Res<Time>,
        time_scale: Res<TimeScale>,
    ) {
        for (mut timer, mut anim) in &mut q_enemy {
            timer.tick(time.delta().mul_f32(**time_scale));

            if timer.finished() && anim.paused() {
                anim.unpause();
            } else if !timer.finished() && !anim.paused() {
                anim.pause();
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::tick_hitstun.run_in_state(GameState::InGame))
            .add_system(Self::update_state.run_in_state(GameState::InGame))
            .add_system(Self::movement.run_in_state(GameState::InGame))
            .register_ldtk_entity::<ElementalBundle>("Elemental");
    }
}

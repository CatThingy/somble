use bevy::{prelude::*, sprite::Anchor};

use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::grid_coords_to_translation_centered;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use ordered_float::OrderedFloat;
use pathfinding::directed::astar::astar;

use crate::level::WalkableTiles;
use crate::utils::UniformAnim;
use crate::{consts::*, player::Player, Enemy, GameState};

#[derive(Component, Deref, DerefMut)]
pub struct HitstunTimer(pub Timer);

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
            collider: Collider::ball(8.0),
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
    fn movement(
        mut q_enemy: Query<
            (&Transform, &mut Velocity, &HitstunTimer),
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

        for (enemy_transform, mut enemy_vel, hitstun) in &mut q_enemy {
            if !hitstun.finished() {
                continue;
            }
            let enemy_pos = enemy_transform.translation.truncate();
            let direction = player_pos - enemy_pos;
            if let Some((hit, _)) =
                rapier_ctx.cast_ray(enemy_pos, direction, f32::MAX, true, sight_filter)
            {
                if hit == player {
                    enemy_vel.linvel = direction.normalize() * ELEMENTAL_SPEED;
                } else {
                    let enemy_tile_pos: IVec2 =
                        translation_to_grid_coords(enemy_pos, IVec2::splat(GRID_SIZE)).into();
                    if let Some((path, _)) = astar(
                        &enemy_tile_pos,
                        |&node| {
                            const SQRT_2: OrderedFloat<f32> =
                                OrderedFloat(std::f32::consts::SQRT_2);
                            const ONE: OrderedFloat<f32> = OrderedFloat(1.0);

                            const NEIGHBOURS: [(IVec2, OrderedFloat<f32>); 8] = [
                                (IVec2::from_array([-1, -1]), SQRT_2),
                                (IVec2::from_array([-1, 0]), ONE),
                                (IVec2::from_array([-1, 1]), SQRT_2),
                                (IVec2::from_array([0, 1]), ONE),
                                (IVec2::from_array([1, 1]), SQRT_2),
                                (IVec2::from_array([1, 0]), ONE),
                                (IVec2::from_array([1, -1]), SQRT_2),
                                (IVec2::from_array([0, -1]), ONE),
                            ];

                            let mut output: Vec<(IVec2, OrderedFloat<f32>)> = vec![];

                            for (neighbour, dist) in NEIGHBOURS {
                                let next = node + neighbour;
                                if walkables.contains(&next) {
                                    output.push((next, dist));
                                }
                            }

                            output
                        },
                        |&node| OrderedFloat((player_tile_pos - node).as_vec2().length()),
                        |&node| player_tile_pos == node,
                    ) {
                        let target = grid_coords_to_translation_centered(
                            path[1].to_owned().into(),
                            IVec2::splat(GRID_SIZE),
                        );

                        let direction = target - enemy_pos;

                        enemy_vel.linvel = direction.normalize() * ELEMENTAL_SPEED;
                    };
                }
            }
        }
    }

    fn tick_hitstun(
        mut q_enemy: Query<(&mut HitstunTimer, &mut UniformAnim), With<Enemy>>,
        time: Res<Time>,
    ) {
        for (mut timer, mut anim) in &mut q_enemy {
            timer.tick(time.delta());

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
            .add_system(Self::movement.run_in_state(GameState::InGame))
            .register_ldtk_entity::<ElementalBundle>("Elemental");
    }
}

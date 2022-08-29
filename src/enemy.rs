use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor};

use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::grid_coords_to_translation_centered;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use ordered_float::OrderedFloat;
use pathfinding::directed::astar::astar;

use crate::health::Dead;
use crate::health::Health;
use crate::hitbox::DamageOnce;
use crate::hitbox::DamagePeriodic;
use crate::hitbox::Falloff;
use crate::hitbox::Hitbox;
use crate::hitstun::HitstunTimer;
use crate::level::NotFromLevel;
use crate::level::WalkableTiles;
use crate::status::Blinded;
use crate::status::Slowed;
use crate::utils::DespawnTimer;
use crate::utils::DestroyOnHit;
use crate::utils::Spiral;
use crate::utils::TimeScale;
use crate::utils::UniformAnim;
use crate::Element;
use crate::{consts::*, player::Player, Enemy, GameState};

#[derive(Component)]
pub struct Attacked(bool);

#[derive(Component, Deref, DerefMut, Debug)]
pub struct AttackTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Default)]
pub struct EnemyStats {
    speed: f32,
    aggro_range: f32,
    forget_range: f32,
    attack_range: f32,
}

#[derive(Component, PartialEq, Debug)]
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
    anim: AnimationTimer,
    health: Health,
    state: EnemyState,
    attack_timer: AttackTimer,
    element: Element,
    stats: EnemyStats,
    attacked: Attacked,
    #[bundle]
    spritesheet: SpriteSheetBundle,
}

impl LdtkEntity for ElementalBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut element = None;
        for field in &entity_instance.field_instances {
            if field.identifier.as_str() == "Element" {
                element = match &field.value {
                    FieldValue::Enum(value) => match value.as_ref().unwrap().as_str() {
                        "Fire" => Some(Element::Fire),
                        "Water" => Some(Element::Water),
                        "Wind" => Some(Element::Wind),
                        "Lightning" => Some(Element::Lightning),
                        "Earth" => Some(Element::Earth),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
        }

        let element = element.unwrap();

        let mut bundle = ElementalBundle {
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
            anim: AnimationTimer(Timer::from_seconds(0.0, true)),
            hitstun: HitstunTimer(Timer::from_seconds(0.0, false)),
            health: Health::new(0.0),
            state: EnemyState::Idle,
            attack_timer: AttackTimer(Timer::from_seconds(0.0, false)),
            element,
            spritesheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    anchor: Anchor::Custom(Vec2::from_array([0.0, -0.25])),
                    ..default()
                },
                ..default()
            },
            stats: EnemyStats::default(),
            attacked: Attacked(false),
        };

        match element {
            Element::Fire => {
                bundle
                    .anim
                    .set_duration(Duration::from_secs_f32(FIRE_ELEMENTAL_ANIM_PERIOD));
                bundle
                    .attack_timer
                    .set_duration(Duration::from_secs_f32(FIRE_ELEMENTAL_ATTACK_PERIOD));
                bundle.health = Health::new(FIRE_ELEMENTAL_HEALTH);
                bundle.stats = EnemyStats {
                    speed: FIRE_ELEMENTAL_SPEED,
                    aggro_range: FIRE_ELEMENTAL_AGGRO_RANGE,
                    forget_range: FIRE_ELEMENTAL_FORGET_RANGE,
                    attack_range: FIRE_ELEMENTAL_ATTACK_RANGE,
                };
                bundle.spritesheet.texture_atlas = {
                    let elemental_texture = asset_server.load("fire_elemental.png");
                    let elemental_atlas =
                        TextureAtlas::from_grid(elemental_texture, Vec2::new(16.0, 32.0), 14, 1);
                    texture_atlases.add(elemental_atlas)
                };

                bundle
            }
            Element::Water => {
                bundle
                    .anim
                    .set_duration(Duration::from_secs_f32(WATER_ELEMENTAL_ANIM_PERIOD));
                bundle
                    .attack_timer
                    .set_duration(Duration::from_secs_f32(WATER_ELEMENTAL_ATTACK_PERIOD));
                bundle.health = Health::new(WATER_ELEMENTAL_HEALTH);
                bundle.stats = EnemyStats {
                    speed: WATER_ELEMENTAL_SPEED,
                    aggro_range: WATER_ELEMENTAL_AGGRO_RANGE,
                    forget_range: WATER_ELEMENTAL_FORGET_RANGE,
                    attack_range: WATER_ELEMENTAL_ATTACK_RANGE,
                };
                bundle.spritesheet.texture_atlas = {
                    let elemental_texture = asset_server.load("water_elemental.png");
                    let elemental_atlas =
                        TextureAtlas::from_grid(elemental_texture, Vec2::new(16.0, 32.0), 14, 1);
                    texture_atlases.add(elemental_atlas)
                };

                bundle
            }
            Element::Wind => {
                bundle
                    .anim
                    .set_duration(Duration::from_secs_f32(WIND_ELEMENTAL_ANIM_PERIOD));
                bundle
                    .attack_timer
                    .set_duration(Duration::from_secs_f32(WIND_ELEMENTAL_ATTACK_PERIOD));
                bundle.health = Health::new(WIND_ELEMENTAL_HEALTH);
                bundle.stats = EnemyStats {
                    speed: WIND_ELEMENTAL_SPEED,
                    aggro_range: WIND_ELEMENTAL_AGGRO_RANGE,
                    forget_range: WIND_ELEMENTAL_FORGET_RANGE,
                    attack_range: WIND_ELEMENTAL_ATTACK_RANGE,
                };
                bundle.spritesheet.texture_atlas = {
                    let elemental_texture = asset_server.load("wind_elemental.png");
                    let elemental_atlas =
                        TextureAtlas::from_grid(elemental_texture, Vec2::new(16.0, 32.0), 14, 1);
                    texture_atlases.add(elemental_atlas)
                };

                bundle
            }
            Element::Lightning => {
                bundle
                    .anim
                    .set_duration(Duration::from_secs_f32(LIGHTNING_ELEMENTAL_ANIM_PERIOD));
                bundle
                    .attack_timer
                    .set_duration(Duration::from_secs_f32(LIGHTNING_ELEMENTAL_ATTACK_PERIOD));
                bundle.health = Health::new(LIGHTNING_ELEMENTAL_HEALTH);
                bundle.stats = EnemyStats {
                    speed: LIGHTNING_ELEMENTAL_SPEED,
                    aggro_range: LIGHTNING_ELEMENTAL_AGGRO_RANGE,
                    forget_range: LIGHTNING_ELEMENTAL_FORGET_RANGE,
                    attack_range: LIGHTNING_ELEMENTAL_ATTACK_RANGE,
                };
                bundle.spritesheet.texture_atlas = {
                    let elemental_texture = asset_server.load("lightning_elemental.png");
                    let elemental_atlas =
                        TextureAtlas::from_grid(elemental_texture, Vec2::new(16.0, 32.0), 14, 1);
                    texture_atlases.add(elemental_atlas)
                };

                bundle
            }
            Element::Earth => {
                bundle
                    .anim
                    .set_duration(Duration::from_secs_f32(EARTH_ELEMENTAL_ANIM_PERIOD));
                bundle
                    .attack_timer
                    .set_duration(Duration::from_secs_f32(EARTH_ELEMENTAL_ATTACK_PERIOD));
                bundle.health = Health::new(EARTH_ELEMENTAL_HEALTH);
                bundle.stats = EnemyStats {
                    speed: EARTH_ELEMENTAL_SPEED,
                    aggro_range: EARTH_ELEMENTAL_AGGRO_RANGE,
                    forget_range: EARTH_ELEMENTAL_FORGET_RANGE,
                    attack_range: EARTH_ELEMENTAL_ATTACK_RANGE,
                };
                bundle.spritesheet.texture_atlas = {
                    let elemental_texture = asset_server.load("earth_elemental.png");
                    let elemental_atlas =
                        TextureAtlas::from_grid(elemental_texture, Vec2::new(16.0, 32.0), 14, 1);
                    texture_atlases.add(elemental_atlas)
                };

                bundle
            }
        }
    }
}

pub struct Plugin;

impl Plugin {
    fn update_state(
        mut q_enemy: Query<
            (
                &Transform,
                &mut EnemyState,
                &TextureAtlasSprite,
                &mut AttackTimer,
                &EnemyStats,
                &mut Attacked,
                Option<&Blinded>,
            ),
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

        for (
            enemy_transform,
            mut enemy_state,
            sprite,
            mut attack_timer,
            stats,
            mut attacked,
            blinded,
        ) in &mut q_enemy
        {
            let enemy_pos = enemy_transform.translation.truncate();
            let direction = player_pos - enemy_pos;
            let distance = direction.length();

            let blinded_multiplier = if blinded.is_some() { 0.33 } else { 1.0 };
            match *enemy_state {
                EnemyState::Idle => {
                    if distance < stats.aggro_range {
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

                    if distance > stats.forget_range * blinded_multiplier {
                        *enemy_state = EnemyState::Idle;
                    } else if attack_timer.finished()
                        && matches!(
                            rapier_ctx.cast_ray(enemy_pos, direction, f32::MAX, true, sight_filter),
                            Some((hit, _)) if hit == player,
                        )
                        && distance < stats.attack_range * blinded_multiplier
                    {
                        *enemy_state = EnemyState::Attack;
                        attack_timer.reset();
                    }
                }
                EnemyState::Attack => {
                    if sprite.index
                        == ELEMENTAL_ATTACK_ANIM_OFFSET + ELEMENTAL_ATTACK_ANIM_FRAMES - 1
                    {
                        *enemy_state = EnemyState::Chase;
                        attacked.0 = false;
                    }
                }
            }
        }
    }

    fn tick_attack(
        mut q_enemy: Query<&mut AttackTimer>,
        time: Res<Time>,
        time_scale: Res<TimeScale>,
    ) {
        let delta = time.delta().mul_f32(**time_scale);
        for mut timer in &mut q_enemy {
            timer.tick(delta);
        }
    }

    fn attack(
        mut cmd: Commands,
        mut q_enemy: Query<
            (&Transform, &TextureAtlasSprite, &Element, &mut Attacked),
            (With<Enemy>, Without<Player>, Changed<TextureAtlasSprite>),
        >,
        q_player: Query<&Transform, (Without<Enemy>, With<Player>)>,
        assets: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let player_transform = match q_player.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };
        let player_pos = player_transform.translation.truncate();

        for (enemy_transform, sprite, element, mut attacked) in &mut q_enemy {
            if sprite.index == ELEMENTAL_ATTACK_EMIT_FRAME && !attacked.0 {
                attacked.0 = true;
                let enemy_pos = enemy_transform.translation.truncate();
                let direction = (player_pos - enemy_pos).normalize();
                // do attack
                match element {
                    Element::Fire => {
                        cmd.spawn_bundle(SpriteBundle {
                            texture: assets.load("fire_elemental_attack.png"),
                            transform: Transform {
                                translation: enemy_transform.translation,
                                ..default()
                            },
                            ..default()
                        })
                        .insert_bundle((
                            RigidBody::Dynamic,
                            Velocity {
                                linvel: direction * FIRE_ELEMENTAL_ATTACK_VELOCITY,
                                angvel: 0.0,
                            },
                            Collider::ball(FIRE_ELEMENTAL_ATTACK_RADIUS),
                            CollisionGroups {
                                memberships: ENEMY_ATTACK_COLLISION_GROUP,
                                filters: PLAYER_COLLISION_GROUP | WALL_COLLISION_GROUP,
                            },
                            ActiveEvents::COLLISION_EVENTS,
                            Sensor,
                            Hitbox,
                            DamageOnce::new(FIRE_ELEMENTAL_ATTACK_DAMAGE, Falloff::none()),
                            DestroyOnHit,
                            NotFromLevel,
                        ));
                    }
                    Element::Water => {
                        let rotation = enemy_transform.rotation.to_euler(EulerRot::XYZ).2;
                        for i in 0..3 {
                            let rotation = rotation + (std::f32::consts::TAU / 3 as f32) * i as f32;

                            let direction = Mat2::from_angle(rotation) * Vec2::X;
                            cmd.spawn_bundle(SpriteBundle {
                                texture: assets.load("water_elemental_attack.png"),
                                transform: Transform {
                                    translation: enemy_transform.translation,
                                    ..default()
                                },
                                ..default()
                            })
                            .insert_bundle((
                                RigidBody::Dynamic,
                                Velocity {
                                    linvel: direction * WATER_ELEMENTAL_ATTACK_VELOCITY,
                                    angvel: 0.0,
                                },
                                Collider::ball(2.0),
                                CollisionGroups {
                                    memberships: ENEMY_ATTACK_COLLISION_GROUP,
                                    filters: PLAYER_COLLISION_GROUP,
                                },
                                ActiveEvents::COLLISION_EVENTS,
                                Sensor,
                                Hitbox,
                                DamageOnce::new(WATER_ELEMENTAL_ATTACK_DAMAGE, Falloff::none()),
                                DespawnTimer(Timer::from_seconds(5.0, false)),
                                Spiral { rate: 2.0 },
                                NotFromLevel,
                            ));
                        }
                    }
                    Element::Wind => {
                        cmd.spawn_bundle(SpriteBundle {
                            texture: assets.load("wind_elemental_attack.png"),
                            transform: Transform {
                                translation: enemy_transform.translation,
                                ..default()
                            },
                            ..default()
                        })
                        .insert_bundle((
                            RigidBody::Dynamic,
                            Velocity {
                                linvel: direction * WIND_ELEMENTAL_ATTACK_VELOCITY,
                                angvel: 0.0,
                            },
                            Collider::ball(8.0),
                            CollisionGroups {
                                memberships: ENEMY_ATTACK_COLLISION_GROUP,
                                filters: PLAYER_COLLISION_GROUP,
                            },
                            ActiveEvents::COLLISION_EVENTS,
                            Sensor,
                            Hitbox,
                            DamageOnce::new(WIND_ELEMENTAL_ATTACK_DAMAGE, Falloff::none()),
                            DespawnTimer(Timer::from_seconds(0.05, false)),
                            NotFromLevel,
                        ));
                    }
                    Element::Lightning => {
                        cmd.spawn_bundle(SpriteSheetBundle {
                            texture_atlas: {
                                let tex = assets.load("lightning_elemental_attack.png");
                                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 6, 1))
                            },
                            transform: Transform {
                                translation: enemy_transform.translation,
                                ..default()
                            },
                            ..default()
                        })
                        .insert_bundle((
                            RigidBody::Dynamic,
                            Velocity {
                                linvel: direction * LIGHTNING_ELEMENTAL_ATTACK_VELOCITY,
                                angvel: 0.0,
                            },
                            Collider::ball(4.0),
                            CollisionGroups {
                                memberships: ENEMY_ATTACK_COLLISION_GROUP,
                                filters: WALL_COLLISION_GROUP,
                            },
                            ActiveEvents::COLLISION_EVENTS,
                            DespawnTimer(Timer::from_seconds(5.0, false)),
                            UniformAnim(Timer::from_seconds(0.1, true)),
                            NotFromLevel,
                            DestroyOnHit,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(SpatialBundle::default())
                                .insert_bundle((
                                    Collider::ball(16.0),
                                    CollisionGroups {
                                        memberships: ENEMY_ATTACK_COLLISION_GROUP,
                                        filters: PLAYER_COLLISION_GROUP,
                                    },
                                    ActiveEvents::COLLISION_EVENTS,
                                    Sensor,
                                    Hitbox,
                                    DamagePeriodic::new(
                                        LIGHTNING_ELEMENTAL_ATTACK_DAMAGE,
                                        Falloff::none(),
                                        0.25,
                                    ),
                                ));
                        });
                    }
                    Element::Earth => {
                        cmd.spawn_bundle(SpriteBundle {
                            texture: assets.load("earth_elemental_attack.png"),
                            transform: Transform {
                                translation: enemy_transform.translation,
                                ..default()
                            },
                            ..default()
                        })
                        .insert_bundle((
                            RigidBody::Dynamic,
                            Velocity {
                                linvel: direction * EARTH_ELEMENTAL_ATTACK_VELOCITY,
                                angvel: 0.0,
                            },
                            Collider::ball(16.0),
                            CollisionGroups {
                                memberships: ENEMY_ATTACK_COLLISION_GROUP,
                                filters: PLAYER_COLLISION_GROUP,
                            },
                            ActiveEvents::COLLISION_EVENTS,
                            Sensor,
                            Hitbox,
                            DamageOnce::new(EARTH_ELEMENTAL_ATTACK_DAMAGE, Falloff::none()),
                            DespawnTimer(Timer::from_seconds(0.05, false)),
                            NotFromLevel,
                        ));
                    }
                }
            }
        }
    }

    fn anim(
        mut q_enemy: Query<
            (
                &mut TextureAtlasSprite,
                &mut EnemyState,
                &mut AnimationTimer,
            ),
            With<Enemy>,
        >,

        time: Res<Time>,
        time_scale: Res<TimeScale>,
    ) {
        let delta = time.delta().mul_f32(**time_scale);
        for (mut sprite, state, mut timer) in &mut q_enemy {
            if state.is_changed() {
                sprite.index = match *state {
                    EnemyState::Idle => ELEMENTAL_IDLE_ANIM_OFFSET,
                    EnemyState::Chase => ELEMENTAL_WALK_ANIM_OFFSET,
                    EnemyState::Attack => ELEMENTAL_ATTACK_ANIM_OFFSET,
                }
            }

            timer.tick(delta);

            if timer.finished() {
                sprite.index = match *state {
                    EnemyState::Idle => {
                        ELEMENTAL_IDLE_ANIM_OFFSET
                            + ELEMENTAL_IDLE_ANIM_FRAMES * 0
                            + ((sprite.index - ELEMENTAL_IDLE_ANIM_OFFSET + 1)
                                % ELEMENTAL_IDLE_ANIM_FRAMES)
                    }
                    EnemyState::Chase => {
                        ELEMENTAL_WALK_ANIM_OFFSET
                            + ELEMENTAL_WALK_ANIM_FRAMES * 0
                            + ((sprite.index - ELEMENTAL_WALK_ANIM_OFFSET + 1)
                                % ELEMENTAL_WALK_ANIM_FRAMES)
                    }
                    EnemyState::Attack => {
                        ELEMENTAL_ATTACK_ANIM_OFFSET
                            + ELEMENTAL_ATTACK_ANIM_FRAMES * 0
                            + ((sprite.index - ELEMENTAL_ATTACK_ANIM_OFFSET + 1)
                                % ELEMENTAL_ATTACK_ANIM_FRAMES)
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
                &EnemyStats,
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

        for (transform, mut vel, hitstun, mut sprite, collider, state, stats, slowed, blinded) in
            &mut q_enemy
        {
            if !hitstun.finished() || state != &EnemyState::Chase {
                continue;
            }
            let pos = transform.translation.truncate();
            let direction = player_pos - pos;

            let speed = stats.speed
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

    fn hitstun(
        mut q_enemy: Query<(&HitstunTimer, &mut AnimationTimer, &mut EnemyState), With<Enemy>>,
    ) {
        for (timer, mut anim, mut state) in &mut q_enemy {
            if timer.finished() && anim.paused() {
                anim.unpause();
            } else if !timer.finished() && !anim.paused() {
                anim.pause();
                *state = EnemyState::Idle;
            }
        }
    }

    fn start_die(
        mut cmd: Commands,
        mut q_enemy: Query<(Entity, &mut TextureAtlasSprite), (With<Enemy>, Added<Dead>)>,
    ) {
        for (entity, mut sprite) in &mut q_enemy {
            sprite.index = ELEMENTAL_DEATH_ANIM_OFFSET;
            cmd.entity(entity)
                .remove::<Collider>()
                .remove::<RigidBody>()
                .remove::<EnemyState>()
                .remove::<AnimationTimer>()
                .insert(UniformAnim(Timer::from_seconds(0.1, true)));
        }
    }
    fn die(
        mut cmd: Commands,
        q_enemy: Query<(Entity, &TextureAtlasSprite), (With<Enemy>, With<Dead>)>,
    ) {
        for (entity, anim) in &q_enemy {
            if anim.index == ELEMENTAL_DEATH_ANIM_OFFSET + ELEMENTAL_DEATH_ANIM_FRAMES - 1 {
                cmd.entity(entity).despawn_recursive()
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::hitstun.run_in_state(GameState::InGame))
            .add_system(Self::update_state.run_in_state(GameState::InGame))
            .add_system(Self::movement.run_in_state(GameState::InGame))
            .add_system(Self::tick_attack.run_in_state(GameState::InGame))
            .add_system(Self::attack.run_in_state(GameState::InGame))
            .add_system(Self::anim.run_in_state(GameState::InGame).label("anim"))
            .add_system(
                Self::start_die
                    .run_in_state(GameState::InGame)
                    .after("anim"),
            )
            .add_system(Self::die.run_in_state(GameState::InGame))
            .register_ldtk_entity::<ElementalBundle>("Elemental");
    }
}

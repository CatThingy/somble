use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    consts::*,
    essence::EssenceCounts,
    hitbox::{
        DamageOnce, DamagePeriodic, DirectedForce, Falloff, Hitbox, Hitstun, RadialForce,
        RadialImpulse, StatusEffect,
    },
    homing::Homing,
    level::NotFromLevel,
    player::Player,
    status::Effect,
    utils::{
        DespawnTimer, Digits, ElementIconAtlases, MousePosition, TimeIndependent, TimeScale,
        UniformAnim, UniformAnimOnce,
    },
    Element, GameState, PauseState,
};

#[derive(Component)]
pub struct PotionBrewUi;

#[derive(Component)]
pub struct PotionUiSelect1;

#[derive(Component)]
pub struct PotionUiSelect2;

#[derive(Component, Clone, Copy)]
pub struct PotionType(Element, Element);

#[derive(Component)]
pub struct EssenceCounter(Element);

#[derive(Component, Clone, Copy)]
struct ExplodePosition(Vec2);

#[derive(Bundle)]
pub struct PotionBundle {
    potion_type: PotionType,
    rigidbody: RigidBody,
    velocity: Velocity,
    collider: Collider,
    #[bundle]
    sprite: SpriteBundle,
    collision_group: CollisionGroups,
    active_events: ActiveEvents,
    explode_pos: ExplodePosition,
    sensor: Sensor,
    nfl: NotFromLevel,
}

#[derive(Default)]
pub struct PotionBrewData {
    pub direction: Vec2,
    pub position: Vec2,
    contents: (Option<Element>, Option<Element>),
}

#[derive(Default, PartialEq)]
pub enum PotionBrewState {
    Active,
    #[default]
    Inactive,
}

pub struct ThrowPotion(pub Element, pub Element);

struct PotionExplode {
    potion_type: PotionType,
    transform: Transform,
    velocity: Velocity,
}

fn fire_fire(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("fire_fire.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(64.0), 6, 1))
            },
            UniformAnim(Timer::from_seconds(0.05, true)),
            DespawnTimer(Timer::from_seconds(0.3, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(FIRE_FIRE_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    Hitstun(FIRE_FIRE_HITSTUN),
                    RadialImpulse::new(FIRE_FIRE_IMPULSE, Falloff::none()),
                    DamageOnce::new(FIRE_FIRE_DAMAGE, Falloff::none()),
                    DespawnTimer(Timer::from_seconds(0.1, false)),
                ));
        });
}

fn water_water(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
    velocity: &Velocity,
) {
    let direction = velocity.linvel.normalize();
    let tex = assets.load("water_water.png");
    let atlas = atlases.add(TextureAtlas::from_grid(tex, Vec2::new(8.0, 16.0), 12, 1));
    spawned
        .insert_bundle((
            Velocity {
                linvel: direction * WATER_WATER_SPEED,
                angvel: 0.0,
            },
            RigidBody::KinematicVelocityBased,
            DespawnTimer(Timer::from_seconds(WATER_WATER_DURATION, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_axis_angle(
                            Vec3::Z,
                            direction.y.atan2(direction.x) + std::f32::consts::PI / 2.0,
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert_bundle((
                    Collider::cuboid(WATER_WATER_HALF_WIDTH, WATER_WATER_HALF_HEIGHT),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    DirectedForce::new(direction * WATER_WATER_FORCE),
                ));

            for i in -8..=8 {
                parent
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: atlas.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                -direction.y * 4.0 * i as f32,
                                direction.x * 4.0 * i as f32,
                                -direction.x * 0.1e-5 * i as f32,
                            ),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(UniformAnim(Timer::from_seconds(0.05, true)));
            }
        });
}

fn wind_wind(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("wind_wind.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(64.0), 4, 1))
            },
            UniformAnim(Timer::from_seconds(0.05, true)),
            DespawnTimer(Timer::from_seconds(WIND_WIND_DURATION, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(WIND_WIND_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    RadialForce::new(
                        -WIND_WIND_FORCE,
                        Falloff::new(
                            WIND_WIND_MAX_FALLOFF,
                            WIND_WIND_FALLOFF_START,
                            WIND_WIND_FALLOFF_END,
                        ),
                    ),
                ));
        });
}

fn lightning_lightning(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            TextureAtlasSprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            {
                let tex = assets.load("lightning_lightning.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::new(16.0, 64.0), 4, 1))
            },
            UniformAnim(Timer::from_seconds(0.05, true)),
            DespawnTimer(Timer::from_seconds(0.25, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(LIGHTNING_LIGHTNING_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    Hitstun(LIGHTNING_LIGHTNING_HITSTUN),
                    DamageOnce::new(
                        LIGHTNING_LIGHTNING_DAMAGE,
                        Falloff::new(
                            LIGHTNING_LIGHTNING_MAX_FALLOFF,
                            LIGHTNING_LIGHTNING_FALLOFF_START,
                            LIGHTNING_LIGHTNING_FALLOFF_END,
                        ),
                    ),
                    DespawnTimer(Timer::from_seconds(0.1, false)),
                ));
        });
}

fn earth_earth(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("earth_earth.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 40, 1))
            },
            UniformAnim(Timer::from_seconds(0.05, true)),
            DespawnTimer(Timer::from_seconds(EARTH_EARTH_DURATION, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(EARTH_EARTH_RADIUS),
                    CollisionGroups {
                        memberships: WALL_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP
                            | PLAYER_COLLISION_GROUP
                            | PLAYER_ATTACK_COLLISION_GROUP
                            | ENEMY_ATTACK_COLLISION_GROUP,
                    },
                ));
        });
}

fn fire_water(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    //TODO: art
    spawned
        .insert_bundle((
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("fire_water.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(64.0), 5, 1))
            },
            UniformAnim(Timer::from_seconds(0.1, true)),
            DespawnTimer(Timer::from_seconds(FIRE_WATER_DURATION, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(FIRE_WATER_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    RadialForce::new(
                        FIRE_WATER_FORCE,
                        Falloff::new(
                            FIRE_WATER_MAX_FALLOFF,
                            FIRE_WATER_FALLOFF_START,
                            FIRE_WATER_FALLOFF_END,
                        ),
                    ),
                ));
        });
}

fn fire_wind(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("fire_wind.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(96.0), 3, 1))
            },
            UniformAnim(Timer::from_seconds(0.05, true)),
            DespawnTimer(Timer::from_seconds(0.15, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(FIRE_WIND_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    StatusEffect(Effect::OnFire),
                    DespawnTimer(Timer::from_seconds(0.5, false)),
                ));
        });
}

fn fire_lightning(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("fire_lightning.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(16.0), 2, 1))
            },
            UniformAnim(Timer::from_seconds(0.1, true)),
            DespawnTimer(Timer::from_seconds(0.2, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(FIRE_LIGHTNING_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    StatusEffect(Effect::DelayedExplosion),
                    DespawnTimer(Timer::from_seconds(FIRE_LIGHTNING_DURATION, false)),
                ));
        });
}

fn fire_earth(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("fire_earth.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(96.0), 14, 1))
            },
            UniformAnim(Timer::from_seconds(0.1, true)),
            DespawnTimer(Timer::from_seconds(FIRE_EARTH_DURATION, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(FIRE_EARTH_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    DamagePeriodic::new(FIRE_EARTH_DAMAGE, Falloff::none(), FIRE_EARTH_TICK),
                ));
        });
}

fn water_wind(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            Velocity::default(),
            RigidBody::KinematicVelocityBased,
            Homing {
                max_speed: WATER_WIND_CHASE_SPEED,
            },
            Sensor,
            Collider::ball(WATER_WIND_CHASE_RADIUS),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups {
                memberships: PLAYER_ATTACK_COLLISION_GROUP,
                filters: ENEMY_COLLISION_GROUP,
            },
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("water_wind.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(64.0), 7, 1))
            },
            UniformAnim(Timer::from_seconds(0.05, true)),
            DespawnTimer(Timer::from_seconds(WATER_WIND_DURATION, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(WATER_WIND_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    StatusEffect(Effect::Slowed),
                ));
        });
}

fn water_lightning(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("water_lightning.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(96.0), 3, 1))
            },
            UniformAnim(Timer::from_seconds(0.05, true)),
            DespawnTimer(Timer::from_seconds(0.15, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(WATER_LIGHTNING_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    StatusEffect(Effect::Shocked),
                    DespawnTimer(Timer::from_seconds(WATER_LIGHTNING_DURATION, false)),
                ));
        });
}

fn water_earth(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
    velocity: &Velocity,
) {
    let direction = velocity.linvel.normalize();
    let tex = assets.load("water_earth.png");
    let atlas = atlases.add(TextureAtlas::from_grid(tex, Vec2::new(16.0, 16.0), 15, 1));
    spawned
        .insert(DespawnTimer(Timer::from_seconds(
            WATER_EARTH_DURATION,
            false,
        )))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_axis_angle(
                            Vec3::Z,
                            direction.y.atan2(direction.x) + std::f32::consts::PI / 2.0,
                        ),
                        translation: (direction * WATER_EARTH_HALF_HEIGHT).extend(0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert_bundle((
                    Collider::cuboid(WATER_EARTH_HALF_WIDTH, WATER_EARTH_HALF_HEIGHT),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    Hitstun(WATER_EARTH_HITSTUN),
                    DamagePeriodic::new(WATER_EARTH_DAMAGE, Falloff::none(), WATER_EARTH_TICK),
                ));

            for i in 0..=1 {
                for j in 1..=16 {
                    parent
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: atlas.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    direction.x * 6.0 * j as f32
                                        - direction.y * 8.0 * (i as f32 - 0.5),
                                    direction.y * 6.0 * j as f32
                                        + direction.x * 8.0 * (i as f32 - 0.5),
                                    direction.y * 0.1e-5 * j as f32,
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(UniformAnimOnce(Timer::from_seconds(0.1, true)));
                }
            }
        });
}

fn wind_lightning(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            Velocity::default(),
            RigidBody::KinematicVelocityBased,
            Homing {
                max_speed: WIND_LIGHTNING_CHASE_SPEED,
            },
            Sensor,
            Collider::ball(WIND_LIGHTNING_CHASE_RADIUS),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups {
                memberships: PLAYER_ATTACK_COLLISION_GROUP,
                filters: ENEMY_COLLISION_GROUP,
            },
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("wind_lightning.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(64.0), 6, 1))
            },
            UniformAnim(Timer::from_seconds(0.05, true)),
            DespawnTimer(Timer::from_seconds(WIND_LIGHTNING_DURATION, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(WIND_LIGHTNING_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    DamagePeriodic::new(
                        WIND_LIGHTNING_DAMAGE,
                        Falloff::none(),
                        WIND_LIGHTNING_TICK,
                    ),
                ));
        });
}

fn wind_earth(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        .insert_bundle((
            TextureAtlasSprite::default(),
            {
                let tex = assets.load("wind_earth.png");
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(96.0), 3, 1))
            },
            UniformAnim(Timer::from_seconds(0.05, true)),
            DespawnTimer(Timer::from_seconds(0.15, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(WIND_EARTH_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    StatusEffect(Effect::Blinded),
                    DespawnTimer(Timer::from_seconds(WIND_EARTH_DURATION, false)),
                ));
        });
}

fn lightning_earth(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
    rotation: f32,
) {
    let tex = assets.load("lightning_earth.png");
    let atlas = atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(4.0), 4, 1));
    //TODO: spawned just has initial strike art
    for i in 0..LIGHTNING_EARTH_COUNT {
        let rotation = rotation + (std::f32::consts::TAU / LIGHTNING_EARTH_COUNT as f32) * i as f32;
        spawned.with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    TextureAtlasSprite::default(),
                    atlas.clone(),
                    UniformAnim(Timer::from_seconds(0.05, true)),
                    Velocity {
                        linvel: Vec2::from_angle(rotation) * LIGHTNING_EARTH_SPEED,
                        angvel: 0.0,
                    },
                    RigidBody::Dynamic,
                    Collider::ball(LIGHTNING_EARTH_RADIUS),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP | WALL_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Hitbox,
                    Hitstun(LIGHTNING_EARTH_HITSTUN),
                    DamageOnce::new(LIGHTNING_EARTH_DAMAGE, Falloff::none()),
                    DespawnTimer(Timer::from_seconds(LIGHTNING_EARTH_DURATION, false)),
                    LockedAxes::ROTATION_LOCKED,
                    Ccd::enabled(),
                ));
        });
    }
}

pub struct Plugin;
impl Plugin {
    fn init(
        mut cmd: Commands,
        assets: Res<AssetServer>,
        element_icons: ResMut<ElementIconAtlases>,
        digits: Res<Digits>,
    ) {
        cmd.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(BREW_UI_SIZE)),
                ..default()
            },
            texture: assets.load("select_circle.png"),
            visibility: Visibility { is_visible: false },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 999.0),
                ..default()
            },
            ..default()
        })
        .insert(PotionBrewUi)
        .with_children(|root| {
            root.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(-BREW_UI_ICON_SIZE, 0.0, 0.1),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                    ..default()
                },
                ..default()
            })
            .insert_bundle((
                UniformAnim(Timer::from_seconds(0.1, true)),
                TimeIndependent,
                PotionUiSelect1,
            ));
            root.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(BREW_UI_ICON_SIZE, 0.0, 0.1),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                    ..default()
                },
                ..default()
            })
            .insert_bundle((
                UniformAnim(Timer::from_seconds(0.1, true)),
                TimeIndependent,
                PotionUiSelect2,
            ));
            for i in 0..5 {
                let angle = std::f32::consts::PI * (1.5 - (2 * i) as f32) / 5.0;

                root.spawn_bundle(SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            angle.cos() * BREW_UI_ICON_DISTANCE,
                            angle.sin() * BREW_UI_ICON_DISTANCE,
                            0.5,
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert_bundle((
                    element_icons[i].clone_weak(),
                    TextureAtlasSprite {
                        custom_size: Some(Vec2::splat(BREW_UI_ICON_SIZE)),
                        ..default()
                    },
                    UniformAnim(Timer::from_seconds(0.1, true)),
                    TimeIndependent,
                ))
                .with_children(|root| {
                    root.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: digits.clone_weak(),
                        transform: Transform {
                            translation: Vec3::new(0.0, -9.0, 0.0),
                            ..default()
                        },
                        sprite: TextureAtlasSprite {
                            color: Color::RED,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(EssenceCounter(match i {
                        0 => Element::Fire,
                        1 => Element::Water,
                        2 => Element::Wind,
                        3 => Element::Lightning,
                        4 => Element::Earth,
                        _ => unreachable!(),
                    }));
                });
            }
        });
    }

    fn throw_potion(
        mut cmd: Commands,
        assets: Res<AssetServer>,
        mut event_reader: EventReader<ThrowPotion>,
        q_player: Query<&Transform, With<Player>>,
        brew_data: Res<PotionBrewData>,
    ) {
        let player_transform = match q_player.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };
        for ThrowPotion(e1, e2) in event_reader.iter() {
            cmd.spawn_bundle(PotionBundle {
                potion_type: PotionType(*e1, *e2),
                rigidbody: RigidBody::Dynamic,
                velocity: Velocity {
                    linvel: brew_data.direction * POTION_THROW_SPEED,
                    angvel: POTION_SPIN_SPEED,
                },
                collider: Collider::ball(2.0),
                sprite: SpriteBundle {
                    texture: assets.load("bottle.png"),
                    transform: Transform {
                        translation: player_transform.translation,
                        ..default()
                    },
                    ..default()
                },
                collision_group: CollisionGroups {
                    memberships: PLAYER_ATTACK_COLLISION_GROUP,
                    filters: WALL_COLLISION_GROUP,
                },
                active_events: ActiveEvents::COLLISION_EVENTS,
                explode_pos: ExplodePosition(brew_data.position),
                sensor: Sensor,
                nfl: NotFromLevel,
            });
        }
    }

    fn update_brew(
        q_brew_ui: Query<&Transform, With<PotionBrewUi>>,
        mut q_brew_display1: Query<
            (&mut TextureAtlasSprite, &mut Handle<TextureAtlas>),
            (With<PotionUiSelect1>, Without<PotionUiSelect2>),
        >,
        mut q_brew_display2: Query<
            &mut Handle<TextureAtlas>,
            (Without<PotionUiSelect1>, With<PotionUiSelect2>),
        >,
        mut event_writer: EventWriter<ThrowPotion>,
        mouse_pos: Res<MousePosition>,
        mouse_buttons: Res<Input<MouseButton>>,
        element_icons: ResMut<ElementIconAtlases>,
        mut brew_data: ResMut<PotionBrewData>,
        mut brew_state: ResMut<PotionBrewState>,
        mut counts: ResMut<EssenceCounts>,
    ) {
        if *brew_state != PotionBrewState::Active {
            return;
        }
        let (mut sprite1, mut handle1) = q_brew_display1.single_mut();
        let mut handle2 = q_brew_display2.single_mut();

        let relative_mouse_pos = (mouse_pos.0 - q_brew_ui.single().translation).truncate();

        let angle = relative_mouse_pos.angle_between(Vec2::NEG_Y) + std::f32::consts::PI;

        let index = (angle / (std::f32::consts::TAU / 5.0)) as usize % 5;
        let element = match index {
            0 => Element::Fire,
            1 => Element::Water,
            2 => Element::Wind,
            3 => Element::Lightning,
            4 => Element::Earth,
            _ => unreachable!(),
        };

        if mouse_buttons.just_pressed(MouseButton::Left) {
            if relative_mouse_pos.length() > BREW_UI_SIZE / 2.0 {
                *brew_state = PotionBrewState::Inactive;
                return;
            } else if relative_mouse_pos.length() > BREW_UI_DEADZONE {
                if counts[element] != 0 {
                    *counts.get_mut(&element).unwrap() -= 1;
                    match brew_data.contents {
                        (None, None) => {
                            brew_data.contents.0 = Some(element);
                            sprite1.color.set_a(1.0);
                            *handle1 = element_icons[index].clone_weak();
                        }
                        (Some(first), None) => {
                            brew_data.contents.1 = Some(element);
                            event_writer.send(ThrowPotion(first, element));
                            *brew_state = PotionBrewState::Inactive;
                        }
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            let out_of_zone = relative_mouse_pos.length() > BREW_UI_SIZE / 2.0
                || relative_mouse_pos.length() < BREW_UI_DEADZONE;
            match brew_data.contents {
                (None, None) => {
                    if out_of_zone {
                        *handle1 = Handle::<TextureAtlas>::default();
                    } else {
                        *handle1 = element_icons[index].clone_weak();
                        sprite1.color.set_a(0.5);
                    }
                }
                (Some(_), None) => {
                    if out_of_zone {
                        *handle2 = Handle::<TextureAtlas>::default();
                    } else {
                        *handle2 = element_icons[index].clone_weak();
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn manage_brew_state(
        brew_state: Res<PotionBrewState>,
        mut brew_data: ResMut<PotionBrewData>,
        mut time_scale: ResMut<TimeScale>,
        mut q_brew_ui: Query<&mut Visibility, With<PotionBrewUi>>,
        mut q_brew_display1: Query<
            &mut Handle<TextureAtlas>,
            (With<PotionUiSelect1>, Without<PotionUiSelect2>),
        >,
        mut q_brew_display2: Query<
            &mut Handle<TextureAtlas>,
            (Without<PotionUiSelect1>, With<PotionUiSelect2>),
        >,
        mut counts: ResMut<EssenceCounts>,
    ) {
        if brew_state.is_changed() && *brew_state == PotionBrewState::Active {
            **time_scale = 0.01;
            q_brew_ui.single_mut().is_visible = true;
        } else if brew_state.is_changed() && *brew_state == PotionBrewState::Inactive {
            **time_scale = 1.0;
            q_brew_ui.single_mut().is_visible = false;

            if brew_data.contents.1.is_none() {
                if let Some(element) = brew_data.contents.0 {
                    *counts.get_mut(&element).unwrap() += 1;
                }
            }
            brew_data.contents = (None, None);
            *q_brew_display1.single_mut() = Handle::<TextureAtlas>::default();
            *q_brew_display2.single_mut() = Handle::<TextureAtlas>::default();
        }
    }

    fn potion_explode(
        mut cmd: Commands,
        mut event_reader: EventReader<CollisionEvent>,
        q_potion: Query<(Entity, &PotionType, &Transform, &Velocity, &ExplodePosition)>,
        mut event_writer: EventWriter<PotionExplode>,
    ) {
        for event in event_reader.iter() {
            match event {
                CollisionEvent::Started(e1, e2, _) => {
                    if let Ok((_, &potion_type, &transform, &velocity, _)) = q_potion.get(*e1) {
                        event_writer.send(PotionExplode {
                            potion_type,
                            transform,
                            velocity,
                        });
                        cmd.entity(*e1).despawn_recursive();
                    } else if let Ok((_, &potion_type, &transform, &velocity, _)) =
                        q_potion.get(*e2)
                    {
                        event_writer.send(PotionExplode {
                            potion_type,
                            transform,
                            velocity,
                        });
                        cmd.entity(*e2).despawn_recursive();
                    } else {
                        continue;
                    }
                }
                _ => (),
            }
        }

        for (entity, &potion_type, &transform, &velocity, &position) in &q_potion {
            let direction = position.0 - transform.translation.truncate();
            if velocity.linvel.dot(direction) <= 0.0 {
                event_writer.send(PotionExplode {
                    potion_type,
                    transform,
                    velocity,
                });
                cmd.entity(entity).despawn_recursive();
            }
        }
    }

    fn potion_effect(
        mut cmd: Commands,
        mut event_reader: EventReader<PotionExplode>,
        assets: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        for event in event_reader.iter() {
            let transform = event.transform;
            let potion_type = &event.potion_type;
            let velocity = event.velocity;
            let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
            let mut spawned = cmd.spawn_bundle(SpatialBundle {
                transform: Transform {
                    rotation: Quat::IDENTITY,
                    ..transform
                },
                ..default()
            });
            spawned.insert(NotFromLevel);

            {
                use Element::*;
                match (potion_type.0, potion_type.1) {
                    (Fire, Fire) => {
                        fire_fire(&mut spawned, &assets, &mut atlases);
                    }
                    (Water, Water) => {
                        water_water(&mut spawned, &assets, &mut atlases, &velocity);
                    }
                    (Wind, Wind) => {
                        wind_wind(&mut spawned, &assets, &mut atlases);
                    }
                    (Lightning, Lightning) => {
                        lightning_lightning(&mut spawned, &assets, &mut atlases);
                    }
                    (Earth, Earth) => {
                        earth_earth(&mut spawned, &assets, &mut atlases);
                        //big rock just sprouts and blocks stuff
                    }
                    (Fire, Water) | (Water, Fire) => {
                        fire_water(&mut spawned, &assets, &mut atlases);
                        //steam geyser - shoves away
                    }
                    (Fire, Wind) | (Wind, Fire) => {
                        fire_wind(&mut spawned, &assets, &mut atlases);
                        //sets things on fire (big area, dot)
                    }
                    (Fire, Lightning) | (Lightning, Fire) => {
                        fire_lightning(&mut spawned, &assets, &mut atlases);
                        //delayed explosion, sticks to 1 enemy
                    }
                    (Fire, Earth) | (Earth, Fire) => {
                        fire_earth(&mut spawned, &assets, &mut atlases);
                        //Damaging lava puddle
                    }
                    (Water, Wind) | (Wind, Water) => {
                        water_wind(&mut spawned, &assets, &mut atlases);
                        //homing rain cloud - slows enemies under it
                    }
                    (Water, Lightning) | (Lightning, Water) => {
                        water_lightning(&mut spawned, &assets, &mut atlases);
                        //Affected enemies shoot lightning at nearby enemies
                    }
                    (Water, Earth) | (Earth, Water) => {
                        water_earth(&mut spawned, &assets, &mut atlases, &velocity);
                        //grows vines on the ground, damaging enemies that walk through
                    }
                    (Wind, Lightning) | (Lightning, Wind) => {
                        wind_lightning(&mut spawned, &assets, &mut atlases);
                        //homing storm cloud
                    }
                    (Wind, Earth) | (Earth, Wind) => {
                        wind_earth(&mut spawned, &assets, &mut atlases);
                        //dust storm - blinds
                    }
                    (Lightning, Earth) | (Earth, Lightning) => {
                        lightning_earth(&mut spawned, &assets, &mut atlases, rotation);
                        //lightning strikes at location, sparks go through ground back to player
                    }
                }
            }
        }
    }

    fn update_counter(
        mut q_counter: Query<(&mut TextureAtlasSprite, &EssenceCounter)>,
        amounts: Res<EssenceCounts>,
    ) {
        if amounts.is_changed() {
            for (mut sprite, essence) in &mut q_counter {
                let count = *amounts.get(&essence.0).unwrap() as usize;
                sprite.index = count;
                if count == 0 {
                    sprite.color = Color::RED;
                } else if count == 5 {
                    sprite.color = Color::YELLOW;
                } else {
                    sprite.color = Color::WHITE;
                }
            }
        }
    }

    fn cleanup(mut cmd: Commands, q_brew_ui: Query<Entity, With<PotionBrewUi>>) {
        cmd.entity(q_brew_ui.single()).despawn_recursive();
    }
}
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init)
            .add_exit_system(GameState::InGame, Self::cleanup)
            .add_system(Self::throw_potion.run_in_state(GameState::InGame))
            .add_system(
                Self::manage_brew_state
                    .run_in_state(GameState::InGame)
                    .run_not_in_state(PauseState::Paused),
            )
            .add_system(Self::update_brew.run_in_state(GameState::InGame))
            .add_system(Self::potion_explode.run_in_state(GameState::InGame))
            .add_system(Self::potion_effect.run_in_state(GameState::InGame))
            .add_system(Self::update_counter.run_in_state(GameState::InGame))
            .init_resource::<PotionBrewData>()
            .init_resource::<PotionBrewState>()
            .add_event::<ThrowPotion>()
            .add_event::<PotionExplode>();
    }
}

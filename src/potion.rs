use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    consts::*,
    hitbox::{
        DamageOnce, DamagePeriodic, DirectedForce, Falloff, Hitbox, Hitstun, RadialForce,
        RadialImpulse,
    },
    player::Player,
    utils::{
        DespawnTimer, ElementIconAtlases, MousePosition, TimeIndependent, TimeScale, UniformAnim,
    },
    Element, GameState,
};

#[derive(Component)]
pub struct PotionBrewUi;

#[derive(Component)]
pub struct PotionUiSelect1;

#[derive(Component)]
pub struct PotionUiSelect2;

#[derive(Component, Debug)]
pub struct PotionType(Element, Element);

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
}

#[derive(Default)]
pub struct PotionBrewData {
    pub direction: Vec2,
    contents: (Option<Element>, Option<Element>),
}

#[derive(Default, PartialEq)]
pub enum PotionBrewState {
    Active,
    #[default]
    Inactive,
}
pub struct ThrowPotion(pub Element, pub Element);

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
                atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 5, 1))
            },
            UniformAnim(Timer::from_seconds(0.1, true)),
            DespawnTimer(Timer::from_seconds(0.5, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(32.0),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    Hitstun(0.5),
                    RadialImpulse::new(25.0, Falloff::none()),
                    DamageOnce::new(25.0, Falloff::none()),
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
    //TODO: art
    let direction = velocity.linvel.normalize();
    spawned
        .insert_bundle((
            Velocity {
                linvel: direction * 100.0,
                angvel: 0.0,
            },
            RigidBody::KinematicVelocityBased,
            DespawnTimer(Timer::from_seconds(0.6, false)),
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
                    Collider::cuboid(32.0, 8.0),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    DirectedForce::new(direction * 15.0),
                    DespawnTimer(Timer::from_seconds(0.3, false)),
                ));
        });
}

fn wind_wind(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    //TODO: art
    spawned
        .insert_bundle((
            //     TextureAtlasSprite::default(),
            //     {
            //         let tex = assets.load("fire_fire.png");
            //         atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 5, 1))
            //     },
            //     UniformAnim(Timer::from_seconds(0.1, true)),
            DespawnTimer(Timer::from_seconds(2.0, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(32.0),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    Hitstun(0.1),
                    RadialForce::new(-5.0, Falloff::new(0.5, 5.0, 32.0)),
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
            // TextureAtlasSprite::default(),
            // {
            //     let tex = assets.load("fire_fire.png");
            //     atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 5, 1))
            // },
            // UniformAnim(Timer::from_seconds(0.1, true)),
            // DespawnTimer(Timer::from_seconds(0.5, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(2.0),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    Hitstun(3.0),
                    DamageOnce::new(250.0, Falloff::none()),
                    DespawnTimer(Timer::from_seconds(0.05, false)),
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
            // TextureAtlasSprite::default(),
            // {
            //     let tex = assets.load("fire_fire.png");
            //     atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 5, 1))
            // },
            // UniformAnim(Timer::from_seconds(0.1, true)),
            // DespawnTimer(Timer::from_seconds(0.5, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(16.0),
                    CollisionGroups {
                        memberships: WALL_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP
                            | PLAYER_COLLISION_GROUP
                            | PLAYER_ATTACK_COLLISION_GROUP
                            | ENEMY_ATTACK_COLLISION_GROUP,
                    },
                    DespawnTimer(Timer::from_seconds(2.0, false)),
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
            //     TextureAtlasSprite::default(),
            //     {
            //         let tex = assets.load("fire_fire.png");
            //         atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 5, 1))
            //     },
            //     UniformAnim(Timer::from_seconds(0.1, true)),
            DespawnTimer(Timer::from_seconds(2.0, false)),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(32.0),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    RadialForce::new(5.0, Falloff::new(0.5, 5.0, 32.0)),
                ));
        });
}

fn fire_wind(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
}

fn fire_lightning(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
}

fn fire_earth(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    spawned
        // .insert_bundle((
        //     TextureAtlasSprite::default(),
        //     {
        //         let tex = assets.load("fire_fire.png");
        //         atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 5, 1))
        //     },
        //     UniformAnim(Timer::from_seconds(0.1, true)),
        //     DespawnTimer(Timer::from_seconds(0.5, false)),
        // ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Collider::ball(48.0),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    DamagePeriodic::new(5.0, Falloff::none(), 0.1),
                    DespawnTimer(Timer::from_seconds(1.5, false)),
                ));
        });
}

fn water_wind(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
}

fn water_lightning(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
}

fn water_earth(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
    velocity: &Velocity,
) {
    let direction = velocity.linvel.normalize();
    spawned
        // .insert_bundle((
        //     TextureAtlasSprite::default(),
        //     {
        //         let tex = assets.load("fire_fire.png");
        //         atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 5, 1))
        //     },
        //     UniformAnim(Timer::from_seconds(0.1, true)),
        //     DespawnTimer(Timer::from_seconds(0.5, false)),
        // ))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_axis_angle(
                            Vec3::Z,
                            direction.y.atan2(direction.x) + std::f32::consts::PI / 2.0,
                        ),
                        translation: (direction * 48.0).extend(0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert_bundle((
                    Collider::cuboid(16.0, 48.0),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Sensor,
                    Hitbox,
                    Hitstun(0.5),
                    DamagePeriodic::new(5.0, Falloff::none(), 0.1),
                    DespawnTimer(Timer::from_seconds(1.5, false)),
                ));
        });
}

fn wind_lightning(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
}

fn wind_earth(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
}

fn lightning_earth(
    spawned: &mut EntityCommands,
    assets: &Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
    rotation: f32,
) {
    //TODO: spawned just has initial strike art
    for i in 0..7 {
        let rotation = rotation + std::f32::consts::TAU / 7.0 * i as f32;
        spawned.with_children(|parent| {
            parent
                .spawn_bundle(SpatialBundle::default())
                .insert_bundle((
                    Velocity {
                        linvel: Vec2::from_angle(rotation) * 600.0,
                        angvel: 0.0,
                    },
                    RigidBody::Dynamic,
                    Collider::ball(2.0),
                    CollisionGroups {
                        memberships: PLAYER_ATTACK_COLLISION_GROUP,
                        filters: ENEMY_COLLISION_GROUP | WALL_COLLISION_GROUP,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Hitbox,
                    Hitstun(1.5),
                    DamageOnce::new(5.0, Falloff::none()),
                    DespawnTimer(Timer::from_seconds(0.6, false)),
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    bevy::render::texture::DEFAULT_IMAGE_HANDLE.typed::<Image>(),
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
                ));
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
                    filters: ENEMY_COLLISION_GROUP | WALL_COLLISION_GROUP,
                },
                active_events: ActiveEvents::COLLISION_EVENTS,
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
    ) {
        if brew_state.is_changed() && *brew_state == PotionBrewState::Active {
            **time_scale = 0.01;
            q_brew_ui.single_mut().is_visible = true;
        } else if brew_state.is_changed() && *brew_state == PotionBrewState::Inactive {
            **time_scale = 1.0;
            q_brew_ui.single_mut().is_visible = false;
            brew_data.contents = (None, None);
            *q_brew_display1.single_mut() = Handle::<TextureAtlas>::default();
            *q_brew_display2.single_mut() = Handle::<TextureAtlas>::default();
        }
    }

    fn potion_explode(
        mut cmd: Commands,
        mut event_reader: EventReader<CollisionEvent>,
        q_potion: Query<(&PotionType, &Transform, &Velocity)>,
        q_player: Query<&Transform, With<Player>>,
        assets: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        for event in event_reader.iter() {
            match event {
                CollisionEvent::Started(e1, e2, _) => {
                    let potion_type;
                    let location;
                    let rotation;
                    let velocity;

                    if let Ok((p, t, v)) = q_potion.get(*e1) {
                        cmd.entity(*e1).despawn_recursive();
                        potion_type = p;
                        location = t.translation;
                        rotation = t.rotation.to_euler(EulerRot::XYZ).2;
                        velocity = v;
                    } else if let Ok((p, t, v)) = q_potion.get(*e2) {
                        cmd.entity(*e2).despawn_recursive();
                        potion_type = p;
                        location = t.translation;
                        rotation = t.rotation.to_euler(EulerRot::XYZ).2;
                        velocity = v;
                    } else {
                        continue;
                    }

                    let mut spawned = cmd.spawn_bundle(SpatialBundle {
                        transform: Transform {
                            translation: location,
                            ..default()
                        },
                        ..default()
                    });
                    {
                        use Element::*;
                        match (potion_type.0, potion_type.1) {
                            (Fire, Fire) => {
                                fire_fire(&mut spawned, &assets, &mut atlases);
                            }
                            (Water, Water) => {
                                water_water(&mut spawned, &assets, &mut atlases, velocity);
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
                                water_earth(&mut spawned, &assets, &mut atlases, velocity);
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
                _ => (),
            }
        }
    }
}
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init)
            .add_system(Self::throw_potion.run_in_state(GameState::InGame))
            .add_system(Self::manage_brew_state.run_in_state(GameState::InGame))
            .add_system(Self::update_brew.run_in_state(GameState::InGame))
            .add_system(Self::potion_explode.run_in_state(GameState::InGame))
            .init_resource::<PotionBrewData>()
            .init_resource::<PotionBrewState>()
            .add_event::<ThrowPotion>();
    }
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    consts::*,
    hitbox::{Hitbox, Hitstun, RadialImpulse},
    player::Player,
    utils::{DespawnTimer, MousePosition, TimeIndependent, TimeScale, UniformAnim},
    Element, GameState,
};

#[derive(Component)]
pub struct PotionBrewUi;

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

pub struct Plugin;
impl Plugin {
    fn init(
        mut cmd: Commands,
        assets: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
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
            let data = [
                //Fire
                {
                    let tex = assets.load("fire_essence.png");
                    atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(8.0), 5, 1))
                },
                //Water
                {
                    let tex = assets.load("water_essence.png");
                    atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(8.0), 5, 1))
                },
                //Wind
                {
                    let tex = assets.load("wind_essence.png");
                    atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(8.0), 5, 1))
                },
                //Lightning
                {
                    let tex = assets.load("lightning_essence.png");
                    atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(8.0), 5, 1))
                },
                //Earth
                {
                    let tex = assets.load("earth_essence.png");
                    atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(8.0), 5, 1))
                },
            ];
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
                    data[i].to_owned(),
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
        mut brew_data: ResMut<PotionBrewData>,
        mut brew_state: ResMut<PotionBrewState>,
        mouse_pos: Res<MousePosition>,
        mouse_buttons: Res<Input<MouseButton>>,
        q_brew_ui: Query<&Transform, With<PotionBrewUi>>,
        mut event_writer: EventWriter<ThrowPotion>,
    ) {
        if *brew_state != PotionBrewState::Active {
            return;
        }

        if mouse_buttons.just_pressed(MouseButton::Left) {
            let relative_mouse_pos = (mouse_pos.0 - q_brew_ui.single().translation).truncate();

            if relative_mouse_pos.length() > BREW_UI_SIZE / 2.0 {
                *brew_state = PotionBrewState::Inactive;
                return;
            } else if relative_mouse_pos.length() > BREW_UI_DEADZONE {
                let angle = relative_mouse_pos.angle_between(Vec2::NEG_Y) + std::f32::consts::PI;

                let element = match (angle / (std::f32::consts::TAU / 5.0)) as i32 {
                    0 => Element::Fire,
                    1 => Element::Water,
                    2 => Element::Wind,
                    3 => Element::Lightning,
                    4 => Element::Earth,
                    _ => unreachable!(),
                };

                match brew_data.contents {
                    (None, None) => brew_data.contents.0 = Some(element),
                    (Some(first), None) => {
                        brew_data.contents.1 = Some(element);
                        event_writer.send(ThrowPotion(first, element));
                        *brew_state = PotionBrewState::Inactive;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    fn manage_brew_state(
        brew_state: Res<PotionBrewState>,
        mut brew_data: ResMut<PotionBrewData>,
        mut time_scale: ResMut<TimeScale>,
        mut q_brew_ui: Query<&mut Visibility, With<PotionBrewUi>>,
    ) {
        if brew_state.is_changed() && *brew_state == PotionBrewState::Active {
            **time_scale = 0.01;
            q_brew_ui.single_mut().is_visible = true;
            brew_data.contents = (None, None);
        } else if brew_state.is_changed() && *brew_state == PotionBrewState::Inactive {
            **time_scale = 1.0;
            q_brew_ui.single_mut().is_visible = false;
        }
    }
    fn potion_explode(
        mut cmd: Commands,
        mut event_reader: EventReader<CollisionEvent>,
        q_potion: Query<(&PotionType, &Transform)>,
        assets: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        for event in event_reader.iter() {
            match event {
                CollisionEvent::Started(e1, e2, _) => {
                    let potion_type;
                    let location;

                    if let Ok((p, t)) = q_potion.get(*e1) {
                        cmd.entity(*e1).despawn_recursive();
                        potion_type = p;
                        location = t.translation;
                    } else if let Ok((p, t)) = q_potion.get(*e2) {
                        cmd.entity(*e2).despawn_recursive();
                        potion_type = p;
                        location = t.translation;
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
                                //big fireball - medium damage, large area
                                spawned
                                    .insert_bundle((
                                        TextureAtlasSprite::default(),
                                        {
                                            let tex = assets.load("fire_fire.png");
                                            atlases.add(TextureAtlas::from_grid(
                                                tex,
                                                Vec2::splat(32.0),
                                                5,
                                                1,
                                            ))
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
                                                RadialImpulse(75.0),
                                                DespawnTimer(Timer::from_seconds(0.1, false)),
                                            ));
                                    });
                            }
                            (Water, Water) => {
                                //big wave - pushes enemies in a direction away from player
                            }
                            (Wind, Wind) => {
                                //Tornado - pulls towards center
                            }
                            (Lightning, Lightning) => {
                                //lightning strike; big damage, small area
                            }
                            (Earth, Earth) => {
                                //big rock just sprouts and blocks stuff
                            }
                            (Fire, Water) | (Water, Fire) => {
                                //steam geyser - shoves away
                            }
                            (Fire, Wind) | (Wind, Fire) => {
                                //sets things on fire (big area, dot)
                            }
                            (Fire, Lightning) | (Lightning, Fire) => {
                                //delayed explosion, sticks to 1 enemy
                            }
                            (Fire, Earth) | (Earth, Fire) => {
                                //Damaging lava puddle
                            }
                            (Water, Wind) | (Wind, Water) => {
                                //homing rain cloud - slows enemies under it
                            }
                            (Water, Lightning) | (Lightning, Water) => {
                                //Affected enemies shoot lightning at nearby enemies
                            }
                            (Water, Earth) | (Earth, Water) => {
                                //grows vines on the ground, damaging enemies that walk through
                            }
                            (Wind, Lightning) | (Lightning, Wind) => {
                                //homing storm cloud
                            }
                            (Wind, Earth) | (Earth, Wind) => {
                                //dust storm - blinds
                            }
                            (Lightning, Earth) | (Earth, Lightning) => {
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

use std::ops::Index;

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    consts::*,
    health::HealthChange,
    player::Player,
    utils::{ElementIconAtlases, UniformAnim},
    Element, GameState,
};

#[derive(Deref, DerefMut)]
pub struct EssenceCounts(HashMap<Element, u32>);

impl Default for EssenceCounts {
    fn default() -> Self {
        EssenceCounts(HashMap::from([
            (Element::Fire, 0),
            (Element::Water, 0),
            (Element::Wind, 0),
            (Element::Lightning, 0),
            (Element::Earth, 0),
        ]))
    }
}

impl Index<Element> for EssenceCounts {
    type Output = u32;

    fn index(&self, index: Element) -> &Self::Output {
        &self.0[&index]
    }
}

#[derive(Component)]
pub struct Essence;

pub struct Plugin;

impl Plugin {
    fn setup(
        mut cmd: Commands,
        q_essence: Query<(Entity, &Element), Added<Essence>>,
        element_icons: Res<ElementIconAtlases>,
    ) {
        for (entity, element) in &q_essence {
            cmd.entity(entity).insert_bundle((
                UniformAnim(Timer::from_seconds(0.1, true)),
                Sensor,
                Collider::ball(1.0),
                TextureAtlasSprite::default(),
                CollisionGroups {
                    memberships: ESSENCE_COLLISION_GROUP,
                    filters: PLAYER_COLLISION_GROUP,
                },
                ActiveEvents::COLLISION_EVENTS,
                RigidBody::Fixed,
            ));
            match element {
                Element::Fire => {
                    cmd.entity(entity).insert(element_icons[0].clone());
                }
                Element::Water => {
                    cmd.entity(entity).insert(element_icons[1].clone());
                }
                Element::Wind => {
                    cmd.entity(entity).insert(element_icons[2].clone());
                }
                Element::Lightning => {
                    cmd.entity(entity).insert(element_icons[3].clone());
                }
                Element::Earth => {
                    cmd.entity(entity).insert(element_icons[4].clone());
                }
            }
        }
    }
    fn collect(
        mut cmd: Commands,
        mut event_reader: EventReader<CollisionEvent>,
        q_essence: Query<(&Element, Entity), (With<Essence>, Without<Player>)>,
        q_player: Query<Entity, With<Player>>,
        mut counts: ResMut<EssenceCounts>,
        mut event_writer: EventWriter<HealthChange>,
    ) {
        for event in event_reader.iter() {
            match event {
                CollisionEvent::Started(e1, e2, _) => {
                    let essence_data;
                    let player;
                    if let Ok(_) = q_player.get(*e1) {
                        if let Ok(data) = q_essence.get(*e2) {
                            essence_data = data;
                            player = *e1;
                        } else {
                            continue;
                        }
                    } else if let Ok(_) = q_player.get(*e2) {
                        if let Ok(data) = q_essence.get(*e1) {
                            essence_data = data;
                            player = *e2;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    let (element, essence) = essence_data;

                    if counts[*element] < 3 {
                        cmd.entity(essence).despawn_recursive();
                        *counts.get_mut(element).unwrap() += 1;
                        event_writer.send(HealthChange {
                            target: player,
                            amount: 10.0,
                        });
                    }
                }
                _ => (),
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::setup.run_in_state(GameState::InGame))
            .add_system(Self::collect.run_in_state(GameState::InGame))
            .init_resource::<EssenceCounts>();
    }
}

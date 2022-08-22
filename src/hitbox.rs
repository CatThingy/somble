use std::time::Duration;

use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{enemy::HitstunTimer, Enemy, GameState};

#[derive(Component)]
pub struct Hitbox;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Hitstun(pub f32);

#[derive(Component, Deref, DerefMut, Debug)]
pub struct RadialImpulse(pub f32);

#[derive(Component, Deref, DerefMut, Debug)]
pub struct DirectedImpulse(pub Vec2);

#[derive(Component, Debug)]
pub struct RadialForce {
    force: f32,
    hostages: HashSet<Entity>,
}

impl RadialForce {
    pub fn new(force: f32) -> Self {
        RadialForce {
            force,
            hostages: HashSet::new(),
        }
    }
}

#[derive(Component, Debug)]
pub struct DirectedForce {
    force: Vec2,
    hostages: HashSet<Entity>,
}

impl DirectedForce {
    pub fn new(force: Vec2) -> Self {
        DirectedForce {
            force,
            hostages: HashSet::new(),
        }
    }
}

pub struct Plugin;

impl Plugin {
    fn handle_hits(
        mut cmd: Commands,
        mut event_reader: EventReader<CollisionEvent>,
        mut q_enemy: Query<(&GlobalTransform, &mut HitstunTimer), (With<Enemy>, Without<Hitbox>)>,
        mut q_hitbox: Query<
            (
                &GlobalTransform,
                Option<&Hitstun>,
                Option<&RadialImpulse>,
                Option<&DirectedImpulse>,
                Option<&mut RadialForce>,
                Option<&mut DirectedForce>,
            ),
            (Without<Enemy>, With<Hitbox>),
        >,
    ) {
        for event in event_reader.iter() {
            match event {
                CollisionEvent::Started(e1, e2, _) => {
                    let hitbox_data;
                    let enemy_data;
                    let enemy_entity;
                    if let Ok(enemy) = q_enemy.get_mut(*e1) {
                        if let Ok(hitbox) = q_hitbox.get_mut(*e2) {
                            enemy_entity = e1;
                            enemy_data = enemy;
                            hitbox_data = hitbox;
                        } else {
                            continue;
                        }
                    } else if let Ok(enemy) = q_enemy.get_mut(*e2) {
                        if let Ok(hitbox) = q_hitbox.get_mut(*e1) {
                            enemy_entity = e2;
                            enemy_data = enemy;
                            hitbox_data = hitbox;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    let (enemy_transform, mut hitstun_timer) = enemy_data;
                    let (
                        hitbox_transform,
                        hitstun,
                        radial_impulse,
                        directed_impulse,
                        radial_force,
                        directed_force,
                    ) = hitbox_data;

                    if let Some(hitstun) = hitstun {
                        hitstun_timer.set_duration(Duration::from_secs_f32(**hitstun));
                        hitstun_timer.reset();
                    }
                    if let Some(radial_impulse) = radial_impulse {
                        let force_direction = (enemy_transform.translation()
                            - hitbox_transform.translation())
                        .truncate();

                        cmd.entity(*enemy_entity).insert(ExternalImpulse {
                            impulse: force_direction.normalize() * **radial_impulse,
                            torque_impulse: 0.0,
                        });
                    }

                    if let Some(directed_impulse) = directed_impulse {
                        cmd.entity(*enemy_entity).insert(ExternalImpulse {
                            impulse: **directed_impulse,
                            torque_impulse: 0.0,
                        });
                    }

                    if let Some(mut radial_force) = radial_force {
                        radial_force.hostages.insert(*enemy_entity);
                    }

                    if let Some(mut directed_force) = directed_force {
                        directed_force.hostages.insert(*enemy_entity);
                    }
                }
                CollisionEvent::Stopped(e1, e2, _) => {
                    let enemy_entity;
                    let hitbox_data;
                    if let Ok(_) = q_enemy.get(*e1) {
                        if let Ok(h) = q_hitbox.get_mut(*e2) {
                            enemy_entity = e1;
                            hitbox_data = h;
                        } else {
                            continue;
                        }
                    } else if let Ok(_) = q_enemy.get(*e2) {
                        if let Ok(h) = q_hitbox.get_mut(*e1) {
                            enemy_entity = e2;
                            hitbox_data = h;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    let (_, _, _, _, radial_force, directed_force) = hitbox_data;

                    if let Some(mut radial_force) = radial_force {
                        radial_force.hostages.remove(enemy_entity);
                    }
                    if let Some(mut directed_force) = directed_force {
                        directed_force.hostages.remove(enemy_entity);
                    }
                }
            }
        }
    }

    fn update_continuous_boxes(
        mut cmd: Commands,
        mut q_enemy: Query<
            (Entity, &GlobalTransform, &mut HitstunTimer),
            (With<Enemy>, Without<Hitbox>),
        >,
        mut q_hitbox: Query<
            (
                &GlobalTransform,
                Option<&RadialForce>,
                Option<&DirectedForce>,
            ),
            (Without<Enemy>, With<Hitbox>),
        >,
    ) {
        for (origin, radial_force, directed_force) in &mut q_hitbox {
            if let Some(radial_force) = radial_force {
                let mut iter = q_enemy.iter_many_mut(radial_force.hostages.iter());

                while let Some((entity, transform, mut hitstun)) = iter.fetch_next() {
                    // hitstun.reset();
                    let force_direction =
                        (transform.translation() - origin.translation()).truncate();
                    cmd.entity(entity).insert(ExternalImpulse {
                        impulse: force_direction.normalize() * radial_force.force,
                        torque_impulse: 0.0,
                    });
                }
            }

            if let Some(directed_force) = directed_force {
                let mut iter = q_enemy.iter_many_mut(directed_force.hostages.iter());

                while let Some((entity, transform, mut hitstun)) = iter.fetch_next() {
                    // hitstun.reset();
                    cmd.entity(entity).insert(ExternalImpulse {
                        impulse: directed_force.force,
                        torque_impulse: 0.0,
                    });
                }
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            Self::handle_hits
                .run_in_state(GameState::InGame)
                .label("handle_hits"),
        )
        .add_system(
            Self::update_continuous_boxes
                .run_in_state(GameState::InGame)
                .before("handle_hits"),
        );
    }
}

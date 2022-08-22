use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{enemy::HitstunTimer, Enemy, GameState};

#[derive(Component)]
pub struct Hitbox;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Hitstun(pub f32);

#[derive(Component, Deref, DerefMut, Debug)]
pub struct RadialImpulse(pub f32);

pub struct Plugin;

impl Plugin {
    fn handle_hits(
        mut cmd: Commands,
        mut event_reader: EventReader<CollisionEvent>,
        mut q_enemy: Query<(&GlobalTransform, &mut HitstunTimer), (With<Enemy>, Without<Hitbox>)>,
        q_hitbox: Query<
            (&GlobalTransform, Option<&Hitstun>, Option<&RadialImpulse>),
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
                        if let Ok(hitbox) = q_hitbox.get(*e2) {
                            enemy_entity = e1;
                            enemy_data = enemy;
                            hitbox_data = hitbox;
                        } else {
                            continue;
                        }
                    } else if let Ok(enemy) = q_enemy.get_mut(*e2) {
                        if let Ok(hitbox) = q_hitbox.get(*e1) {
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
                    let (hitbox_transform, hitstun, radial_impulse) = hitbox_data;

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
                }
                _ => (),
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::handle_hits.run_in_state(GameState::InGame));
    }
}

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::{Enemy, GameState};

#[derive(Component)]
pub struct Homing {
    pub max_speed: f32,
}

pub struct Plugin;

impl Plugin {
    fn calc_homing(
        mut q_homing: Query<(Entity, &GlobalTransform, &mut Velocity, &Homing), Without<Enemy>>,
        q_targets: Query<&GlobalTransform, (Without<Homing>, With<Enemy>)>,
        rapier_ctx: Res<RapierContext>,
    ) {
        for (entity, transform, mut velocity, homing) in &mut q_homing {
            let mut targets = vec![];
            for (e1, e2, _) in rapier_ctx.intersections_with(entity) {
                if e1 == entity {
                    if let Ok(transform) = q_targets.get(e2) {
                        targets.push(transform.translation());
                    }
                } else if e2 == entity {
                    if let Ok(transform) = q_targets.get(e1) {
                        targets.push(transform.translation());
                    }
                }
            }

            if targets.len() != 0 {
                let average =
                    targets.iter().fold(Vec2::ZERO, |a, v| a + v.truncate()) / targets.len() as f32;

                velocity.linvel = (average - transform.translation().truncate())
                    .clamp_length_max(homing.max_speed);
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::calc_homing.run_in_state(GameState::InGame));
    }
}

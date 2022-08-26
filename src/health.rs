use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Health {
            current: amount,
            max: amount,
        }
    }
}

#[derive(Component)]
pub struct Dead;

pub struct HealthChange {
    pub target: Entity,
    pub amount: f32,
}

pub struct Plugin;

impl Plugin {
    fn update_health(
        mut cmd: Commands,
        mut q_health: Query<(Entity, &mut Health)>,
        mut event_reader: EventReader<HealthChange>,
    ) {
        for event in event_reader.iter() {
            if let Ok((entity, mut health)) = q_health.get_mut(event.target) {
                health.current += event.amount;

                if health.current > health.max {
                    health.current = health.max;
                } else if health.current < 0.0 {
                    cmd.entity(entity).remove::<Health>().insert(Dead);
                    info!("yeah that's dead");
                }
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HealthChange>()
            .add_system(Self::update_health.run_in_state(GameState::InGame));
    }
}

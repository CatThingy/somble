use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{health::HealthChange, utils::TimeScale, GameState};

#[derive(Component)]
pub struct OnFire {
    duration: Timer,
    tick: Timer,
}

impl OnFire {
    pub fn new() -> Self {
        OnFire {
            duration: Timer::from_seconds(5.0, false),
            tick: Timer::from_seconds(0.5, true),
        }
    }
}

#[derive(Component)]
pub struct Shocked {
    duration: Timer,
    tick: Timer,
}

impl Shocked {
    pub fn new() -> Self {
        Shocked {
            duration: Timer::from_seconds(5.0, false),
            tick: Timer::from_seconds(0.5, true),
        }
    }
}

#[derive(Component)]
pub struct DelayedExplosion {
    duration: Timer,
}

impl DelayedExplosion {
    pub fn new() -> Self {
        DelayedExplosion {
            duration: Timer::from_seconds(5.0, false),
        }
    }
}

#[derive(Component)]
pub struct Blinded {
    duration: Timer,
}

impl Blinded {
    pub fn new() -> Self {
        Blinded {
            duration: Timer::from_seconds(5.0, false),
        }
    }
}

#[derive(Component)]
pub struct Slowed {
    duration: Timer,
}

impl Slowed {
    pub fn new() -> Self {
        Slowed {
            duration: Timer::from_seconds(5.0, false),
        }
    }
}

#[derive(Component, Clone)]
pub enum Effect {
    OnFire,
    Shocked,
    DelayedExplosion,
    Blinded,
    Slowed,
}

pub struct Plugin;

impl Plugin {
    fn insert_effect(mut cmd: Commands, affected: Query<(Entity, &Effect)>) {
        for (entity, effect) in &affected {
            let mut entity = cmd.entity(entity);
            entity.remove::<Effect>();
            match effect {
                Effect::OnFire => {
                    entity.insert(OnFire::new());
                }
                Effect::DelayedExplosion => {
                    entity.insert(DelayedExplosion::new());
                }
                Effect::Shocked => {
                    entity.insert(Shocked::new());
                }
                Effect::Blinded => {
                    entity.insert(Blinded::new());
                }
                Effect::Slowed => {
                    entity.insert(Slowed::new());
                }
            }
        }
    }

    fn tick_on_fire(
        mut cmd: Commands,
        mut q_on_fire: Query<(Entity, &mut OnFire)>,
        time: Res<Time>,
        time_scale: Res<TimeScale>,
        mut event_writer: EventWriter<HealthChange>,
    ) {
        let delta = time.delta().mul_f32(**time_scale);
        for (entity, mut on_fire) in &mut q_on_fire {
            on_fire.duration.tick(delta);
            on_fire.tick.tick(delta);

            if on_fire.tick.finished() {
                event_writer.send(HealthChange {
                    target: entity,
                    amount: 10.0,
                });
            }

            if on_fire.duration.finished() {
                cmd.entity(entity).remove::<OnFire>();
            }
        }
    }

    fn tick_shocked(
        mut cmd: Commands,
        mut q_shocked: Query<(Entity, &mut Shocked)>,
        time: Res<Time>,
        time_scale: Res<TimeScale>,
        mut event_writer: EventWriter<HealthChange>,
    ) {
        let delta = time.delta().mul_f32(**time_scale);
        for (entity, mut shocked) in &mut q_shocked {
            shocked.duration.tick(delta);
            shocked.tick.tick(delta);

            if shocked.tick.finished() {
                event_writer.send(HealthChange {
                    target: entity,
                    amount: 10.0,
                });
            }

            if shocked.duration.finished() {
                cmd.entity(entity).remove::<Shocked>();
            }
        }
    }

    fn tick_delayed_explosion(
        mut cmd: Commands,
        mut q_delayed_explosion: Query<(Entity, &mut DelayedExplosion)>,
        time: Res<Time>,
        time_scale: Res<TimeScale>,
    ) {
        let delta = time.delta().mul_f32(**time_scale);
        for (entity, mut delayed_explosion) in &mut q_delayed_explosion {
            delayed_explosion.duration.tick(delta);
            if delayed_explosion.duration.finished() {
                cmd.entity(entity).remove::<DelayedExplosion>();
            }
        }
    }

    fn tick_blinded(
        mut cmd: Commands,
        mut q_blinded: Query<(Entity, &mut Blinded)>,
        time: Res<Time>,
        time_scale: Res<TimeScale>,
    ) {
        let delta = time.delta().mul_f32(**time_scale);
        for (entity, mut blinded) in &mut q_blinded {
            blinded.duration.tick(delta);
            if blinded.duration.finished() {
                cmd.entity(entity).remove::<Blinded>();
            }
        }
    }

    fn tick_slowed(
        mut cmd: Commands,
        mut q_slowed: Query<(Entity, &mut Slowed)>,
        time: Res<Time>,
        time_scale: Res<TimeScale>,
    ) {
        let delta = time.delta().mul_f32(**time_scale);
        for (entity, mut slowed) in &mut q_slowed {
            slowed.duration.tick(delta);
            if slowed.duration.finished() {
                cmd.entity(entity).remove::<Slowed>();
            }
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::insert_effect.run_in_state(GameState::InGame))
            .add_system(Self::tick_on_fire.run_in_state(GameState::InGame))
            .add_system(Self::tick_shocked.run_in_state(GameState::InGame))
            .add_system(Self::tick_delayed_explosion.run_in_state(GameState::InGame))
            .add_system(Self::tick_blinded.run_in_state(GameState::InGame))
            .add_system(Self::tick_slowed.run_in_state(GameState::InGame));
    }
}

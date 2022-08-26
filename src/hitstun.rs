use bevy::prelude::*;

use iyes_loopless::prelude::*;

use crate::{utils::TimeScale, GameState};

#[derive(Component, Deref, DerefMut, Debug)]
pub struct HitstunTimer(pub Timer);

pub struct Plugin;

impl Plugin {
    fn tick(mut q_hitstun: Query<&mut HitstunTimer>, time: Res<Time>, time_scale: Res<TimeScale>) {
        for mut timer in &mut q_hitstun {
            timer.tick(time.delta().mul_f32(**time_scale));
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::tick.run_in_state(GameState::InGame));
    }
}

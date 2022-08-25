mod consts;
mod enemy;
mod health;
mod hitbox;
mod homing;
mod level;
mod player;
mod potion;
mod status;
mod utils;

use bevy::{prelude::*, render::texture::ImageSettings};

use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Component)]
pub struct Enemy;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum GameState {
    // MainMenu,
    InGame,
}

#[derive(Component, Clone, Copy, Debug)]
pub enum Element {
    Fire,
    Water,
    Wind,
    Lightning,
    Earth,
}

fn main() {
    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_loopless_state(GameState::InGame)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(utils::Plugin)
        .add_plugin(level::Plugin)
        .add_plugin(player::Plugin)
        .add_plugin(potion::Plugin)
        .add_plugin(enemy::Plugin)
        .add_plugin(hitbox::Plugin)
        .add_plugin(health::Plugin)
        .add_plugin(status::Plugin)
        .add_plugin(homing::Plugin)
        .add_startup_system(init)
        .run();
}

fn init(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.25;
    cmd.spawn_bundle(camera).insert(MainCamera);
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod consts;
mod enemy;
mod essence;
mod game_ui;
mod health;
mod hitbox;
mod hitstun;
mod homing;
mod level;
mod main_menu;
mod player;
mod potion;
mod status;
mod utils;

#[cfg(target_family = "wasm")]
mod preload;

use bevy::{prelude::*, render::texture::ImageSettings};

use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Component)]
pub struct Enemy;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum GameState {
    MainMenu,
    InGame,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum PauseState {
    Paused,
    Unpaused,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Element {
    Fire,
    Water,
    Wind,
    Lightning,
    Earth,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..default()
    })
    .insert_resource(WindowDescriptor {
        width: 1280.0,
        height: 720.0,
        title: "Somble".to_string(),
        canvas: Some("#bevy".to_owned()),
        ..Default::default()
    })
    .insert_resource(ImageSettings::default_nearest())
    .insert_resource(ClearColor(Color::rgb_u8(14, 14, 14)))
    .add_loopless_state(GameState::MainMenu)
    .add_loopless_state(PauseState::Unpaused)
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
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
    .add_plugin(essence::Plugin)
    .add_plugin(main_menu::Plugin)
    .add_plugin(hitstun::Plugin)
    .add_plugin(game_ui::Plugin)
    .add_startup_system(init);

    #[cfg(target_family = "wasm")]
    app.add_plugin(preload::Plugin);

    app.run();
}

fn init(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.25;
    cmd.spawn_bundle(camera).insert(MainCamera);
}

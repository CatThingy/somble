use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod player;

fn main() {
    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(player::Plugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

mod player;
mod utils;

use bevy::{prelude::*, render::texture::ImageSettings, sprite::Anchor};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use utils::MousePosition;

static PLAYER_COLLISION_GROUP: u32 = 1 << 0;
static ENEMY_COLLISION_GROUP: u32 = 1 << 1;
static WALL_COLLISION_GROUP: u32 = 1 << 2;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Enemy;

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
        .add_plugin(utils::Plugin)
        .add_plugin(player::Plugin)
        .add_startup_system(init)
        .add_system(debug_spawn)
        .run();
}

fn init(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}

fn debug_spawn(
    mut cmd: Commands,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mouse: Res<MousePosition>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let texture: Handle<Image> = asset_server.load("elemental.png");
        cmd.spawn_bundle((
            Enemy,
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::ball(16.0),
            CollisionGroups {
                memberships: ENEMY_COLLISION_GROUP,
                filters: PLAYER_COLLISION_GROUP | ENEMY_COLLISION_GROUP | WALL_COLLISION_GROUP,
            },
            LockedAxes::ROTATION_LOCKED,
            Damping { linear_damping: 60.0, angular_damping: 0.0 }
        ))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::from_array([32.0, 64.0])),
                anchor: Anchor::Custom(Vec2::from_array([0.0, -0.25])),
                ..default()
            },
            texture,
            transform: Transform {
                translation: **mouse,
                ..default()
            },
            ..default()
        });
    }
}

mod consts;
mod player;
mod utils;
mod level;

use bevy::{prelude::*, render::texture::ImageSettings, sprite::Anchor};

use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use consts::*;
use utils::{MousePosition, Spritesheets};

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Component)]
pub struct Enemy;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum GameState {
    Loading,
    // MainMenu,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_loopless_state(GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(utils::Plugin)
        .add_plugin(level::Plugin)
        .add_plugin(player::Plugin)
        .add_startup_system(init)
        .add_system(debug_spawn)
        .run();
}

fn init(mut cmd: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    cmd.spawn_bundle(camera)
        .insert(MainCamera);
}

fn debug_spawn(
    mut cmd: Commands,
    keys: Res<Input<KeyCode>>,
    mouse: Res<MousePosition>,
    sheets: Res<Spritesheets>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let texture_atlas = sheets.get("Elemental").unwrap().clone_weak();
        cmd.spawn_bundle((
            Enemy,
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::ball(8.0),
            CollisionGroups {
                memberships: ENEMY_COLLISION_GROUP,
                filters: PLAYER_COLLISION_GROUP
                    | ENEMY_COLLISION_GROUP
                    | WALL_COLLISION_GROUP
                    | PLAYER_ATTACK_COLLISION_GROUP,
            },
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: 60.0,
                angular_damping: 0.0,
            },
        ))
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::from_array([16.0, 32.0])),
                anchor: Anchor::Custom(Vec2::from_array([0.0, -0.25])),
                index: 0,
                ..default()
            },
            texture_atlas,
            transform: Transform {
                translation: **mouse,
                ..default()
            },
            ..default()
        });
    }
}

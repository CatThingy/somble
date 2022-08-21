use bevy::prelude::*;

use bevy::sprite::Anchor;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::consts::*;
use crate::Enemy;
use crate::GameState;

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, assets: Res<AssetServer>) {
        cmd.spawn_bundle(LdtkWorldBundle {
            ldtk_handle: assets.load("test.ldtk"),
            ..default()
        });
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_entity::<ElementalBundle>("Elemental");
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    pub collider: Collider,
    pub groups: CollisionGroups,
    pub locked: LockedAxes,
}

impl Default for WallBundle {
    fn default() -> Self {
        WallBundle {
            // body: RigidBody::Fixed,
            collider: Collider::cuboid(8.0, 8.0),
            groups: CollisionGroups {
                memberships: WALL_COLLISION_GROUP,
                filters: PLAYER_COLLISION_GROUP
                    | ENEMY_COLLISION_GROUP
                    | PLAYER_ATTACK_COLLISION_GROUP
                    | ENEMY_ATTACK_COLLISION_GROUP,
            },
            locked: LockedAxes::all(),
        }
    }
}

impl LdtkIntCell for WallBundle {
    fn bundle_int_cell(_: IntGridCell, _: &LayerInstance) -> Self {
        default()
    }
}

#[derive(Bundle)]
pub struct ElementalBundle {
    enemy: Enemy,
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    groups: CollisionGroups,
    locked: LockedAxes,
    damping: Damping,
    #[bundle]
    spritesheet: SpriteSheetBundle,
}

impl LdtkEntity for ElementalBundle {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let elemental_texture = asset_server.load("elemental.png");
        let elemental_atlas =
            TextureAtlas::from_grid(elemental_texture, Vec2::new(16.0, 32.0), 3, 1);
        let texture_atlas = texture_atlases.add(elemental_atlas);
        ElementalBundle {
            enemy: Enemy,
            body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            collider: Collider::ball(8.0),
            groups: CollisionGroups {
                memberships: ENEMY_COLLISION_GROUP,
                filters: PLAYER_COLLISION_GROUP
                    | ENEMY_COLLISION_GROUP
                    | WALL_COLLISION_GROUP
                    | PLAYER_ATTACK_COLLISION_GROUP,
            },
            locked: LockedAxes::ROTATION_LOCKED,
            damping: Damping {
                linear_damping: 20.0,
                angular_damping: 0.0,
            },
            spritesheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    anchor: Anchor::Custom(Vec2::from_array([0.0, -0.25])),
                    index: 0,
                    color: Color::LIME_GREEN,
                    ..default()
                },
                texture_atlas,
                ..default()
            },
        }
    }
}

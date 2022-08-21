use bevy::prelude::*;

use bevy::utils::HashSet;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::consts::*;
use crate::GameState;

#[derive(Bundle)]
pub struct WallBundle {
    pub collider: Collider,
    pub groups: CollisionGroups,
    pub locked: LockedAxes,
    pub friction: Friction,
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
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        }
    }
}

impl LdtkIntCell for WallBundle {
    fn bundle_int_cell(_: IntGridCell, _: &LayerInstance) -> Self {
        default()
    }
}

#[derive(Component, Default)]
pub struct Walkable;

#[derive(Bundle, LdtkIntCell)]
pub struct WalkableBundle {
    walkable: Walkable,
}

#[derive(Default, Deref, DerefMut)]
pub struct WalkableTiles(HashSet<IVec2>);

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, assets: Res<AssetServer>) {
        cmd.spawn_bundle(LdtkWorldBundle {
            ldtk_handle: assets.load("test.ldtk"),
            ..default()
        });
    }

    fn register_walkable(
        q_tiles: Query<&Transform, Added<Walkable>>,
        mut walkables: ResMut<WalkableTiles>,
    ) {
        for tile in &q_tiles {
            let tile_pos =
                translation_to_grid_coords(tile.translation.truncate(), IVec2::splat(GRID_SIZE));
            walkables.insert(tile_pos.into());
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init)
            .add_system(Self::register_walkable.run_in_state(GameState::InGame))
            .insert_resource(LevelSelection::Index(0))
            .init_resource::<WalkableTiles>()
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WalkableBundle>(2);
    }
}

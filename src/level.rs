use bevy::prelude::*;

use bevy::utils::HashSet;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::essence::EssenceCounts;
use crate::player::Player;
use crate::GameState;
use crate::{consts::*, PauseState};

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
#[derive(Component)]
pub struct NotFromLevel;

#[derive(Component, Default)]
pub struct Walkable;

#[derive(Bundle, LdtkIntCell)]
pub struct WalkableBundle {
    walkable: Walkable,
}

#[derive(Bundle, LdtkEntity)]
pub struct StairEntity {
    #[bundle]
    #[sprite_sheet_bundle("minimal.png", 16.0, 16.0, 4, 5, 0.0, 0.0, 8)]
    sprite_sheet: SpriteSheetBundle,
    #[bundle]
    stairs: StairBundle,
}

#[derive(Component, Default)]
pub struct Stairs;

#[derive(Bundle)]
struct StairBundle {
    collider: Collider,
    sensor: Sensor,
    stairs: Stairs,
    groups: CollisionGroups,
    events: ActiveEvents,
}

impl Default for StairBundle {
    fn default() -> Self {
        StairBundle {
            collider: Collider::cuboid(8.0, 8.0),
            sensor: Sensor,
            stairs: Stairs,
            groups: CollisionGroups {
                memberships: WALL_COLLISION_GROUP,
                filters: PLAYER_COLLISION_GROUP,
            },
            events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct WalkableTiles(HashSet<IVec2>);

pub struct CurrentLevel(pub usize);

pub struct NextLevel;
pub struct RestartLevel;
pub struct Reset;

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

    fn update_stairs(
        mut event_reader: EventReader<CollisionEvent>,
        mut event_writer: EventWriter<NextLevel>,
        q_player: Query<(), With<Player>>,
        q_stairs: Query<(), With<Stairs>>,
    ) {
        for event in event_reader.iter() {
            match event {
                CollisionEvent::Started(e1, e2, _) => {
                    if let Ok(_) = q_stairs.get(*e1) {
                        if let Ok(_) = q_player.get(*e2) {
                            event_writer.send(NextLevel);
                            return;
                        } else {
                            continue;
                        }
                    } else if let Ok(_) = q_stairs.get(*e2) {
                        if let Ok(_) = q_player.get(*e1) {
                            event_writer.send(NextLevel);
                            return;
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
                _ => (),
            }
        }
    }

    fn next_level(
        mut cmd: Commands,
        event_reader: EventReader<NextLevel>,
        mut current_level: ResMut<CurrentLevel>,
        q_despawn: Query<Entity, With<NotFromLevel>>,
    ) {
        if !event_reader.is_empty() {
            current_level.0 += 1;
            cmd.insert_resource(LevelSelection::Index(current_level.0));
            for entity in &q_despawn {
                cmd.entity(entity).despawn_recursive();
            }
        }
    }

    fn restart_level(
        mut cmd: Commands,
        event_reader: EventReader<RestartLevel>,
        q_level: Query<Entity, With<Handle<LdtkLevel>>>,
        q_despawn: Query<Entity, With<NotFromLevel>>,
        mut essences: ResMut<EssenceCounts>,
    ) {
        if !event_reader.is_empty() {
            cmd.entity(q_level.single()).insert(Respawn);
            for entity in &q_despawn {
                cmd.entity(entity).despawn_recursive();
            }
            *essences = EssenceCounts::default();
        }
    }

    fn reset(
        mut cmd: Commands,
        event_reader: EventReader<Reset>,
        mut current_level: ResMut<CurrentLevel>,
        mut essences: ResMut<EssenceCounts>,
    ) {
        if !event_reader.is_empty() {
            current_level.0 = 0;
            cmd.insert_resource(LevelSelection::Index(current_level.0));
            *essences = EssenceCounts::default();
        }
    }

    fn cleanup(
        mut cmd: Commands,
        q_world: Query<Entity, With<Handle<LdtkAsset>>>,
        q_despawn: Query<Entity, With<NotFromLevel>>,
    ) {
        cmd.entity(q_world.single()).despawn_recursive();
        for entity in &q_despawn {
            cmd.entity(entity).despawn_recursive();
        }
        cmd.insert_resource(NextState(PauseState::Unpaused));
        cmd.insert_resource(LevelSelection::Index(0));
        cmd.insert_resource(CurrentLevel(0));
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init)
            .add_exit_system(GameState::InGame, Self::cleanup)
            .add_system(Self::register_walkable.run_in_state(GameState::InGame))
            .add_system(Self::update_stairs.run_in_state(GameState::InGame))
            .add_system(Self::next_level.run_in_state(GameState::InGame))
            .add_system(Self::restart_level.run_in_state(GameState::InGame))
            .add_system(Self::reset.run_in_state(GameState::InGame))
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(CurrentLevel(0))
            .init_resource::<WalkableTiles>()
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WalkableBundle>(2)
            .register_ldtk_entity::<StairEntity>("Stairs")
            .add_event::<NextLevel>()
            .add_event::<RestartLevel>()
            .add_event::<Reset>();
    }
}

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

use crate::consts::*;
use crate::health::Dead;
use crate::hitbox::Hitbox;
use crate::utils::{DespawnTimer, UniformAnim};
use crate::{
    health::HealthChange,
    hitbox::{DamageOnce, Falloff},
    utils::TimeScale,
    GameState,
};

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
                    amount: -10.0,
                });
            }

            if on_fire.duration.finished() {
                cmd.entity(entity).remove::<OnFire>();
            }
        }
    }

    fn tick_shocked(
        mut cmd: Commands,
        mut q_shocked: Query<(Entity, &GlobalTransform, &mut Shocked)>,
        time: Res<Time>,
        time_scale: Res<TimeScale>,
    ) {
        let delta = time.delta().mul_f32(**time_scale);
        for (entity, transform, mut shocked) in &mut q_shocked {
            shocked.duration.tick(delta);
            shocked.tick.tick(delta);

            if shocked.tick.finished() {
                cmd.spawn_bundle(SpatialBundle::from_transform(transform.compute_transform()))
                    .insert_bundle((
                        Collider::ball(24.0),
                        CollisionGroups {
                            memberships: PLAYER_ATTACK_COLLISION_GROUP,
                            filters: ENEMY_COLLISION_GROUP,
                        },
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor,
                        Hitbox,
                        DamageOnce::new(10.0, Falloff::none()),
                        DespawnTimer(Timer::from_seconds(0.05, false)),
                    ));
            }

            if shocked.duration.finished() {
                cmd.entity(entity).remove::<Shocked>();
            }
        }
    }

    fn tick_delayed_explosion(
        mut cmd: Commands,
        mut q_delayed_explosion: Query<(
            Entity,
            &GlobalTransform,
            &mut DelayedExplosion,
            Option<&Dead>,
        )>,
        time: Res<Time>,
        time_scale: Res<TimeScale>,
        assets: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let delta = time.delta().mul_f32(**time_scale);
        for (entity, transform, mut delayed_explosion, dead) in &mut q_delayed_explosion {
            delayed_explosion.duration.tick(delta);
            if delayed_explosion.duration.finished() || dead.is_some() {
                cmd.entity(entity)
                    .remove::<DelayedExplosion>()
                    .despawn_descendants();
                cmd.spawn_bundle(SpatialBundle::from_transform(transform.compute_transform()))
                    .insert_bundle((
                        TextureAtlasSprite::default(),
                        {
                            let tex = assets.load("delayed_explosion.png");
                            atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(48.0), 3, 1))
                        },
                        DespawnTimer(Timer::from_seconds(0.3, false)),
                        UniformAnim(Timer::from_seconds(0.1, true)),
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(SpatialBundle::from_transform(
                                transform.compute_transform(),
                            ))
                            .insert_bundle((
                                Collider::ball(24.0),
                                CollisionGroups {
                                    memberships: PLAYER_ATTACK_COLLISION_GROUP,
                                    filters: ENEMY_COLLISION_GROUP,
                                },
                                ActiveEvents::COLLISION_EVENTS,
                                Sensor,
                                Hitbox,
                                DamageOnce::new(60.0, Falloff::none()),
                                DespawnTimer(Timer::from_seconds(0.05, false)),
                            ));
                    });
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

    fn attach_visuals(
        mut cmd: Commands,
        q_on_fire: Query<Entity, Added<OnFire>>,
        q_shocked: Query<Entity, Added<Shocked>>,
        q_delayed_explosion: Query<Entity, Added<DelayedExplosion>>,
        q_blinded: Query<Entity, Added<Blinded>>,
        q_slowed: Query<Entity, Added<Slowed>>,
        assets: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        for new_on_fire in &q_on_fire {
            let visual = cmd
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: {
                        let tex = assets.load("on_fire.png");
                        atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(16.0), 5, 1))
                    },
                    ..default()
                })
                .insert(UniformAnim(Timer::from_seconds(0.1, true)))
                .id();
            cmd.entity(new_on_fire).add_child(visual);
        }

        for new_shocked in &q_shocked {
            let visual = cmd
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: {
                        let tex = assets.load("shocked.png");
                        atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(32.0), 5, 1))
                    },
                    ..default()
                })
                .insert(UniformAnim(Timer::from_seconds(0.1, true)))
                .id();
            cmd.entity(new_shocked).add_child(visual);
        }

        for new_delayed_explosion in &q_delayed_explosion {
            let visual = cmd
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: {
                        let tex = assets.load("fire_lightning.png");
                        atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(16.0), 2, 1))
                    },
                    ..default()
                })
                .insert(UniformAnim(Timer::from_seconds(0.1, true)))
                .id();
            cmd.entity(new_delayed_explosion).add_child(visual);
        }

        for new_blinded in &q_blinded {
            let visual = cmd
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: {
                        let tex = assets.load("blinded.png");
                        atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(16.0), 5, 1))
                    },
                    ..default()
                })
                .insert(UniformAnim(Timer::from_seconds(0.1, true)))
                .id();
            cmd.entity(new_blinded).add_child(visual);
        }

        for new_slowed in &q_slowed {
            let visual = cmd
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: {
                        let tex = assets.load("slowed.png");
                        atlases.add(TextureAtlas::from_grid(tex, Vec2::splat(16.0), 4, 1))
                    },
                    ..default()
                })
                .insert(UniformAnim(Timer::from_seconds(0.1, true)))
                .id();
            cmd.entity(new_slowed).add_child(visual);
        }
    }

    fn remove_visuals<T: Component>(
        mut cmd: Commands,
        q_removed: RemovedComponents<T>,
        q_all: Query<()>,
    ) {
        for entity in q_removed.iter() {
            if q_all.get(entity).is_ok() {
                cmd.entity(entity).despawn_descendants();
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
            .add_system(Self::tick_slowed.run_in_state(GameState::InGame))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                Self::remove_visuals::<OnFire>.run_in_state(GameState::InGame),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                Self::remove_visuals::<Shocked>.run_in_state(GameState::InGame),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                Self::remove_visuals::<Blinded>.run_in_state(GameState::InGame),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                Self::remove_visuals::<Slowed>.run_in_state(GameState::InGame),
            )
            .add_system(Self::attach_visuals.run_in_state(GameState::InGame));
    }
}

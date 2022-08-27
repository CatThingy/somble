use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{health::Health, player::Player, GameState};

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct HealthBar;

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands) {
        cmd.spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Root)
        .with_children(|root| {
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(33.0),
                        height: Val::Px(20.0),
                    },
                    align_self: AlignSelf::FlexStart,
                    margin: UiRect {
                        left: Val::Px(20.0),
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
                color: Color::RED.into(),
                ..default()
            })
            .with_children(|bar| {
                bar.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                        },
                        align_self: AlignSelf::FlexStart,
                        ..default()
                    },
                    color: Color::GREEN.into(),
                    ..default()
                })
                .insert(HealthBar);
            });
        });
    }

    fn update_healthbar(
        q_player: Query<&Health, With<Player>>,
        mut q_bar: Query<&mut Style, With<HealthBar>>,
    ) {
        let player_health = match q_player.get_single() {
            Ok(v) => v,
            Err(_) => return,
        };
        let mut bar = match q_bar.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };

        bar.size.width = Val::Percent(player_health.percentage() * 100.0);
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init)
            .add_system(Self::update_healthbar.run_in_state(GameState::InGame));
    }
}

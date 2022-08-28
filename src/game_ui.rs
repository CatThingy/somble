use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    health::{Dead, Health},
    level::{Reset, RestartLevel},
    player::Player,
    potion::PotionBrewState,
    utils::TimeScale,
    GameState, PauseState,
};

#[derive(Component)]
pub struct Root;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct GameMenu;

#[derive(Component)]
pub struct PauseText;

#[derive(Component)]
pub struct DeathText;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct MainMenuButton;

pub struct Plugin;

impl Plugin {
    fn init(mut cmd: Commands, assets: Res<AssetServer>) {
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

            root.spawn_bundle(NodeBundle {
                style: Style {
                    display: Display::None,
                    flex_direction: FlexDirection::ColumnReverse,
                    justify_content: JustifyContent::SpaceEvenly,
                    size: Size {
                        width: Val::Percent(75.0),
                        height: Val::Percent(75.0),
                    },
                    position: UiRect::all(Val::Percent(12.5)),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                color: Color::DARK_GRAY.into(),
                ..default()
            })
            .insert(GameMenu)
            .with_children(|panel| {
                panel
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::SpaceEvenly,
                            size: Size {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                            },
                            ..default()
                        },
                        color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|panel| {
                        panel
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    display: Display::None,
                                    justify_content: JustifyContent::SpaceEvenly,
                                    size: Size {
                                        width: Val::Px(512.0),
                                        height: Val::Px(128.0),
                                    },
                                    ..default()
                                },
                                image: UiImage(assets.load("pause_text.png")),
                                ..default()
                            })
                            .insert(PauseText);
                        panel
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    display: Display::None,
                                    justify_content: JustifyContent::SpaceEvenly,
                                    size: Size {
                                        width: Val::Px(512.0),
                                        height: Val::Px(128.0),
                                    },
                                    ..default()
                                },
                                image: UiImage(assets.load("death_text.png")),
                                ..default()
                            })
                            .insert(DeathText);
                    });
                panel
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::SpaceEvenly,
                            size: Size {
                                width: Val::Percent(100.0),
                                height: Val::Auto,
                            },
                            ..default()
                        },
                        color: Color::NONE.into(),
                        ..default()
                    })
                    .with_children(|panel| {
                        panel
                            .spawn_bundle(ButtonBundle {
                                image: UiImage(assets.load("restart_button.png")),
                                style: Style {
                                    size: Size {
                                        width: Val::Px(256.0),
                                        height: Val::Px(64.0),
                                    },
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(RestartButton);
                        panel
                            .spawn_bundle(ButtonBundle {
                                image: UiImage(assets.load("menu_button.png")),
                                style: Style {
                                    size: Size {
                                        width: Val::Px(256.0),
                                        height: Val::Px(64.0),
                                    },
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(MainMenuButton);
                    });
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
    fn handle_restart_click(
        mut cmd: Commands,
        mut event_writer: EventWriter<RestartLevel>,
        q_button: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
        mouse: Res<Input<MouseButton>>,
    ) {
        if mouse.just_released(MouseButton::Left) {
            for button in &q_button {
                if button == &Interaction::Hovered {
                    event_writer.send(RestartLevel);
                    cmd.insert_resource(NextState(PauseState::Unpaused));
                }
            }
        }
    }
    fn handle_menu_click(
        mut cmd: Commands,
        mut event_writer: EventWriter<Reset>,
        q_button: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
        mouse: Res<Input<MouseButton>>,
    ) {
        if mouse.just_released(MouseButton::Left) {
            for button in &q_button {
                if button == &Interaction::Hovered {
                    event_writer.send(Reset);
                    cmd.insert_resource(NextState(GameState::MainMenu))
                }
            }
        }
    }

    fn pause(
        mut q_panel: Query<&mut Style, (With<GameMenu>, Without<PauseText>, Without<DeathText>)>,
        mut time_scale: ResMut<TimeScale>,
    ) {
        time_scale.0 = f32::EPSILON;

        let mut panel = match q_panel.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };

        panel.display = Display::Flex;
    }

    fn unpause(mut q_panel: Query<&mut Style, With<GameMenu>>, mut time_scale: ResMut<TimeScale>) {
        time_scale.0 = 1.0;

        let mut panel = match q_panel.get_single_mut() {
            Ok(v) => v,
            Err(_) => return,
        };
        panel.display = Display::None;
    }

    fn handle_pause(
        mut cmd: Commands,
        mut q_pause_text: Query<&mut Style, (With<PauseText>, Without<DeathText>)>,
        mut q_death_text: Query<&mut Style, (Without<PauseText>, With<DeathText>)>,
        q_dead_player: Query<(), (With<Player>, With<Dead>, Without<Style>)>,
        paused: Res<CurrentState<PauseState>>,
        keyboard: Res<Input<KeyCode>>,
        mut brew_state: ResMut<PotionBrewState>,
    ) {
        if keyboard.just_pressed(KeyCode::Escape) && q_dead_player.is_empty() {
            match paused.0 {
                PauseState::Paused => cmd.insert_resource(NextState(PauseState::Unpaused)),
                PauseState::Unpaused => cmd.insert_resource(NextState(PauseState::Paused)),
            }
            let mut pause_text = match q_pause_text.get_single_mut() {
                Ok(v) => v,
                Err(_) => return,
            };
            let mut death_text = match q_death_text.get_single_mut() {
                Ok(v) => v,
                Err(_) => return,
            };
            pause_text.display = Display::Flex;
            death_text.display = Display::None;

            *brew_state = PotionBrewState::Inactive;
        }
    }

    fn cleanup(mut cmd: Commands, q_root: Query<Entity, With<Root>>) {
        cmd.entity(q_root.single()).despawn_recursive();
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, Self::init)
            .add_exit_system(GameState::InGame, Self::cleanup)
            .add_enter_system(PauseState::Paused, Self::pause)
            .add_enter_system(PauseState::Unpaused, Self::unpause)
            .add_system(Self::update_healthbar.run_in_state(GameState::InGame))
            .add_system(Self::handle_restart_click.run_in_state(GameState::InGame))
            .add_system(Self::handle_menu_click.run_in_state(GameState::InGame))
            .add_system(Self::handle_pause.run_in_state(GameState::InGame));
    }
}

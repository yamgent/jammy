// SPDX-License-Identifier: MIT OR Apache-2.0

//! jammy Splash Screen crate
//!
//! This bevy crate allows you to create an initial
//! loading splash screen quickly. It uses no other assets,
//! so it itself does not require any pre-loading,
//! and does not waste asset disk space just for
//! pure loading purposes.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use iyes_progress::{ProgressCounter, TrackedProgressSet};

const SPLASH_SCREEN_BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

const LOADING_BAR_TOTAL_SIZE: Vec2 = Vec2::new(512.0, 24.0);

const LOADING_BAR_BORDER_THICKNESS: Vec2 = Vec2::new(4.0, 4.0);
const LOADING_BAR_BORDER_COLOR: Color = Color::rgb(0.6, 0.6, 0.56);

const LOADING_BAR_BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const LOADING_BAR_COLOR: Color = Color::rgb(0.5, 0.8, 0.2);

const LOADING_BAR_INNER_SIZE: Vec2 = Vec2::new(
    LOADING_BAR_TOTAL_SIZE.x - (LOADING_BAR_BORDER_THICKNESS.x * 2.0),
    LOADING_BAR_TOTAL_SIZE.y - (LOADING_BAR_BORDER_THICKNESS.y * 2.0),
);

const LOADING_TEXT_MARGIN_TOP: f32 = 20.0;
const LOADING_TEXT_SIZE: Vec2 = Vec2::new(275.0, 40.0);
const LOADING_TEXT_COLOR: Color = Color::rgb(0.8, 0.8, 0.78);

const LOADING_SYMBOL_MARGIN_BOTTOM: f32 = 32.0;
const LOADING_SYMBOL_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const LOADING_SYMBOL_COLOR: Color = LOADING_TEXT_COLOR;

pub struct JammySplashScreenPlugin<S: States> {
    pub state: S,
    // TODO: Is this necessary?
    pub next_state: Option<S>,
}

impl<S: States> JammySplashScreenPlugin<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            next_state: None,
        }
    }

    pub fn continue_to(mut self, next_state: S) -> Self {
        self.next_state = Some(next_state);
        self
    }
}

impl<S: States> Plugin for JammySplashScreenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_system(setup_splash_screen.in_schedule(OnEnter(self.state.clone())))
            .add_system(destroy_splash_screen.in_schedule(OnExit(self.state.clone())))
            .add_system(
                update_loading_bar
                    .in_set(OnUpdate(self.state.clone()))
                    .after(TrackedProgressSet),
            )
            .add_system(update_loading_symbol);
    }
}

#[derive(Component)]
struct SplashScreenUi;

#[derive(Component)]
struct LoadingBarUi;

#[derive(Component)]
struct LoadingSymbolUi;

fn setup_splash_screen(mut commands: Commands) {
    fn setup_camera(commands: &mut Commands) {
        commands.spawn((
            SplashScreenUi,
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(SPLASH_SCREEN_BACKGROUND_COLOR),
                },
                ..Default::default()
            },
        ));
    }

    fn setup_loading_bar(commands: &mut Commands) {
        // outer box
        commands.spawn((
            SplashScreenUi,
            SpriteBundle {
                sprite: Sprite {
                    color: LOADING_BAR_BORDER_COLOR,
                    custom_size: Some(LOADING_BAR_TOTAL_SIZE),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));

        // inner box
        commands.spawn((
            SplashScreenUi,
            SpriteBundle {
                sprite: Sprite {
                    color: LOADING_BAR_BACKGROUND_COLOR,
                    custom_size: Some(LOADING_BAR_INNER_SIZE),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));

        // actual bar
        commands.spawn((
            SplashScreenUi,
            LoadingBarUi,
            SpriteBundle {
                sprite: Sprite {
                    color: LOADING_BAR_COLOR,
                    custom_size: Some(Vec2::new(0.0, LOADING_BAR_INNER_SIZE.y)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }

    fn setup_loading_text(commands: &mut Commands) {
        commands
            .spawn(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    -LOADING_BAR_TOTAL_SIZE.y * 0.5
                        - LOADING_TEXT_MARGIN_TOP
                        - LOADING_TEXT_SIZE.y * 0.5,
                    0.0,
                )),
                ..Default::default()
            })
            .with_children(|parent| {
                const CELL_SIZE: Vec2 =
                    Vec2::new(LOADING_TEXT_SIZE.x / 26.0, LOADING_TEXT_SIZE.y / 5.0);
                const BOTTOM_LEFT: Vec3 = Vec3::new(
                    (-LOADING_TEXT_SIZE.x + CELL_SIZE.x) * 0.5,
                    (-LOADING_TEXT_SIZE.y + CELL_SIZE.y) * 0.5,
                    0.0,
                );

                [
                    // L
                    ((0.0, 2.0), (1.0, 5.0)),
                    ((1.5, 0.0), (2.0, 1.0)),
                    // O
                    ((4.0, 2.0), (1.0, 5.0)),
                    ((6.0, 2.0), (1.0, 5.0)),
                    ((5.0, 0.0), (1.0, 1.0)),
                    ((5.0, 4.0), (1.0, 1.0)),
                    // A
                    ((8.0, 2.0), (1.0, 5.0)),
                    ((10.0, 2.0), (1.0, 5.0)),
                    ((9.0, 2.0), (1.0, 1.0)),
                    ((9.0, 4.0), (1.0, 1.0)),
                    // D
                    ((12.5, 2.0), (1.0, 3.0)),
                    ((14.5, 2.0), (1.0, 3.0)),
                    ((13.0, 0.0), (3.0, 1.0)),
                    ((13.0, 4.0), (3.0, 1.0)),
                    // I
                    ((16.75, 2.0), (1.0, 5.0)),
                    // N
                    ((19.0, 2.0), (1.0, 5.0)),
                    ((21.0, 2.0), (1.0, 5.0)),
                    ((20.0, 4.0), (1.0, 1.0)),
                    // G
                    ((23.0, 2.0), (1.0, 5.0)),
                    ((25.0, 1.0), (1.0, 3.0)),
                    ((24.5, 0.0), (2.0, 1.0)),
                    ((24.5, 4.0), (2.0, 1.0)),
                ]
                .into_iter()
                .for_each(|(origin, size)| {
                    parent.spawn((
                        SplashScreenUi,
                        SpriteBundle {
                            sprite: Sprite {
                                color: LOADING_TEXT_COLOR,
                                custom_size: Some(Vec2::new(
                                    CELL_SIZE.x * size.0,
                                    CELL_SIZE.y * size.1,
                                )),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(
                                BOTTOM_LEFT
                                    + Vec3::new(
                                        CELL_SIZE.x * origin.0,
                                        CELL_SIZE.y * origin.1,
                                        0.0,
                                    ),
                            ),
                            ..Default::default()
                        },
                    ));
                });
            });
    }

    fn setup_loading_symbol(commands: &mut Commands) {
        commands
            .spawn(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    LOADING_BAR_TOTAL_SIZE.y * 0.5
                        + LOADING_SYMBOL_MARGIN_BOTTOM
                        + LOADING_SYMBOL_SIZE.y * 0.5,
                    0.0,
                )),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn((
                    SplashScreenUi,
                    LoadingSymbolUi,
                    SpriteBundle {
                        sprite: Sprite {
                            color: LOADING_SYMBOL_COLOR,
                            custom_size: Some(LOADING_SYMBOL_SIZE),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ));
            });
    }

    setup_camera(&mut commands);
    setup_loading_bar(&mut commands);
    setup_loading_text(&mut commands);
    setup_loading_symbol(&mut commands);
}

fn update_loading_bar(
    progress: Res<ProgressCounter>,
    mut loading_bar_uis: Query<(&mut Sprite, &mut Transform), With<LoadingBarUi>>,
) {
    let progress = progress.progress();
    let percentage = progress.done as f32 / progress.total as f32;

    loading_bar_uis
        .iter_mut()
        .for_each(|(mut bar, mut transform)| {
            let width = percentage * LOADING_BAR_INNER_SIZE.x;
            transform.translation.x = (-LOADING_BAR_INNER_SIZE.x + width) * 0.5;
            bar.custom_size = Some(Vec2::new(width, LOADING_BAR_INNER_SIZE.y));
        });
}

fn update_loading_symbol(
    time: Res<Time>,
    mut loading_symbol_uis: Query<&mut Transform, With<LoadingSymbolUi>>,
) {
    let time = time.elapsed_seconds() * 0.5;

    loading_symbol_uis.iter_mut().for_each(|mut transform| {
        *transform = Transform::from_rotation(Quat::from_rotation_z(-time)).with_scale(Vec3::new(
            0.2 + (time.sin() + 1.0) * 0.4,
            0.2 + (time.cos() + 1.0) * 0.4,
            0.0,
        ));
    });
}

fn destroy_splash_screen(mut commands: Commands, uis: Query<Entity, With<SplashScreenUi>>) {
    uis.for_each(|ui| {
        commands.entity(ui).despawn();
    });
}

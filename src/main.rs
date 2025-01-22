#![allow(dead_code, unused_imports)] //WARN:Remove when no longer necessary!
use bevy::{
    asset::*,
    audio::AudioPlugin,
    audio::*,
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowLevel, WindowTheme},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tetris".into(),
                    name: Some("bevy.app".into()),
                    //Placeholder resolution
                    resolution: (500., 300.).into(),
                    present_mode: PresentMode::AutoVsync,
                    resizable: false,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    window_level: WindowLevel::Normal,
                    //Temporary options until game menu functionality added.
                    titlebar_shown: true,
                    titlebar_show_title: true,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        minimize: false,
                        close: true,
                    },
                    visible: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            AudioPlugin {
                ..Default::default()
            },
            //TODO: set a frame limiter!
        ))
        .add_systems(Update, make_visible)
        .run();
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}
// let tetrominoes = [
//     ('I', "Cyan"),
//     ('O', "Yellow"),
//     ('T', "Purple"),
//     ('S', "Green"),
//     ('Z', "Red"),
//     ('J', "Blue"),
//     ('L', "Orange")
// ];
//TODO: Korobeiniki song mandatory, Katyusha and Kalinka optional
fn play_theme_song() {}
fn spawn_tetromino() {
    //The I and O spawn in the middle columns
    //The rest spawn in the left-middle columns
    //The tetrominoes spawn horizontally with J, L and T spawning flat-side first.
}
fn srs() {
    //https://harddrop.com/wiki/SRS
}
fn hard_drop() {}
fn soft_drop() {}
fn hold() {}
fn ghost() {}

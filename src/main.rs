use std::path::PathBuf;

use bevy::{input::mouse::MouseButtonInput, prelude::*, render::texture::ImageSettings};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d_assets::BevyRapier2dAssetsPlugin;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyRapier2dAssetsPlugin)
        .add_startup_system(setup)
        .add_system(adjust_scale)
        .run();
}

#[derive(Component)]
struct MainImage;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();

    commands.spawn_bundle(Camera2dBundle::default());

    // image
    let file = std::env::args()
        .skip(1)
        .next()
        .expect("provide path to PNG file");
    let mut file = PathBuf::from(file);
    if file.starts_with("assets") {
        file = file.strip_prefix("assets").unwrap().to_owned();
    }
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(file),
            ..Default::default()
        })
        .insert(MainImage);
}

fn adjust_scale(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<MainImage>>,
) {
    let scale_change = if keyboard_input.just_pressed(KeyCode::Key1) {
        Some(1.0)
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        Some(2.0)
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        Some(3.0)
    } else if keyboard_input.just_pressed(KeyCode::Key4) {
        Some(4.0)
    } else if keyboard_input.just_pressed(KeyCode::Key5) {
        Some(5.0)
    } else if keyboard_input.just_pressed(KeyCode::Key6) {
        Some(6.0)
    } else if keyboard_input.just_pressed(KeyCode::Key7) {
        Some(7.0)
    } else if keyboard_input.just_pressed(KeyCode::Key8) {
        Some(8.0)
    } else if keyboard_input.just_pressed(KeyCode::Key9) {
        Some(9.0)
    } else if keyboard_input.just_pressed(KeyCode::Key0) {
        Some(10.0)
    } else {
        None
    };

    if let Some(scale) = scale_change {
        for mut transform in &mut query {
            transform.scale = Vec3::splat(scale);
        }
    }
}

fn handle_mouse(mut mouse_button_input_events: EventReader<MouseButtonInput>) {
    for event in mouse_button_input_events.iter() {
        event
    }
}

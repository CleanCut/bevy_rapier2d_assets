use std::path::PathBuf;

use bevy::{input::mouse::MouseButtonInput, prelude::*, render::texture::ImageSettings};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d_assets::BevyRapier2dAssetsPlugin;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(MouseLocation(Vec2::ZERO))
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyRapier2dAssetsPlugin)
        .add_startup_system(setup)
        .add_system(adjust_scale)
        .add_system(handle_mouse)
        .run();
}

#[derive(Debug, Copy, Clone)]
struct MouseLocation(Vec2);

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

fn handle_mouse(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_location: ResMut<MouseLocation>,
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<MainImage>>,
) {
    // Get window dimensions
    // It's possible to not have window dimensions for the first frame or two
    let window_dimensions;
    if let Some(window) = windows.get_primary() {
        window_dimensions = Vec2::new(window.width(), window.height());
    } else {
        return;
    }

    // Update the mouse location
    if let Some(event) = cursor_moved_events.iter().last() {
        mouse_location.0 = event.position - window_dimensions * 0.5;
        println!("{:?}", mouse_location);
    }

    // Handle mouse click
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Ok(mut transform) = query.get_single_mut() {
            transform.translation = mouse_location.0.extend(0.0);
        }
    }
}

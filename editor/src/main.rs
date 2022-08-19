use std::path::PathBuf;

use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d_assets::{BevyRapier2dAssetsPlugin, SpritePhysicsAsset};
use lyon::{
    geom::euclid::Point2D,
    lyon_tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers},
    path::Path,
};

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(MouseLocation(Vec2::ZERO))
        .insert_resource(Scale(1.0))
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyRapier2dAssetsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(5.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(adjust_scale)
        .add_system(handle_mouse.label("handle_mouse"))
        .add_system(reconstruct_collider.after("handle_mouse"))
        .run();
}

#[derive(Debug, Copy, Clone)]
struct MouseLocation(Vec2);

#[derive(Debug, Copy, Clone)]
struct Scale(f32);

#[derive(Component)]
struct MainImage {
    dirty: bool,
}

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
            texture: asset_server.load(&*file),
            ..Default::default()
        })
        .insert(SpritePhysicsAsset {
            img_file: file,
            points: Vec::new(),
            sensor: false,
        })
        .insert(MainImage { dirty: true });
}

fn adjust_scale(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<MainImage>>,
    mut scale: ResMut<Scale>,
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

    if let Some(new_scale) = scale_change {
        scale.0 = new_scale;
        for mut transform in &mut query {
            transform.scale = Vec3::splat(new_scale);
        }
    }
}

fn handle_mouse(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut mouse_location: ResMut<MouseLocation>,
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &mut SpritePhysicsAsset, &mut MainImage)>,
    scale: Res<Scale>,
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
        mouse_location.0 = event.position - (window_dimensions * 0.5);
    }

    // Handle mouse click
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Ok((mut _transform, mut sprite_physics_asset, mut main_image)) =
            query.get_single_mut()
        {
            //transform.translation = mouse_location.0.extend(0.0);
            let scaled = mouse_location.0 / scale.0;
            let rounded_scaled = (scaled * 2.0).round() * 0.5; // round to nearest half
            sprite_physics_asset.points.push(rounded_scaled);
            main_image.dirty = true;
            // println!("{:?}", sprite_physics_asset.points);
        }
    }
}

fn reconstruct_collider(
    mut commands: Commands,
    mut query: Query<(Entity, &mut MainImage, &mut SpritePhysicsAsset)>,
) {
    for (entity, mut main_image, mut sprite_physics_asset) in &mut query {
        if !main_image.dirty {
            return;
        }
        main_image.dirty = false;
        if sprite_physics_asset.points.len() < 3 {
            return;
        }
        // let length = sprite_physics_asset.points.len();
        // let mut indices = Vec::new();
        // for i in 0..length {
        //     let a = i;
        //     let b = if a + 1 < length { a + 1 } else { 0 };
        //     indices.push([a as u32, b as u32]);
        // }
        // commands
        //     .entity(entity)
        //     .remove::<Collider>()
        //     .insert(Collider::convex_decomposition(
        //         &sprite_physics_asset.points,
        //         &indices,
        //     ));
        let mut points = sprite_physics_asset.points.clone();
        let mut builder = Path::builder();
        builder.begin(points.pop().unwrap().to_array().into());
        for point in points {
            builder.line_to(point.to_array().into());
        }
        builder.close();
        let path = builder.build();

        let mut geometry: VertexBuffers<Vec2, usize> = VertexBuffers::new();

        let mut tessellator = FillTessellator::new();

        {
            // Compute the tessellation.
            tessellator
                .tessellate_path(
                    &path,
                    &FillOptions::default(),
                    &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                        Vec2::from_array(vertex.position().to_array())
                    }),
                )
                .unwrap();
        }

        let mut colliders = Vec::new();
        let mut chunks = geometry.indices.chunks_exact(3);
        while let Some(inner_chunk) = chunks.next() {
            colliders.push((
                Vec2::ZERO,
                0.0,
                Collider::triangle(
                    geometry.vertices[inner_chunk[0]],
                    geometry.vertices[inner_chunk[1]],
                    geometry.vertices[inner_chunk[2]],
                ),
            ))
        }

        commands
            .entity(entity)
            .remove::<Collider>()
            .insert(Collider::compound(colliders));
    }
}

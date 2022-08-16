use std::path::PathBuf;

use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use super::{BevyRapier2dAssetsPlugin, SpritePhysicsAsset};
}

pub struct BevyRapier2dAssetsPlugin;

impl Plugin for BevyRapier2dAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<SpritePhysicsAsset>::new(&["sprite.ron"]));
    }
}

#[derive(Clone, Deserialize, Serialize, TypeUuid, Component)]
#[uuid = "8452acf1-39ab-1062-d972-10877b3ef456"]
pub struct SpritePhysicsAsset {
    pub img_file: PathBuf,
    pub points: Vec<Vec2>,
    pub sensor: bool,
}

// #[derive(Clone, Deserialize, Serialize)]
// pub enum SpriteColliderShape {
//     Ball { radius: f32 },
//     Polygon { points: Vec<[f32; 2]> },
// }

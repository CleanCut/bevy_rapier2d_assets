use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use super::BevyRapier2dAssetsPlugin;
}

pub struct BevyRapier2dAssetsPlugin;

impl Plugin for BevyRapier2dAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<SpriteWithPhysics>::new(&["sprite.ron"]));
    }
}

#[derive(Deserialize, Serialize, TypeUuid)]
#[uuid = "8452acf1-39ab-1062-d972-10877b3ef456"]
pub struct SpriteWithPhysics {
    file: String,
    collider: Vec<[f32; 2]>,
}

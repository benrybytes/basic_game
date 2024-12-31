use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub ketchup: Handle<Scene>
}

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
        .add_systems(Startup, load_assets);
    }
}

// load the assets into the scene to be referenced if needed to load
fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        ketchup:asset_server.load(GltfAssetLabel::Scene(0).from_asset("../assets/salmon_sushi.glb"))
    };
}

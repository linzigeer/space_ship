use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub spaceship: Handle<Scene>,
    pub missile: Handle<Scene>,
    pub asteroid: Handle<Scene>,
}

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        spaceship: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Spaceship.glb")),
        missile: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Missiles.glb")),
        asteroid: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Asteroid.glb")),
    }
}

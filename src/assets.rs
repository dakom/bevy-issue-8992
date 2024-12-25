use bevy::prelude::*;

use super::state::AppState;

#[derive(Resource)]
pub struct AssetHandles {
    pub helmet: Handle<Gltf>,
}

pub fn load_assets_sys(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AssetHandles{
        helmet: asset_server.load::<Gltf>("https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/DamagedHelmet/glTF-Binary/DamagedHelmet.glb"),
    });
}

pub fn check_loading_sys(
    mut events: EventReader<AssetEvent<Gltf>>,
    mut next_state: ResMut<NextState<AppState>>,
    asset_handles: Res<AssetHandles>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&asset_handles.helmet) {
            next_state.set(AppState::Playing);
        }
    }
}

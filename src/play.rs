use bevy::prelude::*;

use crate::{assets::AssetHandles, state::AppState};

#[derive(Resource)]
pub struct Entities {
    pub helmet: Entity,
}

pub fn play_start_sys(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_handles: Res<AssetHandles>,
    gltf_assets: Res<Assets<Gltf>>,
) {

    // Lights
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::IDENTITY.with_rotation(Quat::from_euler(EulerRot::XYZ, 1.0, 1.0, 1.0))
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::IDENTITY.with_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -1.0, -1.0))
    ));

    // Camera
    commands.spawn((
        Camera3d::default(), 
        Transform::IDENTITY
            .with_translation(Vec3::new(0.0, 0.0, 4.5)),
        DistanceFog {
            color: Color::srgb(0.25, 0.25, 0.25),
            falloff: FogFalloff::from_visibility(20.0),
            ..default()
        },
    )); 

    // Action
    let helmet_gltf = gltf_assets.get(&asset_handles.helmet).unwrap();

    let helmet_entity = commands.spawn((
        SceneRoot(helmet_gltf.scenes[0].clone()),
    )).id();

    commands.insert_resource(Entities {
        helmet: helmet_entity,
    });

    next_state.set(AppState::Playing);

    tracing::info!("playing!");

}

pub fn play_rotate_sys(
    time: Res<Time>,
    mut query: Query<&mut Transform>,
    entities: Res<Entities>,
) {
    if let Ok(mut transform) = query.get_mut(entities.helmet) {
        transform.rotation = Quat::from_rotation_y(time.elapsed_secs());
    }
}
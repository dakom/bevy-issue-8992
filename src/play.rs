use bevy::prelude::*;

use crate::{assets::AssetHandles, config::Config, loading::BevyReadySender};

#[derive(Resource)]
pub struct Entities {
    pub helmet: Entity,
}

pub fn play_start_sys(
    mut commands: Commands,
    mut ready_sender: ResMut<BevyReadySender>,
    asset_handles: Res<AssetHandles>,
    gltf_assets: Res<Assets<Gltf>>,
    config: Res<Config>,
) {
    // Lights
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::IDENTITY.with_rotation(Quat::from_euler(EulerRot::XYZ, 1.0, 1.0, 1.0)),
    ));

    if config.light {
        commands.spawn((
            DirectionalLight {
                illuminance: 10000.0,
                ..default()
            },
            Transform::IDENTITY.with_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -1.0, -1.0)),
        ));
    }

    // Camera
    let mut entity_commands = commands.spawn((
        Camera3d::default(),
        Transform::IDENTITY.with_translation(Vec3::new(0.0, 0.0, 4.5)),
    ));

    if config.fog {
        entity_commands.insert(DistanceFog {
            color: Color::srgb(1.0, 0.0, 0.0),
            falloff: FogFalloff::from_visibility(100.0),
            ..default()
        });
    }

    // Action
    let helmet_gltf = gltf_assets.get(&asset_handles.helmet).unwrap();

    let helmet_entity = commands
        .spawn((SceneRoot(helmet_gltf.scenes[0].clone()),))
        .id();

    commands.insert_resource(Entities {
        helmet: helmet_entity,
    });

    ready_sender.send();
}

pub fn play_rotate_sys(time: Res<Time>, mut query: Query<&mut Transform>, entities: Res<Entities>) {
    if let Ok(mut transform) = query.get_mut(entities.helmet) {
        transform.rotation = Quat::from_rotation_y(time.elapsed_secs());
    }
}

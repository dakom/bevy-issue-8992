mod assets;
mod logging;
mod play;
mod state;

use assets::{check_loading_sys, load_assets_sys};
use bevy::{log::LogPlugin, prelude::*};
use play::{play_rotate_sys, play_start_sys};
use state::AppState;

pub fn main() {
    logging::init_logger();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some(format!("#canvas")),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .disable::<LogPlugin>(),
        )
        .insert_state(AppState::Loading)
        .add_systems(OnEnter(AppState::Loading), load_assets_sys)
        .add_systems(OnEnter(AppState::Playing), play_start_sys)
        .add_systems(
            Update,
            (
                check_loading_sys.run_if(in_state(AppState::Loading)),
                play_rotate_sys.run_if(in_state(AppState::Playing)),
            ),
        )
        .run();
}

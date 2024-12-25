use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum AppState {
    Loading,
    Playing,
}
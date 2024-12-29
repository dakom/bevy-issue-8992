use bevy::prelude::*;

#[derive(Resource)]
pub struct Config {
    pub light: bool,
    pub fog: bool,
}

impl Config {
    pub fn new() -> Self {
        let mut _self = Self {
            light: true,
            fog: true,
        };

        let search_params =
            web_sys::Url::new(&web_sys::window().unwrap().location().href().unwrap())
                .unwrap()
                .search_params();

        if let Some(value) = search_params.get("light") {
            _self.light = value.to_lowercase() == "true";
        }

        if let Some(value) = search_params.get("fog") {
            _self.fog = value.to_lowercase() == "true";
        }

        _self
    }
}

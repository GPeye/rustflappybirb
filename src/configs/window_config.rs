use bevy::prelude::*;
use serde::Deserialize;

const WINDOW_CONFIG: &str = "assets/configs/window_config.ron";

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub resizable: bool,
}

impl WindowConfig {
    pub fn new() -> Self {
        let ron_option = &std::fs::read_to_string(WINDOW_CONFIG);
        let ron_str = match ron_option {
            Ok(v) => v,
            Err(_) => "",
        };

        let config = if ron_str == "" {
            Self::default()
        } else {
            match ron::from_str::<WindowConfig>(ron_str) {
                Ok(v) => v,
                Err(_) => {
                    error!("Failed to load window_config.ron");
                    Self::default()
                }
            } //.expect("Failed to load window_config.ron")
        };
        config
    }

    fn default() -> Self {
        Self {
            title: "My App".to_string(),
            width: 800.,
            height: 400.,
            resizable: true,
        }
    }
}

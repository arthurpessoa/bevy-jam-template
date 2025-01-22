use bevy::math::Vec2;
use bevy::prelude::Resource;
use bevy::render::settings::Backends;

#[derive(Resource, Clone, Copy)]
pub struct WindowSettings {
    pub window_resolution: Vec2,
    pub backend: Backends,
}

impl Default for WindowSettings {
    //TODO: Load from file?
    fn default() -> Self {
        Self {
            window_resolution: Vec2::new(1920.0, 1080.0),
            backend: Backends::VULKAN,
        }
    }
}
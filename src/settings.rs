use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::prelude::{resource_changed, IntoSystemConfigs, Res, Resource, Single, Window};
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::DefaultPlugins;

/// A plugin that makes it easy to configure the game settings
pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings = WindowSettings::default();
        app

            .add_plugins(DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(settings.backend),
                        ..default()
                    }),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy".to_string(),
                        resolution: (600., 420.).into(),
                        decorations: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }))
            .insert_resource(settings)
            .add_systems(Update, changed_window_settings.run_if(resource_changed::<WindowSettings>));
    }
}

fn changed_window_settings(settings: Res<WindowSettings>, mut window: Single<&mut Window>) {
    window.resolution.set(settings.window_resolution.x, settings.window_resolution.y);
    window.decorations = true;
}
#[derive(Resource, Clone, Copy)]
struct WindowSettings {
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
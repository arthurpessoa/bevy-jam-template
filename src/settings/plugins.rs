use crate::settings::actions::Action;
use crate::settings::resources::WindowSettings;
use crate::settings::systems::{changed_window_settings, config_input};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::plugin::InputManagerPlugin;

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
            .add_plugins(InputManagerPlugin::<Action>::default())
            .add_plugins(WireframePlugin)
            .add_plugins(WorldInspectorPlugin::new())
            .add_systems(Update, changed_window_settings.run_if(resource_changed::<WindowSettings>))
            .add_systems(Startup, config_input);
    }
}

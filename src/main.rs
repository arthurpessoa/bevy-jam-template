mod settings;
mod screen;
mod ui;
mod terrain;

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use screen::StatePlugin;
use settings::SettingsPlugin;
use ui::UIPlugin;
use crate::terrain::TerrainPlugin;

fn main() {
    App::new()
        .add_plugins((
            SettingsPlugin,
            StatePlugin,
            UIPlugin,
            TerrainPlugin,
            PanOrbitCameraPlugin
        )).run();
}
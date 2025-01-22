mod settings;
mod ui;
mod terrain;
mod screen;

use crate::terrain::TerrainPlugin;
use crate::screen::ScreenPlugin;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use settings::SettingsPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins((
            SettingsPlugin,
            ScreenPlugin,
            UIPlugin,
            TerrainPlugin,
            PanOrbitCameraPlugin
        )).run();
}
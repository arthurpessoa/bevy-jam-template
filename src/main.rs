mod settings;
mod screen;

use crate::screen::ScreenPlugin;
use crate::settings::SettingsPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            SettingsPlugin,
            ScreenPlugin,
        )).run();
}
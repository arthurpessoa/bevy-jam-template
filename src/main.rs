mod settings;
mod screen;
mod ui;

use bevy::prelude::*;
use screen::StatePlugin;
use settings::SettingsPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins((
            SettingsPlugin,
            StatePlugin,
            UIPlugin,
        )).run();
}



use crate::settings::actions::Action;
use crate::settings::resources::WindowSettings;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(crate) fn changed_window_settings(settings: Res<WindowSettings>, mut window: Single<&mut Window>) {
    window.resolution.set(settings.window_resolution.x, settings.window_resolution.y);
    window.decorations = true;
}

pub(crate) fn config_input(mut commands: Commands) {
    let input_map = InputMap::new([(Action::Accept, KeyCode::Space)]);

    commands.spawn(InputManagerBundle::with_map(input_map));
}
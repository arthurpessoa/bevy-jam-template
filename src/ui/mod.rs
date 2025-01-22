mod game_ui;
mod main_menu;

use crate::screen::GameState;
use bevy::prelude::*;
use game_ui::*;
use leafwing_input_manager::prelude::*;
pub use main_menu::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<UIAction>::default())
            .add_systems(Update, on_enter_main_menu_ui)
            .add_systems(Update, jump);
    }
}
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum UIAction {
    Accept,
    Cancel,
}

#[derive(Component, Clone, Default, Debug)]
pub struct InGameUI;


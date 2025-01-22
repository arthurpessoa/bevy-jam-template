mod game_ui;

use bevy::prelude::*;
use game_ui::*;
use leafwing_input_manager::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<UIAction>::default());
    }
}
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub(crate) enum UIAction {
    Accept,
    Cancel,
}

#[derive(Component, Clone, Default, Debug)]
pub struct InGameUI;

#[derive(Component, Clone, Default, Debug)]
#[require(Camera2d)]
pub struct MainMenuUI;

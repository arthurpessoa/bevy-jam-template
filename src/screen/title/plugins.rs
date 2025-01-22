use crate::screen::states::ScreenState;
use crate::screen::title::systems::{cleanup, setup};
use bevy::app::App;
use bevy::prelude::{OnEnter, OnExit, Plugin};

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::InTitle), setup);
        app.add_systems(OnExit(ScreenState::InTitle), cleanup);
    }
}
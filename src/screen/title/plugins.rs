use crate::screen::states::ScreenState;
use crate::screen::title::systems::{cleanup, enter_world, setup};
use bevy::prelude::*;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(ScreenState::InTitle), setup)
            .add_systems(Update, enter_world.run_if(in_state(ScreenState::InTitle)))
            .add_systems(OnExit(ScreenState::InTitle), cleanup);
    }
}
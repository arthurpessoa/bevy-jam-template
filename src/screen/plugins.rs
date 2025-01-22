use crate::screen::title::TitleScreenPlugin;
use crate::screen::states::ScreenState;
use bevy::prelude::*;
use std::fmt::Debug;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<ScreenState>()
            .add_plugins(TitleScreenPlugin)
        ;
    }
}
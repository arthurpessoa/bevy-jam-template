use crate::screen::states::ScreenState;
use crate::screen::TitleScreenPlugin;
use crate::screen::WorldScreenPlugin;
use bevy::prelude::*;
use std::fmt::Debug;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<ScreenState>()
            .add_plugins(TitleScreenPlugin)
            .add_plugins(WorldScreenPlugin)
        ;
    }
}
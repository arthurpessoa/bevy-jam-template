use crate::screen::title::components::TitleScreen;
use bevy::prelude::*;

pub(super) fn setup(mut commands: Commands) {
    commands.spawn((
        TitleScreen
    ));
    print!("TitleScreen setup\n");
}


pub(super) fn cleanup() {}
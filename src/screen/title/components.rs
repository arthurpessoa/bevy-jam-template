use crate::ui::MainMenuUI;
use bevy::prelude::*;
#[derive(Component, Clone, Default, Debug)]
#[require(MainMenuUI)]
pub struct TitleScreen;


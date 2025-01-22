use bevy::prelude::Component;
use crate::terrain::Terrain;
use crate::ui::InGameUI;

#[derive(Component, Clone, Default, Debug)]
#[require(
    Terrain,
    InGameUI
)]
pub struct InGame;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Clipmap {
    pub heightmap: Handle<Image>,
}
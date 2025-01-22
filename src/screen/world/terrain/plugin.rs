
use bevy::prelude::*;
use crate::screen::terrain::systems::on_terrain_added;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_terrain_added);
    }
}
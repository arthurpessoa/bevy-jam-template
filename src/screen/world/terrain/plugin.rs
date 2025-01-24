use crate::screen::terrain::materials::CustomMaterial;
use crate::screen::terrain::systems::*;
use bevy::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(Startup, load_clipmap)
            .add_systems(Update, on_terrain_added);
    }
}
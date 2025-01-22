use crate::screen::states::ScreenState;
use crate::screen::terrain::TerrainPlugin;
use crate::screen::world::systems::*;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

pub struct WorldScreenPlugin;

impl Plugin for WorldScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((TerrainPlugin, PanOrbitCameraPlugin))

            .add_systems(OnEnter(ScreenState::InWorld), setup)
            .add_systems(OnExit(ScreenState::InWorld), cleanup);
    }
}
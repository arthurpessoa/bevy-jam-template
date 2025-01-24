use crate::screen::terrain::Terrain;
use crate::screen::world::components::WorldScreen;
use bevy::prelude::*;

pub(super) fn setup(mut commands: Commands) {
    commands.spawn((
        WorldScreen,
        Terrain,
    ));
}

pub(super) fn cleanup(mut commands: Commands, query: Query<Entity, With<WorldScreen>>) {
    for entity in query.iter() {
        println!("Despawning title screen entity: {:?}", entity);
        commands.entity(entity).despawn_descendants();
        commands.entity(entity).despawn();
    }
}
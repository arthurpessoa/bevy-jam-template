use crate::screen::world::terrain::Terrain;
use bevy::pbr::light_consts::lux::OVERCAST_DAY;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use std::f32::consts::PI;
use std::time::Instant;

pub fn on_terrain_added(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<Entity, Added<Terrain>>,
) {
    let start = Instant::now();

    for entity in query.iter() {
        let mesh = meshes.add(Mesh::from(
            Plane3d::default()
                .mesh()
                .size(100., 100.)
                .subdivisions(10)
        ));

        commands.entity(entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(0., 0., 0.)
        ));

        //Light
        commands.spawn((
            DirectionalLight {
                color: Color::WHITE,
                illuminance: OVERCAST_DAY,
                shadows_enabled: true,
                ..default()
            },
            Transform {
                translation: Vec3::new(0., 2., 0.),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
        ));


        //Camera
        commands.spawn((
            Camera3d::default(),
            PanOrbitCamera {
                button_orbit: MouseButton::Right,
                button_pan: MouseButton::Middle,
                ..default()
            },
            Transform::from_xyz(0., 20., 75.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ));
        let duration = start.elapsed();

        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
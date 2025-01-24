use crate::screen::terrain::materials::{CustomMaterial, ATTRIBUTE_BLEND_COLOR};
use crate::screen::terrain::resources::Clipmap;
use crate::screen::world::terrain::Terrain;
use bevy::pbr::light_consts::lux::OVERCAST_DAY;
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use std::f32::consts::PI;
use std::time::Instant;

pub fn on_terrain_added(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cmaterials: ResMut<Assets<CustomMaterial>>,

    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<Entity, Added<Terrain>>,
    clipmap: Res<Clipmap>,
) {
    let start = Instant::now();

    for entity in query.iter() {
        let mesh = meshes.add(Mesh::from(
            Plane3d::default()
                .mesh()
                .size(100., 100.)
                .subdivisions(10)
        ).with_inserted_attribute(
            ATTRIBUTE_BLEND_COLOR,
            // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
            vec![[1.0, 0.0, 0.0, 1.0]; 144],
        ));

        commands.spawn((
            Camera2d,
            Camera {
                order: 1,
                ..default()
            },
        ));

        //commands.spawn(Sprite::from_image(clipmap.heightmap.clone()));

        commands.entity(entity).insert((
            Wireframe,
            Mesh3d(mesh),
            MeshMaterial3d(cmaterials.add(CustomMaterial {
                color: LinearRgba::WHITE,
            })),
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


        /*        let mesh2 = Mesh::from(Cuboid::default())
                    // Sets the custom attribute
                    .with_inserted_attribute(
                        ATTRIBUTE_BLEND_COLOR,
                        // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
                        vec![[1.0, 0.0, 0.0, 1.0]; 24],
                    );

                commands.spawn((
                    Mesh3d(meshes.add(mesh2)),
                    MeshMaterial3d(cmaterials.add(CustomMaterial {
                        color: LinearRgba::WHITE,
                    })),
                    Transform::from_xyz(0.0, 0.5, 0.0),
                ));
        */
        //Camera
        commands.spawn((
            Camera3d::default(),
            Camera {
                order: 0,
                ..default()
            },
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

pub fn load_clipmap(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(Clipmap { heightmap: asset_server.load("textures/debug.png") });
    println!("Clipmap loaded");
}

use bevy::prelude::*;

use crate::StrawberryPlant;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Resource)]
struct PlantToViz(StrawberryPlant);

pub fn viz_plant(plant: StrawberryPlant) {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(PlantToViz(plant))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_config, draw_nodes, draw_mesh),
        )
        .run();
}

fn draw_nodes(mut gizmos: Gizmos, mut plant: ResMut<PlantToViz>) {
    let plant = &plant.0.export();
    for node in plant {
        gizmos.sphere(node.translation, Quat::IDENTITY, 0.1, Color::RED);
    }
}

fn draw_mesh(mut gizmos: Gizmos, mut plant: ResMut<PlantToViz>) {
    let mesh = plant.0.generate_mesh();
    for vertex_id in mesh.vertex_iter() {
        let vertex: Vec3 = mesh.vertex_position(vertex_id).into();
        gizmos.sphere(vertex.into(), Quat::IDENTITY, 0.02, Color::YELLOW_GREEN);
    }
    for face_id in mesh.face_iter() {
        let (a, b, c) = mesh.face_positions(face_id);
        for (a, b) in vec![(a, b), (b, c), (c, a)] {
            gizmos.line(a.into(), b.into(), Color::YELLOW_GREEN);
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut plant: ResMut<PlantToViz>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 1.5, 12.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let mesh = plant.0.generate_mesh();
    let cube_mesh_handle: Handle<Mesh> = meshes.add(mesh.bevy_mesh());

    commands.spawn((PbrBundle {
        mesh: cube_mesh_handle,
        material: materials.add(Color::GRAY.into()),
        ..default()
    },));
}

fn update_config(mut config: ResMut<GizmoConfig>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    if keyboard.just_pressed(KeyCode::D) {
        config.depth_bias = if config.depth_bias == 0. { -1. } else { 0. };
    }
    if keyboard.just_pressed(KeyCode::P) {
        // Toggle line_perspective
        config.line_perspective ^= true;
        // Increase the line width when line_perspective is on
        config.line_width *= if config.line_perspective { 5. } else { 1. / 5. };
    }

    if keyboard.pressed(KeyCode::Right) {
        config.line_width += 5. * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        config.line_width -= 5. * time.delta_seconds();
    }
}

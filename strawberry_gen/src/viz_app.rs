use std::f32::consts::PI;

use bevy::prelude::*;

use crate::StrawberryPlant;

#[derive(Resource)]
struct PlantToViz(StrawberryPlant);


pub fn viz_plant(plant: StrawberryPlant) {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PlantToViz(plant))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_camera, update_config, draw_nodes,draw_mesh))
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
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 12.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
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

    // example instructions
    // commands.spawn(
    //     TextBundle::from_section(
    //         "Press 'D' to toggle drawing gizmos on top of everything else in the scene\n\
    //         Press 'P' to toggle perspective for line gizmos\n\
    //         Hold 'Left' or 'Right' to change the line width",
    //         TextStyle {
    //             font_size: 20.,
    //             ..default()
    //         },
    //     )
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         top: Val::Px(12.0),
    //         left: Val::Px(12.0),
    //         ..default()
    //     }),
    // );
}

// fn system(mut gizmos: Gizmos, time: Res<Time>) {
//     gizmos.cuboid(
//         Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.)),
//         Color::BLACK,
//     );
//     gizmos.rect(
//         Vec3::new(time.elapsed_seconds().cos() * 2.5, 1., 0.),
//         Quat::from_rotation_y(PI / 2.),
//         Vec2::splat(2.),
//         Color::GREEN,
//     );

//     gizmos.sphere(Vec3::new(1., 0.5, 0.), Quat::IDENTITY, 0.5, Color::RED);

//     for y in [0., 0.5, 1.] {
//         gizmos.ray(
//             Vec3::new(1., y, 0.),
//             Vec3::new(-3., (time.elapsed_seconds() * 3.).sin(), 0.),
//             Color::BLUE,
//         );
//     }

//     // Circles have 32 line-segments by default.
//     gizmos.circle(Vec3::ZERO, Vec3::Y, 3., Color::BLACK);
//     // You may want to increase this for larger circles or spheres.
//     gizmos
//         .circle(Vec3::ZERO, Vec3::Y, 3.1, Color::NAVY)
//         .segments(64);
//     gizmos
//         .sphere(Vec3::ZERO, Quat::IDENTITY, 3.2, Color::BLACK)
//         .circle_segments(64);
// }

fn rotate_camera(mut query: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    let mut transform = query.single_mut();

    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.));
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
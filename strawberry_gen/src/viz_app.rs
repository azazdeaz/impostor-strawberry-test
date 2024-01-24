use aery::prelude::*;
use bevy::{prelude::*, transform::commands};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iter_tools::Itertools;

use crate::{
    AxisUp, MeshMap, ParticlePosition, PlantPhysicsPlugin, Stem, StrawberryPlant,
    StrawberryPlantPlugin,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Resource)]
struct PlantToViz(StrawberryPlant);

pub fn viz_plant() {
    App::new()
        .add_plugins((DefaultPlugins, Aery))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((PlantPhysicsPlugin, StrawberryPlantPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_config, draw_nodes, draw_mesh, debug_draw_mesh))
        .run();
}

fn draw_nodes(mut gizmos: Gizmos, particles: Query<&ParticlePosition>) {
    for particle in &particles {
        gizmos.sphere(**particle, Quat::IDENTITY, 0.1, Color::RED);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    //
    commands.spawn(StrawberryPlant);
    commands.spawn((
        PlantMesh,
        PbrBundle {
            material: materials.add(Color::GREEN.with_a(0.3).into()),
            ..default()
        },
    ));
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

#[derive(Component)]
struct PlantMesh;

fn draw_mesh(
    mut commands: Commands,
    mut target: Query<(Entity, &mut Handle<Mesh>), With<PlantMesh>>,
    roots: Query<Entity, Root<AxisUp>>,
    changed_particles: Query<Entity, Changed<ParticlePosition>>,
    stems: Query<((&Stem, &Transform), Relations<AxisUp>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gizmos: Gizmos,
) {
    if changed_particles.is_empty() {
        return;
    }

    let mut mesh = MeshMap::default();
    let ring_resolution = 6;
    let mut rings = Vec::new();
    stems
        .traverse::<AxisUp>(roots.iter())
        .for_each(|(stem, transform), _| {
            let ring = (0..ring_resolution)
                .map(|i| {
                    let angle = i as f32 / ring_resolution as f32 * std::f32::consts::TAU;
                    let relative_transform = Transform::from_rotation(Quat::from_rotation_y(angle))
                        * Transform::from_translation(Vec3::X * stem.size);
                    let transform = **transform * relative_transform;
                    mesh.add_vertex(transform.translation)
                })
                .collect_vec();
            rings.push(ring);
        });

    for ab in rings.windows(2) {
        let a = &ab[0];
        let b = &ab[1];
        for i in 0..ring_resolution {
            let j = (i + 1) % ring_resolution;
            mesh.add_face((a[i], a[j], b[i]));
            mesh.add_face((b[i], a[j], b[j]));
        }
    }

    // Update mesh
    let mut target = target.single_mut();
    *target.1 = meshes.add(mesh.bevy_mesh());
    commands.entity(target.0).insert(mesh);
}

fn debug_draw_mesh(mut mesh_maps: Query<&MeshMap>, mut gizmos: Gizmos) {
    for mesh in &mesh_maps {
        // Draw the mesh triangles
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
}

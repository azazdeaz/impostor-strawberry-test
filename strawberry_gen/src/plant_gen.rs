use aery::prelude::*;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::*;
use iter_tools::Itertools;

use crate::MeshMap;

pub struct StrawberryPlant {
    pub world: World,
}

#[derive(Component, Debug)]
struct Stem {
    size: f32,
    length: f32,
    rotation: Quat,
}
impl Stem {
    pub fn new(size: f32, length: f32, rotation: Quat) -> Self {
        Self {
            size,
            length,
            rotation,
        }
    }
    pub fn simple() -> Self {
        Self::new(1.0, 1.0, Quat::default())
    }
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    pub fn with_length(mut self, length: f32) -> Self {
        self.length = length;
        self
    }
}

#[derive(Component)]
struct RootAxe;

#[derive(Relation)]
struct AxisUp;

impl StrawberryPlant {
    pub fn new() -> Self {
        Self {
            world: World::default(),
        }
    }
    pub fn init(&mut self) {
        self.world
            .spawn((
                Stem::simple().with_length(1.1),
                Transform::default(),
                RootAxe,
            ))
            .scope::<AxisUp>(|scope| {
                scope
                    .add((Stem::simple().with_length(1.2), Transform::default()))
                    .scope::<AxisUp>(|scope| {
                        scope.add((Stem::simple().with_length(1.3), Transform::default()));
                    });
            });

        let update_transforms = self.world.register_system(update_stem_transforms);
        self.world.run_system(update_transforms).ok();
        let print_stems = self.world.register_system(print_stems);
        self.world.run_system(print_stems).ok();
    }

    pub fn export(&mut self) -> Vec<Transform> {
        let mut query = self.world.query_filtered::<&Transform, With<Stem>>();
        query.iter(&self.world).cloned().collect()
    }

    pub fn generate_mesh(&mut self) -> MeshMap {
        self.world.run_system_once(build_mesh)
    }
}

fn update_stem_transforms(
    // Orient the tree so the `Root`s are in the soil.
    // Aery tracks `Root<R>`, `Branch<R>`, `Leaf<R>` (s) for you
    roots: Query<Entity, Root<AxisUp>>,
    mut plants: Query<((&mut Transform, &Stem), Relations<AxisUp>)>,
) {
    plants
        .traverse_mut::<AxisUp>(roots.iter())
        .track_self()
        .for_each(|(p_transf, _), _, (c_transf, stem), _| {
            **c_transf = **p_transf * Transform::from_translation(Vec3::Y * stem.length);
        });
}

fn print_stems(
    roots: Query<Entity, With<RootAxe>>,
    stems: Query<((&Stem, &Transform), Relations<AxisUp>)>,
) {
    stems
        .traverse::<AxisUp>(roots.iter())
        .for_each(|(stem, transform), _| {
            println!("Stem: {:?} {:?}", stem, transform.translation);
        });
}

fn build_mesh(
    roots: Query<Entity, With<RootAxe>>,
    stems: Query<((&Stem, &Transform), Relations<AxisUp>)>,
) -> MeshMap {
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

    info!("Rings: {:?}", rings);
    for ab in rings.windows(2) {
        let a = &ab[0];
        let b = &ab[1];
        info!("A: {:?} B: {:?}", a, b);
        for i in 0..ring_resolution {
            let j = (i + 1) % ring_resolution;
            info!("add face: {:?} {:?} {:?}", a[i], a[j], b[i]);
            mesh.add_face((a[i], a[j], b[i]));
            info!("add face: {:?} {:?} {:?}", a[j], b[j], b[i]);
            mesh.add_face((b[i], a[j], b[j]));
        }
    }
    info!("Mesh done");
    mesh
}

use std::hash::Hash;

use aery::prelude::*;
use bevy::{prelude::*, utils::HashMap};

fn main() {
    let mut plant = StrawberryPlant::new();
    plant.init();
}

struct StrawberryPlant {
    world: World,
}

#[derive(Component, Debug)]
struct Stem {
    size: f32,
    length: f32,
    rotation: Quat,
}
impl Stem {
    fn new(size: f32, length: f32, rotation: Quat) -> Self {
        Self {
            size,
            length,
            rotation,
        }
    }
    fn simple() -> Self {
        Self::new(1.0, 1.0, Quat::default())
    }
    fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    fn with_length(mut self, length: f32) -> Self {
        self.length = length;
        self
    }
}

#[derive(Component)]
struct RootAxe;

#[derive(Relation)]
struct AxisUp;

impl StrawberryPlant {
    fn new() -> Self {
        Self {
            world: World::default(),
        }
    }
    fn init(&mut self) {
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
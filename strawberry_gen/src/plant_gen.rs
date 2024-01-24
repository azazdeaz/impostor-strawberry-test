use aery::prelude::*;
use bevy::asset::AssetLoader;
use bevy::ecs::system::{Command, EntityCommand, RunSystemOnce};
use bevy::prelude::*;
use iter_tools::Itertools;

use crate::{MeshMap, ParticleBundle, ParticlePosition};

#[derive(Component, Debug)]
pub struct StrawberryPlant;

#[derive(Component, Debug)]
pub struct Stem {
    pub size: f32,
    pub length: f32,
    pub rotation: Quat,
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
#[derive(Relation)]
pub struct AxisUp;

fn init_plant(mut commands: Commands, plants: Query<Entity, Changed<StrawberryPlant>>) {
    for plant in &plants {
        commands
            .entity(plant)
            .insert((
                Name::new("P1"),
                Stem::simple().with_length(1.1),
                Transform::default(),
            ))
            .scope::<AxisUp>(|scope| {
                scope
                    .add((
                        Name::new("P2"),
                        Stem::simple().with_length(1.2),
                        ParticleBundle::default(),
                    ))
                    .scope::<AxisUp>(|scope| {
                        scope.add((
                            Name::new("P3"),
                            Stem::simple().with_length(1.3),
                            ParticleBundle::default(),
                        ));
                    });
            });
    }
}

fn update_stem_transforms(
    // Orient the tree so the `Root`s are in the soil.
    // Aery tracks `Root<R>`, `Branch<R>`, `Leaf<R>` (s) for you
    roots: Query<Entity, Root<AxisUp>>,
    mut plants: Query<((&mut ParticlePosition, &Stem), Relations<AxisUp>)>,
) {
    plants
        .traverse_mut::<AxisUp>(roots.iter())
        .track_self()
        .for_each(|(prev_pos, _), _, (this_pos, stem), _| {
            this_pos.0 = prev_pos.0 + Vec3::Y * stem.length;
        });
}

fn print_stems(
    roots: Query<Entity, Root<AxisUp>>,
    stems: Query<((&Stem, &Transform), Relations<AxisUp>)>,
) {
    stems
        .traverse::<AxisUp>(roots.iter())
        .for_each(|(stem, transform), _| {
            println!("Stem: {:?} {:?}", stem, transform.translation);
        });
}

pub struct StrawberryPlantPlugin;
impl Plugin for StrawberryPlantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_plant);
    }
}
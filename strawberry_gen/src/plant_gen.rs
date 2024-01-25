use aery::prelude::*;
use bevy::asset::AssetLoader;
use bevy::ecs::system::{Command, EntityCommand, RunSystemOnce};
use bevy::prelude::*;
use iter_tools::Itertools;

use crate::{
    ConstraintToConstraint, EdgeConstraint, MeshMap, ParticleBundle, ParticlePosition, P0, P1,
};

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
        let p3 = commands
            .spawn((
                Name::new("P3"),
                Stem::simple().with_length(1.3),
                ParticleBundle::default(),
            ))
            .id();
        let p2 = commands
            .spawn((
                Name::new("P2"),
                Stem::simple().with_length(1.2),
                ParticleBundle::default(),
            ))
            .set::<AxisUp>(p3)
            .id();
        let p1 = commands
            .spawn((
                Name::new("P1"),
                Stem::simple().with_length(1.1),
                ParticleBundle::default(),
            ))
            .set::<AxisUp>(p2)
            .id();
        let constrain1 = commands
            .spawn((Name::new("C1"), EdgeConstraint::from_rest_length(1.1)))
            .set::<P0>(p1)
            .set::<P1>(p2)
            .id();
        commands
            .spawn((Name::new("C2"), EdgeConstraint::from_rest_length(1.2)))
            .set::<P0>(p2)
            .set::<P1>(p3)
            .set::<ConstraintToConstraint>(constrain1);
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

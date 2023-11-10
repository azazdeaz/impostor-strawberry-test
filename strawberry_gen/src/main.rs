use bevy::prelude::*;
use aery::prelude::*;

fn main() {
    let mut plant = StrawberryPlant::new();
    plant.init();
}

struct StrawberryPlant {
    world: World,
}

#[derive(Component, Debug)]
struct StemNode {
    size: f32,
    length: f32,
    rotation: Quat,
}
impl StemNode {
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
        self.world.spawn((StemNode::simple(), RootAxe)).scope::<AxisUp>(|scope| {
            scope.add(StemNode::simple()).scope::<AxisUp>(|scope| {
                scope.add(StemNode::simple());
            });
        });

        let update_transforms = self.world.register_system(update_stem_transforms);
        self.world.run_system(update_transforms);
    }
}

fn update_stem_transforms(roots: Query<Entity, With<RootAxe>>, mut stems: Query<(&StemNode, Relations<AxisUp>)>) {
    stems.traverse_mut::<AxisUp>(roots.iter()).for_each(|ref mut stem, _| {
        println!("Stem: {:?}", stem);
        // TCF::Continue;
    });
}
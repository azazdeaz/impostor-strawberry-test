use strawberry_gen::{StrawberryPlant, viz_plant};

fn main() {
    let mut plant = StrawberryPlant::new();
    plant.init();

    viz_plant(plant);
}

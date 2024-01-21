use plexus::index::{Flat3, HashIndexer};
use plexus::prelude::*;
use plexus::primitive::cube::Cube;
use plexus::primitive::generate::Position;

type E3 = nalgebra::Point3<f32>;

// type E3 = Point3<R64>;

fn main() {
    // These trait functions are imported from `prelude`.
    // let (indices, vertices) = Cube::new()
    //     .polygons::<Position<E3>>()
    //     .triangulate()
    //     .index_vertices::<Flat3, _>(HashIndexer::default());

    // println!("Indices: {:?}", indices);
    // println!("Vertices: {:?}", vertices);
}

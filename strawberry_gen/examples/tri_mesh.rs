use three_d_asset::Zero;
use tri_mesh::{Mesh, Vec3};

fn main() {
    let mut mesh = Mesh::new(&three_d_asset::TriMesh::default());
    for _ in 0..23 {
        let vertex_id1 = mesh.add_vertex(Vec3::zero());
        let vertex_id2 = mesh.add_vertex(Vec3::zero());
        let vertex_id3 = mesh.add_vertex(Vec3::zero());
        let face_id = mesh.add_face(vertex_id1, vertex_id2, vertex_id3);

        println!("Added {:?}, Halfedge count: {:?}", face_id, mesh.no_halfedges());
    }
}

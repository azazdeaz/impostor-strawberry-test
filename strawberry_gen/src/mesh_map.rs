use std::ops::Deref;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use iter_tools::Itertools;

#[derive(Component, Debug, Default, Clone)]
pub struct MeshMap {
    vertices: Vec<[f32; 3]>,
    faces: Vec<[VertexId; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexId(u32);
impl From<u32> for VertexId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}
impl Deref for VertexId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaceId(u32);
impl From<u32> for FaceId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}
impl Deref for FaceId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MeshMap {
    pub fn add_vertex<T: Into<[f32; 3]>>(&mut self, vertex: T) -> VertexId {
        let vertex = vertex.into();
        let index = self.vertices.len() as u32;
        self.vertices.push(vertex);
        self.normals.push([0.0, 0.0, 0.0]);
        self.uvs.push([0.0, 0.0]);
        index.into()
    }
    pub fn add_face<T: Into<[VertexId; 3]>>(&mut self, face: T) -> FaceId {
        let index = self.faces.len() as u32;
        self.faces.push(face.into());
        index.into()
    }
    pub fn vertex_iter(&self) -> impl Iterator<Item = VertexId> {
        (0..self.vertices.len() as u32).map(|i| i.into())
    }
    pub fn face_iter(&self) -> impl Iterator<Item = FaceId> {
        (0..self.faces.len() as u32).map(|i| i.into())
    }
    pub fn vertex_position(&self, vertex: VertexId) -> [f32; 3] {
        self.vertices[*vertex as usize]
    }
    pub fn face_vertices(&self, face: FaceId) -> [VertexId; 3] {
        self.faces[*face as usize]
    }
    pub fn face_positions(&self, face: FaceId) -> ([f32; 3], [f32; 3], [f32; 3]) {
        let [a, b, c] = self.face_vertices(face);
        (
            self.vertex_position(a),
            self.vertex_position(b),
            self.vertex_position(c),
        )
    }
    pub fn face_center(&self, face: FaceId) -> [f32; 3] {
        let (a, b, c) = self.face_positions(face);
        [
            (a[0] + b[0] + c[0]) / 3.0,
            (a[1] + b[1] + c[1]) / 3.0,
            (a[2] + b[2] + c[2]) / 3.0,
        ]
    }
    pub fn compute_face_normal(&self, face: FaceId) -> [f32; 3] {
        let (a, b, c) = self.face_positions(face);
        let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let normal = [
            ab[1] * ac[2] - ab[2] * ac[1],
            ab[2] * ac[0] - ab[0] * ac[2],
            ab[0] * ac[1] - ab[1] * ac[0],
        ];
        normal
    }
    pub fn compute_vertex_normal(&self, vertex: VertexId) -> [f32; 3] {
        let mut normal = [0.0, 0.0, 0.0];
        let mut count = 0;
        for face in self.face_iter() {
            let [a, b, c] = self.face_vertices(face);
            if a == vertex || b == vertex || c == vertex {
                let face_normal = self.compute_face_normal(face);
                normal[0] += face_normal[0];
                normal[1] += face_normal[1];
                normal[2] += face_normal[2];
                count += 1;
            }
        }
        normal[0] /= count as f32;
        normal[1] /= count as f32;
        normal[2] /= count as f32;
        normal
    }
    pub fn update_normals(&mut self) {
        for vertex in self.vertex_iter() {
            let normal = self.compute_vertex_normal(vertex);
            self.normals[*vertex as usize] = normal;
        }
    }
    pub fn set_uv<T: Into<[f32; 2]>>(&mut self, vertex: VertexId, uv: T) {
        self.uvs[*vertex as usize] = uv.into();
    }

    pub fn bevy_mesh(&self) -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone())
            .with_indices(Some(Indices::U32(
                self.faces
                    .iter()
                    .flat_map(|[a, b, c]| vec![**a, **b, **c])
                    .collect_vec(),
            )))
    }
}

use std::hash::Hash;

use crate::vector::{RVec3, VecComponent};

mod impls;

#[derive(PartialEq)]
pub struct Vertex<P: VertexPosition, UV: VertexUV> {
    pub position: RVec3<P>,
    pub uv: UV,
}

impl<P: VertexPosition, UV: VertexUV> Eq for Vertex<P, UV> {}

impl<P: VertexPosition, UV: VertexUV> Clone for Vertex<P, UV> {
    fn clone(&self) -> Self {
        Vertex {
            position: self.position,
            uv: self.uv,
        }
    }
}

impl<P: VertexPosition, UV: VertexUV> Copy for Vertex<P, UV> {}

pub trait VertexPosition: 'static + VecComponent + PartialEq + Hash + Send + Sync {
    fn to_f32(&self) -> f32;
    fn from_f32(val: f32) -> Self;
}

pub trait VertexUV: 'static + Copy + PartialEq + Hash + Send + Sync {
    fn to_f32x2(&self) -> [f32; 2];
}

impl<P: VertexPosition, UV: VertexUV + Hash> Hash for Vertex<P, UV> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.uv.hash(state);
    }
}

impl<P: VertexPosition, UV: VertexUV> Vertex<P, UV> {
    pub fn new(position: RVec3<P>, uv: UV) -> Self {
        Vertex { position, uv }
    }
}

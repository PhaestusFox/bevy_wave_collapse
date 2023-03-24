mod vector;

mod wave_mesh;
pub mod vertex;
mod errors;

#[cfg(feature="with_bevy")]
/// I cant be fucked making two copys of this one using bevy handles the other just with owned data
pub mod objects;

pub mod prelude {
    pub use super::wave_mesh::WaveMesh;
    pub use super::wave_mesh::WaveBuilder;
    pub use super::vector::RVec3;
    #[cfg(feature="with_bevy")]
    pub use super::wave_mesh::loader::WaveMeshObjLoader;
    pub use super::vertex::{VertexPosition, VertexUV};
}
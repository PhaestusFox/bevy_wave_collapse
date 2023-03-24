mod vector;

mod errors;
pub mod vertex;
mod wave_mesh;

#[cfg(feature = "with_bevy")]
/// I cant be fucked making two copys of this one using bevy handles the other just with owned data
pub mod objects;

pub mod prelude {
    pub use super::vector::RVec3;
    pub use super::vertex::{VertexPosition, VertexUV};
    #[cfg(feature = "with_bevy")]
    pub use super::wave_mesh::loader::WaveMeshObjLoader;
    pub use super::wave_mesh::WaveBuilder;
    pub use super::wave_mesh::WaveMesh;
}

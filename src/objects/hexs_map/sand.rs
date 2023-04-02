use super::*;
use crate::{
    errors::BakeError,
    objects::Connection,
    prelude::RVec3,
    vertex::{VertexPosition, VertexUV},
    wave_mesh::{WaveBuilder, WaveMesh},
};
use fixed::{types::extra::LeEqU32, FixedI32};
pub struct Sand;

impl Sand {
    pub fn new<'a, P: LeEqU32, UV: VertexUV, Seed: Into<u64>, Data>(
        asset_server: &AssetServer,
        path: &str,
    ) -> WaveObject<FixedI32<P>, UV, Seed, Data>
    where
        FixedI32<P>: VertexPosition,
    {
        let mut meshes = HashMap::new();
        meshes.insert(Connection::new("Core"), asset_server.load(path));
        WaveObject {
            meshes,
            build_fn: Sand::bake,
            can_connect_fn: Sand::can_connect,
        }
    }
    pub fn bake<'a, P: LeEqU32, UV: VertexUV, Seed: Into<u64>, Data>(
        obj: &WaveObject<FixedI32<P>, UV, Seed, Data>,
        offset: RVec3<FixedI32<P>>,
        meshs: &Assets<WaveMesh<FixedI32<P>, UV>>,
        main_mesh: &mut WaveBuilder<FixedI32<P>, UV>,
        _neighbours: &Data,
        _id: Seed,
    ) -> Result<(), BakeError>
    where
        FixedI32<P>: VertexPosition,
    {
        main_mesh.bake(
            offset,
            meshs
                .get(
                    obj.get("Core")
                        .ok_or(BakeError::MeshNotSet("Core", "Sand"))?,
                )
                .ok_or(BakeError::MeshNotFound("Sand Core"))?,
        )
    }
    pub fn can_connect(connection: Connection) -> bool {
        connection == Connection::new("Sand")
    }
}

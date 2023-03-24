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
    pub fn new<P: LeEqU32, UV: VertexUV, Id: Into<u64>>(
        asset_server: &AssetServer,
        path: &str,
    ) -> WaveObject<FixedI32<P>, UV, Id, 6>
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
    pub fn bake<P: LeEqU32, UV: VertexUV, Id: Into<u64>, const N: usize>(
        obj: &WaveObject<FixedI32<P>, UV, Id, N>,
        offset: RVec3<FixedI32<P>>,
        meshs: &Assets<WaveMesh<FixedI32<P>, UV>>,
        main_mesh: &mut WaveBuilder<FixedI32<P>, UV>,
        _neighbours: [&WaveObject<FixedI32<P>, UV, Id, N>; N],
        _id: Id,
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

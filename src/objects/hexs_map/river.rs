use super::super::*;
use super::*;
use crate::{
    errors::BakeError,
    prelude::RVec3,
    vertex::{VertexPosition, VertexUV},
    wave_mesh::{WaveBuilder, WaveMesh},
};
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

pub struct RiverObject;

use bevy::asset::AssetPath;
use fixed::{types::extra::LeEqU32, FixedI32};
impl RiverObject {
    pub fn new<'a, P: LeEqU32, UV: VertexUV, Seed: Into<u64>>(
        asset_server: &AssetServer,
        path: &str,
    ) -> WaveObject<FixedI32<P>, UV, Seed, WaveObjects<'a, FixedI32<P>, UV, Seed, 6>>
    where
        FixedI32<P>: VertexPosition,
    {
        use ConnectionType::*;
        let mut meshes = HashMap::new();
        let base_path: PathBuf = path.into();

        for connection in ConnectionType::iter() {
            match connection {
                Core => {
                    let path = AssetPath::new(base_path.clone(), None);
                    meshes.insert(Connection::from(Core), asset_server.load(path));
                }
                not_core => {
                    let name: &'static str = not_core.into();
                    let path = AssetPath::new(base_path.clone(), Some(format!("{}", name)));
                    meshes.insert(Connection::from(not_core), asset_server.load(path));
                }
            }
        }
        WaveObject {
            meshes,
            can_connect_fn: RiverObject::can_connect,
            build_fn: RiverObject::bake,
        }
    }
    pub fn bake<'a, P: LeEqU32, UV: VertexUV, Seed: Into<u64>>(
        obj: &WaveObject<FixedI32<P>, UV, Seed, WaveObjects<'a, FixedI32<P>, UV, Seed, 6>>,
        offset: RVec3<FixedI32<P>>,
        meshs: &Assets<WaveMesh<FixedI32<P>, UV>>,
        main_mesh: &mut WaveBuilder<FixedI32<P>, UV>,
        neighbours: &WaveObjects<'a, FixedI32<P>, UV, Seed, 6>,
        _id: Seed,
    ) -> Result<(), BakeError>
    where
        FixedI32<P>: VertexPosition,
    {
        use ConnectionType::*;
        use HasConnection::*;
        main_mesh.bake(
            offset,
            meshs
                .get(
                    obj.get(Core)
                        .ok_or(BakeError::MeshNotSet{ mesh: "Core", obj: "River"})?,
                )
                .ok_or(BakeError::MeshNotFound{ mesh: "River Core", obj: "River"})?,
        )?;
        let water_connection = Connection::new("Water");
        let sand_connection = Connection::new("Sand");
        let mut has_connection = [Flat; 6];
        for i in 0..6 {
            if neighbours.0[i].can_connect(water_connection.clone()) {
                has_connection[i] = Water;
            } else if neighbours.0[i].can_connect(sand_connection.clone()) {
                has_connection[i] = Sand;
            }
        }
        for i in 0..6 {
            let stright = match has_connection[i] {
                Water => obj
                    .get(SW)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Stright Water", obj: "River"})?,
                Flat => obj
                    .get(SF)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Stright Flat", obj: "River"})?,
                Sand => obj
                    .get(SS)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Stright Sand", obj: "River"})?,
            };
            let mut stright = meshs
                .get(stright)
                .ok_or(BakeError::MeshNotFound{ mesh: "Stright", obj: "River"})?
                .clone();
            let cos = FixedI32::<P>::ROTATIONS_COS[i];
            let sin = FixedI32::<P>::ROTATIONS_SIN[i];
            stright.rotate(sin, cos);
            main_mesh.bake(offset, &stright)?;
            let corner = match (has_connection[i], has_connection[(i + 1) % 6]) {
                (Water, Water) => obj
                    .get(CWW)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Corner Water Water", obj: "River"})?,
                (Water, Flat) => obj
                    .get(CWF)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Corner Water Flat", obj: "River"})?,
                (Flat, Water) => obj
                    .get(CFW)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Corner Flat Water", obj: "River"})?,
                (Flat, Flat) => obj
                    .get(CFF)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Corner Flat Flat", obj: "River"})?,
                (Flat, Sand) => obj
                    .get(CFS)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Corner Flat Sand", obj: "River"})?,
                (Water, Sand) => obj
                    .get(CWS)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Corner Water Sand", obj: "River"})?,
                (Sand, Flat) => obj
                    .get(CSF)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Corner Sand Flat", obj: "River"})?,
                (Sand, Water) => obj
                    .get(CSW)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Corner Sand Water", obj: "River"})?,
                (Sand, Sand) => obj
                    .get(CSS)
                    .ok_or(BakeError::MeshNotSet{ mesh: "Corner Sand Sand", obj: "River"})?,
            };
            let mut corner = meshs
                .get(corner)
                .ok_or(BakeError::MeshNotFound{ mesh: "Stright", obj: "River"})?
                .clone();
            let cos = FixedI32::<P>::ROTATIONS_COS[i];
            let sin = FixedI32::<P>::ROTATIONS_SIN[i];
            corner.rotate(sin, cos);
            main_mesh.bake(offset, &corner)?;
        }
        Ok(())
    }
    pub fn can_connect(connection: Connection) -> bool {
        connection == Connection::new("Water")
    }
}

#[derive(Debug, Hash, IntoStaticStr, EnumIter, Clone, Copy)]
pub enum ConnectionType {
    Core = 0,
    SW,
    SF,
    CFF,
    CFW,
    CWF,
    CWW,
    SS,
    CFS,
    CSF,
    CWS,
    CSW,
    CSS,
}

#[derive(Debug, Clone, Copy)]
enum HasConnection {
    Flat,
    Water,
    Sand,
}

impl Into<Cow<'static, str>> for ConnectionType {
    fn into(self) -> Cow<'static, str> {
        let str: &'static str = self.into();
        Cow::Borrowed(str)
    }
}

use std::{path::PathBuf};
use super::*;
use super::super::*;
use crate::{wave_mesh::{WaveMesh, WaveBuilder}, vertex::{VertexPosition, VertexUV}, prelude::RVec3, errors::BakeError};
use strum_macros::{IntoStaticStr, EnumIter};
use strum::IntoEnumIterator;

pub struct RiverObject;


use bevy::asset::AssetPath;
use fixed::{FixedI32, types::extra::LeEqU32};
impl RiverObject {
    pub fn new<P: LeEqU32, UV: VertexUV, Id>(asset_server: &AssetServer, path: &str) -> WaveObject<FixedI32::<P>, UV, Id, 6>
    where FixedI32::<P>: VertexPosition
    {
        use ConnectionType::*;
        let mut meshes = HashMap::new();
        let base_path: PathBuf = path.into();

        for connection in ConnectionType::iter() {
            match connection {
                Core => {
                    let path = AssetPath::new(base_path.clone(), None);
                    meshes.insert(Connection::from(Core), asset_server.load(path));
                },
                not_core => {
                    let name: &'static str = not_core.into();
                    let path = AssetPath::new(base_path.clone(), Some(format!("{}", name)));
                    meshes.insert(Connection::from(not_core), asset_server.load(path));
                }
            }
        }
        WaveObject::<FixedI32::<P>, UV, Id, 6> {
            meshes,
            can_connect_fn: RiverObject::can_connect,
            build_fn: RiverObject::bake
        }
    }
    pub fn bake<P: LeEqU32, UV: VertexUV, Id, const N: usize>(obj: &WaveObject<FixedI32::<P>, UV, Id, N>, offset: RVec3<FixedI32::<P>>, meshs: &Assets<WaveMesh<FixedI32::<P>, UV>>, main_mesh: &mut WaveBuilder<FixedI32::<P>, UV>, neighbours: [&WaveObject<FixedI32::<P>, UV, Id, N>; N], _id: Id) -> Result<(), BakeError>
    where FixedI32::<P>: VertexPosition
    {
        use ConnectionType::*;
        use HasConnection::*;
        main_mesh.bake(offset, meshs.get(obj.get(Core).ok_or(BakeError::MeshNotSet("Core", "River"))?).ok_or(BakeError::MeshNotFound("River Core"))?)?;
        let water_connection = Connection::new("Water");
        let sand_connection = Connection::new("Sand");
        let mut has_connection = [Flat; 6];
        for i in 0..6 {
            if neighbours[i].can_connect(water_connection.clone()) {
                has_connection[i] = Water;
            } else if neighbours[i].can_connect(sand_connection.clone()) {
                has_connection[i] = Sand;
            }
        }
        for i in 0..6 {
            let stright = match has_connection[i] {
                Water => obj.get(SW).ok_or(BakeError::MeshNotSet("Stright Water", "River"))?,
                Flat => obj.get(SF).ok_or(BakeError::MeshNotSet("Stright Flat", "River"))?,
                Sand => obj.get(SS).ok_or(BakeError::MeshNotSet("Stright Sand", "River"))?
            };
            let mut stright = meshs.get(stright).ok_or(BakeError::MeshNotFound("Stright"))?.clone();
            let cos = FixedI32::<P>::ROTATIONS_COS[i];
            let sin = FixedI32::<P>::ROTATIONS_SIN[i];
            stright.rotate(sin, cos);
            main_mesh.bake(offset, &stright)?;
            let corner = match (has_connection[i], has_connection[(i + 1) % 6]) {
                (Water, Water) => obj.get(CWW).ok_or(  BakeError::MeshNotSet("Corner Water Water", "River"))?,
                (Water, Flat) => obj.get(CWF).ok_or( BakeError::MeshNotSet("Corner Water Flat", "River"))?,
                (Flat, Water) => obj.get(CFW).ok_or( BakeError::MeshNotSet("Corner Flat Water", "River"))?,
                (Flat, Flat) => obj.get(CFF).ok_or(BakeError::MeshNotSet("Corner Flat Flat", "River"))?,
                (Flat, Sand) => obj.get(CFS).ok_or(BakeError::MeshNotSet("Corner Flat Sand", "River"))?,
                (Water, Sand) => obj.get(CWS).ok_or(BakeError::MeshNotSet("Corner Water Sand", "River"))?,
                (Sand,  Flat) => obj.get(CSF).ok_or(BakeError::MeshNotSet("Corner Sand Flat", "River"))?,
                (Sand, Water) => obj.get(CSW).ok_or(BakeError::MeshNotSet("Corner Sand Water", "River"))?,
                (Sand,  Sand) => obj.get(CSS).ok_or(BakeError::MeshNotSet("Corner Sand Sand", "River"))?,
            };
            let mut corner = meshs.get(corner).ok_or(BakeError::MeshNotFound("Stright"))?.clone();
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
    Sand
}
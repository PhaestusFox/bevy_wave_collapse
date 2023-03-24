use super::*;
use crate::{
    errors::BakeError,
    objects::Connection,
    prelude::RVec3,
    vertex::{VertexPosition, VertexUV},
    wave_mesh::{WaveBuilder, WaveMesh},
};
use bevy::asset::AssetPath;
use fixed::{types::extra::LeEqU32, FixedI32};
use rand::{SeedableRng, Rng, rngs::StdRng, seq::SliceRandom};
use strum::IntoEnumIterator;
use strum_macros::{IntoStaticStr, EnumIter};
pub struct Desert;

use ConnectionType::*;
impl Desert {
    pub fn new<P: LeEqU32, UV: VertexUV, Id: Into<u64>>(
        asset_server: &AssetServer,
        path: &str,
    ) -> WaveObject<FixedI32<P>, UV, Id, 6>
    where
        FixedI32<P>: VertexPosition,
    {
        let mut meshes = HashMap::new();
        meshes.insert(Connection::new("Core"), asset_server.load(path));
        for connection in ConnectionType::iter() {
            match connection {
                Core => {
                    let path = AssetPath::new(path.into(), None);
                    meshes.insert(Connection::from(Core), asset_server.load(path));
                }
                not_core => {
                    let name: &'static str = not_core.into();
                    let path = AssetPath::new(path.into(), Some(format!("{}", name)));
                    meshes.insert(Connection::from(not_core), asset_server.load(path));
                }
            }
        }
        WaveObject {
            meshes,
            build_fn: Desert::bake,
            can_connect_fn: Desert::can_connect,
        }
    }
    pub fn bake<P: LeEqU32, UV: VertexUV, Id: Into<u64>, const N: usize>(
        obj: &WaveObject<FixedI32<P>, UV, Id, N>,
        offset: RVec3<FixedI32<P>>,
        meshs: &Assets<WaveMesh<FixedI32<P>, UV>>,
        main_mesh: &mut WaveBuilder<FixedI32<P>, UV>,
        _neighbours: [&WaveObject<FixedI32<P>, UV, Id, N>; N],
        id: Id,
    ) -> Result<(), BakeError>
    where
        FixedI32<P>: VertexPosition,
    {
        let mut rng = rand::rngs::StdRng::seed_from_u64(id.into());
        main_mesh.bake(
            offset,
            meshs
                .get(
                    obj.get("Core")
                        .ok_or(BakeError::MeshNotSet("Core", "Sand"))?,
                )
                .ok_or(BakeError::MeshNotFound("Sand Core"))?,
        )?;
        Cactus::new(&mut rng).build(offset, meshs, main_mesh, &obj.meshes)
    }
    pub fn can_connect(connection: Connection) -> bool {
        connection == Connection::new("Sand")
    }
}

#[derive(Debug, Hash, IntoStaticStr, EnumIter, Clone, Copy)]
pub enum ConnectionType {
    Core = 0,
    CactusBranch,
    CactusTop,
    CactusStem,
    CactusBig
}

impl Into<Cow<'static, str>> for ConnectionType {
    fn into(self) -> Cow<'static, str> {
        let str: &'static str = self.into();
        Cow::Borrowed(str)
    }
}

struct Cactus<P: LeEqU32> where FixedI32<P>: VertexPosition {
    hight: FixedI32<P>,
    arms: Vec<CactusArm<P>>,
}

impl<P: LeEqU32> Cactus<P>
where FixedI32<P>: VertexPosition {
    fn new(rng: &mut StdRng) -> Cactus<P> {
        let mut arms = ROTATION::iter().collect::<Vec<_>>();
        arms.shuffle(rng);
        let hight = rng.gen_range(0.25, 0.5);
        Cactus {
            arms: if rng.gen_bool(0.10) {Vec::with_capacity(0)} else {(0..=rng.gen_range(0, 4)).map(|_| {
                CactusArm { slot: arms.pop().expect("Less then 7 Arms"), hight: FixedI32::<P>::from_num(rng.gen_range(0.1, hight - 0.05)), length: FixedI32::<P>::from_num(rng.gen_range(0.05, 0.75)) }
            }).collect()},
            hight: FixedI32::<P>::from_num(hight),
        }
    }

    fn build<UV: VertexUV>(self, offset: RVec3<FixedI32<P>>, meshes: &Assets<WaveMesh<FixedI32<P>, UV>>, main_mesh: &mut WaveBuilder<FixedI32<P>, UV>, wave_meshes: &HashMap<Connection, Handle<WaveMesh<FixedI32<P>, UV>>>) -> Result<(), BakeError> {
        let stem = meshes.get(wave_meshes.get(&Connection::from(CactusStem)).ok_or(BakeError::MeshNotSet("CactusStem", "Desert"))?).ok_or(BakeError::MeshNotFound("CactusStem"))?;
        let mut main_stem = stem.clone();
        main_stem.scale_y(self.hight);
        main_mesh.bake(offset, &main_stem)?;
        let top = meshes.get(wave_meshes.get(&Connection::from(CactusTop)).ok_or(BakeError::MeshNotSet("CactusTop", "Desert"))?).ok_or(BakeError::MeshNotFound("CactusTop"))?;
        main_mesh.bake(offset + RVec3 {y: self.hight, ..Default::default()}, top)?;
        for arm in self.arms {
            let mut top = top.clone();
            top.offset(RVec3 { x: FixedI32::<P>::from_num(0.1), y: arm.hight + arm.length + FixedI32::<P>::from_num(0.1), z: FixedI32::<P>::ZERO });
            let mut arm_stem = stem.clone();
            arm_stem.scale_y(arm.length);
            arm_stem.offset(RVec3 { x: FixedI32::<P>::from_num(0.1), y: arm.hight + FixedI32::<P>::from_num(0.1), z: FixedI32::<P>::ZERO });
            let mut branch = meshes.get(wave_meshes.get(&Connection::from(CactusBranch)).ok_or(BakeError::MeshNotSet("CactusBranch", "Desert"))?).ok_or(BakeError::MeshNotFound("CactusBranch"))?.clone();
            branch.offset(RVec3 {y: arm.hight, ..Default::default()});
            let sin = FixedI32::<P>::ROTATIONS_SIN[arm.slot as usize];
            let cos = FixedI32::<P>::ROTATIONS_COS[arm.slot as usize];
            top.rotate(sin, cos);
            arm_stem.rotate(sin, cos);
            branch.rotate(sin, cos);
            main_mesh.bake(offset, &top)?;
            main_mesh.bake(offset, &arm_stem)?;
            main_mesh.bake(offset, &branch)?;
        }
        Ok(())
    }
}

struct CactusArm<P: LeEqU32> where FixedI32<P>: VertexPosition {
    slot: ROTATION,
    hight: FixedI32<P>,
    length: FixedI32<P>,
}

#[derive(Clone, Copy, EnumIter)]
enum ROTATION {
    Zero = 0,
    One,
    Two,
    Three,
    Fore,
    Five
}
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
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rand_distr::{StandardNormal, Distribution, Binomial, Pert};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};
pub struct Desert;

use ConnectionType::*;
impl Desert {
    pub fn new<P: LeEqU32, UV: VertexUV, Seed: Into<u64>>(
        asset_server: &AssetServer,
        path: &str,
    ) -> WaveObject<FixedI32<P>, UV, Seed, 6>
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
    pub fn bake<P: LeEqU32, UV: VertexUV, Seed: Into<u64>, const N: usize>(
        obj: &WaveObject<FixedI32<P>, UV, Seed, N>,
        offset: RVec3<FixedI32<P>>,
        meshs: &Assets<WaveMesh<FixedI32<P>, UV>>,
        main_mesh: &mut WaveBuilder<FixedI32<P>, UV>,
        _neighbours: [&WaveObject<FixedI32<P>, UV, Seed, N>; N],
        id: Seed,
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
        for (cactus, cactus_offset) in CactusShapes::ThreeDubble.gen_cactus(&mut rng) {
            cactus.build(offset + cactus_offset, meshs, main_mesh, &obj.meshes)?;
        }
        Ok(())
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
    CactusBig,
}

impl Into<Cow<'static, str>> for ConnectionType {
    fn into(self) -> Cow<'static, str> {
        let str: &'static str = self.into();
        Cow::Borrowed(str)
    }
}

struct Cactus<P: LeEqU32>
where
    FixedI32<P>: VertexPosition,
{
    hight: FixedI32<P>,
    arms: Vec<CactusArm<P>>,
}

impl<P: LeEqU32> Cactus<P>
where
    FixedI32<P>: VertexPosition,
{
    fn new(rng: &mut StdRng) -> Cactus<P> {
        let mut arms = ROTATION::iter().collect::<Vec<_>>();
        arms.shuffle(rng);
        let sampler = Pert::new(0.5, 1.2, 0.75).unwrap();
        let hight = rng.sample::<f32, _>(sampler).abs() / 2. + 0.05;
        let num_arms = Binomial::new(4, 0.4).unwrap().sample(rng);
        Cactus {
            arms: if num_arms == 0 {
                Vec::with_capacity(0)
            } else {
                (0..=num_arms)
                    .map(|_| CactusArm {
                        slot: arms.pop().expect("Less then 7 Arms"),
                        hight: FixedI32::<P>::from_num(rng.sample::<f32, _>(Pert::new(0.1, (hight - 0.1).max(0.4), (hight/2.).max(0.15)).unwrap())),
                        length: FixedI32::<P>::from_num(rng.sample::<f32, _>(Pert::new(0.1, 1.5, 0.66).unwrap())),
                    })
                    .collect()
            },
            hight: FixedI32::<P>::from_num(hight),
        }
    }

    fn build<UV: VertexUV>(
        self,
        offset: RVec3<FixedI32<P>>,
        meshes: &Assets<WaveMesh<FixedI32<P>, UV>>,
        main_mesh: &mut WaveBuilder<FixedI32<P>, UV>,
        wave_meshes: &HashMap<Connection, Handle<WaveMesh<FixedI32<P>, UV>>>,
    ) -> Result<(), BakeError> {
        let stem = meshes
            .get(
                wave_meshes
                    .get(&Connection::from(CactusStem))
                    .ok_or(BakeError::MeshNotSet("CactusStem", "Desert"))?,
            )
            .ok_or(BakeError::MeshNotFound("CactusStem"))?;
        let mut main_stem = stem.clone();
        main_stem.scale_y(self.hight);
        main_mesh.bake(offset, &main_stem)?;
        let top = meshes
            .get(
                wave_meshes
                    .get(&Connection::from(CactusTop))
                    .ok_or(BakeError::MeshNotSet("CactusTop", "Desert"))?,
            )
            .ok_or(BakeError::MeshNotFound("CactusTop"))?;
        main_mesh.bake(
            offset
                + RVec3 {
                    y: self.hight,
                    ..Default::default()
                },
            top,
        )?;
        for arm in self.arms {
            let mut top = top.clone();
            top.offset(RVec3 {
                x: FixedI32::<P>::from_num(0.1),
                y: arm.hight + arm.length + FixedI32::<P>::from_num(0.1),
                z: FixedI32::<P>::ZERO,
            });
            let mut arm_stem = stem.clone();
            arm_stem.scale_y(arm.length);
            arm_stem.offset(RVec3 {
                x: FixedI32::<P>::from_num(0.1),
                y: arm.hight + FixedI32::<P>::from_num(0.1),
                z: FixedI32::<P>::ZERO,
            });
            let mut branch = meshes
                .get(
                    wave_meshes
                        .get(&Connection::from(CactusBranch))
                        .ok_or(BakeError::MeshNotSet("CactusBranch", "Desert"))?,
                )
                .ok_or(BakeError::MeshNotFound("CactusBranch"))?
                .clone();
            branch.offset(RVec3 {
                y: arm.hight,
                ..Default::default()
            });
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

struct CactusArm<P: LeEqU32>
where
    FixedI32<P>: VertexPosition,
{
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
    Five,
}

#[derive(Clone, Copy, EnumIter)]
enum CactusShapes {
    One,
    TwoOp,
    TwoSingle,
    TwoDubble,
    ThreeSingle,
    ThreeDubble,
}

impl CactusShapes {
    fn gen_cactus<P: LeEqU32>(
        self,
        rng: &mut rand::rngs::StdRng,
    ) -> Vec<(Cactus<P>, RVec3<FixedI32<P>>)>
    where
        FixedI32<P>: VertexPosition,
    {
        match self {
            CactusShapes::One => {
                let mut offset = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.0..0.36)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                let sin = FixedI32::<P>::ROTATIONS_SIN[rng.gen_range(0..6)];
                let cos = FixedI32::<P>::ROTATIONS_COS[rng.gen_range(0..6)];
                offset.rotate_y(sin, cos);
                vec![(Cactus::new(rng), offset)]
            }
            CactusShapes::TwoOp => {
                let mut offset_one = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.1..0.36)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                let mut offset_two = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(-0.36..-0.1)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                let sin = FixedI32::<P>::ROTATIONS_SIN[rng.gen_range(0..6)];
                let cos = FixedI32::<P>::ROTATIONS_COS[rng.gen_range(0..6)];
                offset_one.rotate_y(sin, cos);
                offset_two.rotate_y(sin, cos);
                vec![
                    (Cactus::new(rng), offset_one),
                    (Cactus::new(rng), offset_two),
                ]
            }
            CactusShapes::TwoSingle => {
                let mut offset_one = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.05..0.30)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                let mut offset_two = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.1..0.36)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                offset_two.rotate_y(
                    FixedI32::<P>::ROTATIONS_SIN[1],
                    FixedI32::<P>::ROTATIONS_COS[1],
                );
                let rot = rng.gen_range(0..6);
                let sin = FixedI32::<P>::ROTATIONS_SIN[rot];
                let cos = FixedI32::<P>::ROTATIONS_COS[rot];
                offset_one.rotate_y(sin, cos);
                offset_two.rotate_y(sin, cos);
                vec![
                    (Cactus::new(rng), offset_one),
                    (Cactus::new(rng), offset_two),
                ]
            }
            CactusShapes::TwoDubble => {
                let mut offset_one = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.05..0.30)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                let mut offset_two = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.1..0.36)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                offset_two.rotate_y(
                    FixedI32::<P>::ROTATIONS_SIN[2],
                    FixedI32::<P>::ROTATIONS_COS[2],
                );
                let rot = rng.gen_range(0..6);
                let sin = FixedI32::<P>::ROTATIONS_SIN[rot];
                let cos = FixedI32::<P>::ROTATIONS_COS[rot];
                offset_one.rotate_y(sin, cos);
                offset_two.rotate_y(sin, cos);
                vec![
                    (Cactus::new(rng), offset_one),
                    (Cactus::new(rng), offset_two),
                ]
            }
            CactusShapes::ThreeSingle => {
                let mut offset_one = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.05..0.30)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                let mut offset_two = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.15..0.36)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                let mut offset_three = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.1..0.25)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                offset_one.rotate_y(
                    FixedI32::<P>::ROTATIONS_SIN[1],
                    FixedI32::<P>::ROTATIONS_COS[1],
                );
                offset_two.rotate_y(
                    FixedI32::<P>::ROTATIONS_SIN[2],
                    FixedI32::<P>::ROTATIONS_COS[2],
                );
                let rot = rng.gen_range(0..6);
                let sin = FixedI32::<P>::ROTATIONS_SIN[rot];
                let cos = FixedI32::<P>::ROTATIONS_COS[rot];
                offset_one.rotate_y(sin, cos);
                offset_two.rotate_y(sin, cos);
                offset_three.rotate_y(sin, cos);
                vec![
                    (Cactus::new(rng), offset_one),
                    (Cactus::new(rng), offset_two),
                    (Cactus::new(rng), offset_three),
                ]
            }
            CactusShapes::ThreeDubble => {
                let mut offset_one = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.05..0.30)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                let mut offset_two = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.15..0.36)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                let mut offset_three = RVec3::new(
                    FixedI32::<P>::from_num(rng.gen_range(0.1..0.25)),
                    FixedI32::<P>::ZERO,
                    FixedI32::<P>::ZERO,
                );
                offset_one.rotate_y(
                    FixedI32::<P>::ROTATIONS_SIN[4],
                    FixedI32::<P>::ROTATIONS_COS[2],
                );
                offset_three.rotate_y(
                    FixedI32::<P>::ROTATIONS_SIN[2],
                    FixedI32::<P>::ROTATIONS_COS[4],
                );
                let rot: usize = rng.gen_range(0..6);
                let sin = FixedI32::<P>::ROTATIONS_SIN[rot];
                let cos = FixedI32::<P>::ROTATIONS_COS[rot];
                offset_one.rotate_y(sin, cos);
                offset_two.rotate_y(sin, cos);
                offset_three.rotate_y(sin, cos);
                vec![
                    (Cactus::new(rng), offset_one),
                    (Cactus::new(rng), offset_two),
                    (Cactus::new(rng), offset_three),
                ]
            }
        }
    }
}

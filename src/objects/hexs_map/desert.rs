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
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::{Distribution, Pert, StandardGeometric};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};
pub struct Desert;

use ConnectionType::*;
impl Desert {
    pub fn new<'a, P: LeEqU32 + Send + Sync, UV: VertexUV, Seed: Into<u64>, Data>(
        asset_server: &AssetServer,
        path: &str,
    ) -> WaveObject<FixedI32<P>, UV, Seed, Data>
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
    pub fn bake<'a, P: LeEqU32 + Send + Sync, UV: VertexUV, Seed: Into<u64>, Data>(
        obj: &WaveObject<FixedI32<P>, UV, Seed, Data>,
        offset: RVec3<FixedI32<P>>,
        meshs: &Assets<WaveMesh<FixedI32<P>, UV>>,
        main_mesh: &mut WaveBuilder<FixedI32<P>, UV>,
        _neighbours: &'a Data,
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

struct Cactus<P: LeEqU32 + Send + Sync>
where
    FixedI32<P>: VertexPosition,
{
    big: bool,
    hight: FixedI32<P>,
    arms: Vec<CactusArm<P>>,
}

impl<P: LeEqU32 + Send + Sync> Cactus<P>
where
    FixedI32<P>: VertexPosition,
{
    fn new(rng: &mut StdRng) -> Cactus<P> {
        let sampler = Pert::new(0.25, 0.5, 0.375).unwrap();
        let hight = rng.sample::<f32, _>(sampler);
        Cactus {
            big: rng.gen_bool(0.25),
            arms: ArmSampler.sample(rng).gen_arm(hight, rng),
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
                    .get(&Connection::from(if self.big {
                        CactusBig
                    } else {
                        CactusStem
                    }))
                    .ok_or(BakeError::MeshNotSet(
                        if self.big { "CactusBig" } else { "CactusStem" },
                        "Desert",
                    ))?,
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
    slot: Rotation,
    hight: FixedI32<P>,
    length: FixedI32<P>,
}

impl<P: LeEqU32 + Send + Sync> CactusArm<P> {
    fn new(rng: &mut StdRng, max_hight: f32, rotation: Rotation) -> CactusArm<P> {
        CactusArm {
            slot: rotation,
            hight: FixedI32::<P>::from_num(
                rng.sample::<f32, _>(
                    Pert::new(
                        0.1,
                        (max_hight - 0.1).max(0.2),
                        (max_hight * 0.25).max(0.15),
                    )
                    .unwrap(),
                ),
            ),
            length: FixedI32::<P>::from_num(rng.sample::<f32, _>(
                Pert::new(0.1, max_hight, (max_hight * 0.66).max(0.15)).unwrap(),
            )),
        }
    }
}

#[derive(Clone, Copy, EnumIter)]
enum Rotation {
    Zero = 0,
    One,
    Two,
    Three,
    Fore,
    Five,
}

impl From<usize> for Rotation {
    fn from(value: usize) -> Self {
        match value % 6 {
            0 => Rotation::Zero,
            1 => Rotation::One,
            2 => Rotation::Two,
            3 => Rotation::Three,
            4 => Rotation::Fore,
            5 => Rotation::Five,
            _ => Rotation::One,
        }
    }
}

#[derive(Clone, Copy, EnumIter)]
enum CactusShapes {
    Zero,
    One,
    TwoOp,
    TwoSingle,
    TwoDubble,
    ThreeSingle,
    ThreeDubble,
}

struct ArmSampler;

impl Distribution<CactusShapes> for ArmSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CactusShapes {
        CactusShapes::arm_distribution(StandardGeometric.sample(rng))
    }
}

struct CactusSampler;

impl Distribution<CactusShapes> for CactusSampler {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CactusShapes {
        CactusShapes::cactus_distribution(StandardGeometric.sample(rng))
    }
}

impl CactusShapes {
    fn gen_cactus<P: LeEqU32 + Send + Sync>(
        self,
        rng: &mut rand::rngs::StdRng,
    ) -> Vec<(Cactus<P>, RVec3<FixedI32<P>>)>
    where
        FixedI32<P>: VertexPosition,
    {
        match self {
            CactusShapes::Zero => {
                vec![]
            }
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

    fn gen_arm<P: LeEqU32 + Send + Sync>(
        self,
        cactus_hight: f32,
        rng: &mut rand::rngs::StdRng,
    ) -> Vec<CactusArm<P>> {
        let rot = rng.gen_range(0..6);
        match self {
            CactusShapes::Zero => {
                vec![]
            }
            CactusShapes::One => {
                vec![CactusArm::new(rng, cactus_hight, Rotation::from(rot))]
            }
            CactusShapes::TwoSingle | CactusShapes::TwoOp => {
                vec![
                    CactusArm::new(rng, cactus_hight, Rotation::from(rot)),
                    CactusArm::new(rng, cactus_hight, Rotation::from(rot + 3)),
                ]
            }
            CactusShapes::TwoDubble => {
                vec![
                    CactusArm::new(rng, cactus_hight, Rotation::from(rot)),
                    CactusArm::new(rng, cactus_hight, Rotation::from(rot + 2)),
                ]
            }
            CactusShapes::ThreeSingle | CactusShapes::ThreeDubble => {
                vec![
                    CactusArm::new(rng, cactus_hight, Rotation::from(rot)),
                    CactusArm::new(rng, cactus_hight, Rotation::from(rot + 2)),
                    CactusArm::new(rng, cactus_hight, Rotation::from(rot + 4)),
                ]
            }
        }
    }

    fn arm_distribution(value: u64) -> Self {
        match value {
            0 => CactusShapes::Zero,
            1 => CactusShapes::One,
            2 => CactusShapes::TwoOp,
            3 => CactusShapes::ThreeDubble,
            4 => CactusShapes::TwoDubble,
            5 => CactusShapes::TwoSingle,
            _ => CactusShapes::ThreeSingle,
        }
    }
    fn cactus_distribution(value: u64) -> Self {
        match value {
            5 => CactusShapes::Zero,
            0 => CactusShapes::One,
            1 => CactusShapes::TwoOp,
            2 => CactusShapes::ThreeDubble,
            3 => CactusShapes::TwoDubble,
            4 => CactusShapes::TwoSingle,
            _ => CactusShapes::ThreeSingle,
        }
    }
}

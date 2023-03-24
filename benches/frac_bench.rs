use std::collections::HashMap;
use criterion::{criterion_group, criterion_main, Criterion};
use fixed::FixedI32;
use typenum::*;
use bevy_wave_collapse::{prelude::*, vertex::{VertexPosition, VertexUV}};
use hex_plugin::{prelude::*, components::ids::HexRangeIterator};
use fixed::types::extra::LeEqU32;

fn load_test_wavemesh<P: LeEqU32 + 'static + Send + Sync>() -> RiverObject<FixedI32<P>, u8> {
    let meshes = bevy_wave_collapse::prelude::WaveMesh::<FixedI32<P>, u8>::from_obj_str(include_str!("river.wfo"))
        .unwrap();
    RiverObject { meshes: [
        meshes.get("CORE").unwrap().clone(),
        meshes.get("SW").unwrap().clone(),
        meshes.get("SF").unwrap().clone(),
        meshes.get("CFF").unwrap().clone(),
        meshes.get("CFW").unwrap().clone(),
        meshes.get("CWF").unwrap().clone(),
        meshes.get("CWW").unwrap().clone(),
    ] }
}

fn frac_test(b: &mut Criterion) {
    let mut group = b.benchmark_group("Fixed Accs");
    let mut res = HashMap::with_capacity(10);
    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U6>();
    group.bench_function("6", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(6,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U8>();
    group.bench_function("8", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(8,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U10>();
    group.bench_function("10", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(10,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U12>();
    group.bench_function("12", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(12,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U14>();
    group.bench_function("14", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(14,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U16>();
    group.bench_function("16", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(16,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U18>();
    group.bench_function("18", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(18,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U20>();
    group.bench_function("20", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(20,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U22>();
    group.bench_function("22", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(22,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U24>();
    group.bench_function("24", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(24,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U26>();
    group.bench_function("26", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(26,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });

    let mut mesh_builder = WaveBuilder::new();
    let mesh = load_test_wavemesh::<U28>();
    group.bench_function("28", |b| {
        b.iter(|| {
            mesh_builder.clear();
            res.insert(28,
            frac_build(&mut mesh_builder, &mesh, [true, true, true, true, true, true]));
        });
    });
    let gen = HexRangeIterator::<ChunkId>::new(5).count();
    println!("Generated {} Hexs", gen);
    println!("Max vertexes = {}", gen * 66);
    for i in (6..30).step_by(2) {
        if let Some(r) = res.get(&i) {
            println!("({}, {})", i, r);
        }
    }
}

fn frac_build<P: LeEqU32 + 'static + Send + Sync>(
    mesh_builder: &mut WaveBuilder<FixedI32<P>, u8>,
    mesh: &RiverObject<FixedI32<P>, u8>,
    is_water: [bool; 6] 
) -> usize {
    for id in HexRangeIterator::<CellId>::new(5).collect::<Vec<_>>() {
        let offset = RVec3::new(FixedI32::<P>::from_f32(id.x()), FixedI32::<P>::from_f32(0.0), FixedI32::<P>::from_f32(id.z()));
        mesh.bake(offset, mesh_builder, is_water).unwrap();
    }
    mesh_builder.vertex_len()
}

criterion_group!(benches, frac_test);
criterion_main!(benches);
use std::hash::Hash;

pub struct RiverObject<P: VertexPosition, UV: VertexUV> {
    pub meshes: [WaveMesh<P, UV>;7],
}

pub enum ConnectionType {
    Core = 0,
    SW,
    SF,
    CFF,
    CFW,
    CWF,
    CWW
}
use bevy_wave_collapse::objects::hexs::HexTrig;

impl<P: fixed::types::extra::LeEqU32 + 'static + Send + Sync, UV: VertexUV + Hash> RiverObject<fixed::FixedI32<P>, UV> {
    pub fn bake(
        &self,
        offset: RVec3<fixed::FixedI32<P>>,
        hights: &mut WaveBuilder<fixed::FixedI32<P>, UV>,
        is_water: [bool; 6],
    ) -> Result<(), String> {
        
        hights.bake(offset, &self.meshes[ConnectionType::Core as usize]).or_else(|e| Err(e.to_string()))?;
        for i in 0..6 {
            let mut stright = self.meshes[if is_water[i] {ConnectionType::SW} else {ConnectionType::SF} as usize].clone();
            let cos = FixedI32::<P>::ROTATIONS_COS[i];
            let sin = FixedI32::<P>::ROTATIONS_SIN[i];
            stright.rotate(sin, cos);
            hights.bake(offset, &stright).or_else(|e| Err(e.to_string()))?;
            let mut corner = self.meshes[match (is_water[i], is_water[(i + 1) % 6]) {
                (true, true) => ConnectionType::CWW,
                (false, true) => ConnectionType::CFW,
                (true, false) => ConnectionType::CWF,
                (false, false) => ConnectionType::CFF,
            } as usize].clone();
            let cos = FixedI32::<P>::ROTATIONS_COS[i];
            let sin = FixedI32::<P>::ROTATIONS_SIN[i];
            corner.rotate(sin, cos);
            hights.bake(offset, &corner).or_else(|e| Err(e.to_string()))?;
            }
        Ok(())
    }
}
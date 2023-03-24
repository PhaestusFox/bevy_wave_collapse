use fixed::{FixedI32, FixedI64};
use fixed::types::extra::{LeEqU64, LeEqU32};
use super::*;
use az::Cast;

const U8_OFFSET: f32 = 1./16.;
const U8_START: f32 = U8_OFFSET/2.;
impl VertexUV for u8 {
    fn to_f32x2(&self) -> [f32;2] {
        [
            U8_START + U8_OFFSET * (self % 16) as f32,
            U8_START + U8_OFFSET * (self / 16) as f32
        ]
    }
}

#[cfg(not(feature="with_bevy"))]
trait FixedVertexPosition: LeEqU32 {}

#[cfg(not(feature="with_bevy"))]
impl<T: LeEqU32> FixedVertexPosition for T  {
    
}

#[cfg(feature="with_bevy")]
trait FixedVertexPosition: 'static + VecComponent + Send + Sync + std::hash::Hash + PartialEq {}
#[cfg(feature="with_bevy")]
impl<T: 'static + VecComponent + Send + Sync + std::hash::Hash + PartialEq> FixedVertexPosition for T  {
    
}

impl<T:'static + LeEqU64 + Send + Sync> VertexPosition for FixedI64<T> {
    fn to_f32(&self) -> f32 {
        self.cast()
    }
    fn from_f32(val: f32) -> Self {
        FixedI64::from_num(val)
    }
}
impl<T:'static + LeEqU32 + Send + Sync> VertexPosition for FixedI32<T> {
    fn to_f32(&self) -> f32 {
        self.cast()
    }
    fn from_f32(val: f32) -> Self {
        FixedI32::from_num(val)
    }
}
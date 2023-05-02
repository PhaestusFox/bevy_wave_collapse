use std::collections::HashMap;

use crate::prelude::VertexUV;

pub trait WavePalate<UV: VertexUV> {
    fn apply(&self, val: &mut UV);
}

impl<UV: VertexUV> WavePalate<UV> for HashMap<UV, UV> {
    fn apply(&self, val: &mut UV) {
        if let Some(uv) = self.get(val) {
            *val = *uv;
        }
    }
}

impl<UV: VertexUV> WavePalate<UV> for &HashMap<UV, UV> {
    fn apply(&self, val: &mut UV) {
        if let Some(uv) = self.get(val) {
            *val = *uv;
        }
    }
}

impl<UV: VertexUV + Into<usize>> WavePalate<UV> for &[UV]  {
    fn apply(&self, val: &mut UV) {
        if let Some(uv) = self.get::<usize>((*val).into()) {
            *val = *uv;
        }
    }
}
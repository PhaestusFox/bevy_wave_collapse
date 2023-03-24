use std::{marker::PhantomData, str::FromStr};

use bevy::asset::{AssetLoader, LoadedAsset};
use crate::vertex::VertexPosition;

use super::WaveMesh;

#[derive(Default)]
pub struct WaveMeshObjLoader<T: VertexPosition>(PhantomData<T>);

impl<T:'static + VertexPosition + Send + Sync + FromStr> AssetLoader for WaveMeshObjLoader<T> {
    fn extensions(&self) -> &[&str] {
        &["wfo"]
    }
    fn load<'a>(
            &'a self,
            bytes: &'a [u8],
            load_context: &'a mut bevy::asset::LoadContext,
        ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let str = String::from_utf8_lossy(bytes);
            for (name, mesh) in WaveMesh::<T, u8>::from_obj_str(&str)? {
                if !name.to_lowercase().starts_with("core") {
                    load_context.set_labeled_asset(&name, LoadedAsset::new(mesh));
                } else {
                    load_context.set_default_asset(LoadedAsset::new(mesh));
                }
            }
            Ok(())
        })
    }
}
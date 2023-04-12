use std::{marker::PhantomData, str::FromStr};

use crate::{vertex::VertexPosition, prelude::VertexUV};
use bevy::asset::{AssetLoader, LoadedAsset};

use super::WaveMesh;

#[derive(Default)]
pub struct WaveMeshObjLoader<P: VertexPosition, UV: VertexUV>(PhantomData<P>, PhantomData<UV>);

impl<T: 'static + VertexPosition + Send + Sync + FromStr, UV: 'static + VertexUV + Send + Sync + FromStr + Default> AssetLoader for WaveMeshObjLoader<T, UV> {
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
            for (name, mesh) in WaveMesh::<T, UV>::from_obj_str(&str)? {
                load_context.set_labeled_asset(&name, LoadedAsset::new(mesh));
            }
            Ok(())
        })
    }
}
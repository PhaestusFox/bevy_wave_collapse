use crate::prelude::*;
use bevy::prelude::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{Hash, Hasher},
};
pub mod hexs;
use crate::errors::BakeError;

#[derive(Clone)]
pub struct Connection {
    name: Cow<'static, str>,
    hash: u64,
}

impl Connection {
    pub fn new<T: Into<Cow<'static, str>> + Hash>(id: T) -> Self {
        let mut hasher = bevy::utils::AHasher::default();
        id.hash(&mut hasher);
        Connection {
            name: id.into(),
            hash: hasher.finish(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::hash::Hash for Connection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl Eq for Connection {}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        if self.hash != other.hash {
            // Makes the common case of two strings not been equal very fast
            return false;
        }
        self.name.eq(&other.name)
    }
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Connection({})", self.name))
    }
}

pub struct WaveObject<P: VertexPosition, UV: VertexUV, Id: Into<u64>, const N: usize> {
    pub meshes: HashMap<Connection, Handle<WaveMesh<P, UV>>>,
    pub build_fn: fn(
        &WaveObject<P, UV, Id, N>,
        RVec3<P>,
        &Assets<WaveMesh<P, UV>>,
        &mut WaveBuilder<P, UV>,
        [&WaveObject<P, UV, Id, N>; N],
        Id,
    ) -> Result<(), BakeError>,
    pub can_connect_fn: fn(Connection) -> bool,
}

impl<P: VertexPosition, UV: VertexUV, Id: Into<u64>, const N: usize> WaveObject<P, UV, Id, N> {
    pub fn build(
        &self,
        offset: RVec3<P>,
        meshs: &Assets<WaveMesh<P, UV>>,
        main_mesh: &mut WaveBuilder<P, UV>,
        neighbours: [&WaveObject<P, UV, Id, N>; N],
        id: Id,
    ) -> Result<(), BakeError> {
        (self.build_fn)(self, offset, meshs, main_mesh, neighbours, id)
    }
    pub fn can_connect(&self, connection: Connection) -> bool {
        (self.can_connect_fn)(connection)
    }
}

impl<P: VertexPosition, UV: VertexUV, Id: Into<u64>, const N: usize> WaveObject<P, UV, Id, N> {
    pub fn get<T: Into<&'static str>>(&self, connection: T) -> Option<&Handle<WaveMesh<P, UV>>>
    where
        Connection: From<T>,
    {
        self.meshes.get(&Connection::from(connection))
    }
}

impl<T: Into<Cow<'static, str>> + Hash> From<T> for Connection {
    fn from(value: T) -> Self {
        Connection::new(value)
    }
}

use crate::prelude::*;
use bevy::prelude::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{Hash, Hasher},
};
pub mod hexs_map;
use crate::errors::BakeError;

#[derive(Clone)]
pub struct Connection {
    name: Cow<'static, str>,
    hash: u64,
}

impl Connection {
    pub fn new<T: Into<Cow<'static, str>> + Hash>(id: T) -> Self {
        let mut hasher = bevy::utils::AHasher::default();
        let name: Cow<'static, str> = id.into();
        name.hash(&mut hasher);
        Connection {
            name,
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
        self.name.eq(other.name.as_ref())
    }
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Connection({})", self.name))
    }
}

pub struct WaveObjects<'a, P: VertexPosition, UV: VertexUV, const N: usize>(pub [&'a WaveObject<P, UV, Self>; N]);

pub struct SeededWaveObjects<'a, P: VertexPosition, UV: VertexUV, const N: usize> {
    pub neighbours: [&'a WaveObject<P, UV, Self>; N],
    pub seed: u64,
}

#[cfg(feature="bevy")]
impl<P: VertexPosition, UV: VertexUV, DATA> bevy::reflect::TypeUuid for WaveObject<P, UV, DATA> {
    const TYPE_UUID: uuid::Uuid = uuid::uuid!("50baca88-21e3-47e8-9a4e-05fe89565e21");
}

pub struct WaveObject<P: VertexPosition, UV: VertexUV, DATA> {
    pub meshes: HashMap<Connection, Handle<WaveMesh<P, UV>>>,
    pub build_fn: fn(
        &WaveObject<P, UV, DATA>,
        RVec3<P>,
        &Assets<WaveMesh<P, UV>>,
        &mut WaveBuilder<P, UV>,
        &DATA,
    ) -> Result<(), BakeError>,
    pub can_connect_fn: fn(Connection) -> bool,
}

impl<P: VertexPosition, UV: VertexUV, DATA> WaveObject<P, UV, DATA> {
    pub fn build(
        &self,
        offset: RVec3<P>,
        meshs: &Assets<WaveMesh<P, UV>>,
        main_mesh: &mut WaveBuilder<P, UV>,
        neighbours: &DATA,
    ) -> Result<(), BakeError> {
        (self.build_fn)(self, offset, meshs, main_mesh, neighbours)
    }
    pub fn can_connect(&self, connection: Connection) -> bool {
        (self.can_connect_fn)(connection)
    }
}

impl<P: VertexPosition, UV: VertexUV, DATA> WaveObject<P, UV, DATA> {
    pub fn get<T: Into<&'static str>>(&self, connection: T) -> Option<&Handle<WaveMesh<P, UV>>>
    where
        Connection: From<T>,
    {
        self.meshes.get(&Connection::from(connection))
    }
}

impl<T: Into<&'static str> + Hash> From<T> for Connection {
    fn from(val: T) -> Self {
        let name: &'static str = val.into();
        let name = Cow::Borrowed(name);
        Connection::new(name)
    }
}

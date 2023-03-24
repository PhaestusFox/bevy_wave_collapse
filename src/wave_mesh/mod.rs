use std::hash::Hash;

use crate::{
    errors::{BakeError, ParseObjError},
    vector::RVec3,
    vertex::{Vertex, VertexPosition, VertexUV},
};

#[cfg(feature = "bevy")]
pub(crate) mod loader;

use bevy::utils::HashMap; //30% faster the std
                          // use std::collections::HashMap;
#[cfg(feature = "bevy")]
use uuid::uuid;

#[derive(Clone)]
pub struct WaveMesh<P: VertexPosition, UV: VertexUV> {
    pub vertexs: Vec<Vertex<P, UV>>,
    pub indices: Vec<u32>,
}

impl<P: VertexPosition, UV: VertexUV> WaveMesh<P, UV> {
    pub fn new() -> WaveMesh<P, UV> {
        WaveMesh {
            vertexs: Vec::new(),
            indices: Vec::new(),
        }
    }
    pub fn rotate(&mut self, sin: P, cos: P) {
        for mut vertex in self.vertexs.iter_mut() {
            let x = vertex.position.x * cos - vertex.position.z * sin;
            let z = vertex.position.z * cos + vertex.position.x * sin;
            vertex.position.x = x;
            vertex.position.z = z;
        }
    }
}

impl<P: VertexPosition + std::str::FromStr> WaveMesh<P, u8> {
    pub fn from_obj_str(str: &str) -> Result<HashMap<String, WaveMesh<P, u8>>, ParseObjError> {
        let mut meshs = HashMap::new();
        let mut current_mesh = WaveMesh::new();
        let mut current_color: u8 = 0;
        let mut current_name = String::new();
        let mut first = true;
        let mut points = Vec::new();
        let mut vertex_map = HashMap::new();
        for (num, line) in str.lines().enumerate() {
            let mut words = line.split(' ');
            match {
                let Some(w) = words.next() else {continue;};
                w
            } {
                "#" | "vt" | "s" | "l" | "mtllib" | "vn" => continue,
                "usemtl" => {
                    current_color = words
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Color",
                            line: num,
                        })?
                        .parse()
                        .or_else(|e| Err(ParseObjError::FailedToParseInt(e, num)))?;
                }
                "f" => {
                    let mut f0 = words
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Face Index 0",
                            line: num,
                        })?
                        .split('/');
                    let f0_0 = f0
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Face Vertex 0",
                            line: num,
                        })?
                        .parse::<usize>()
                        .or_else(|e| Err(ParseObjError::FailedToParseInt(e, num)))?
                        - 1;
                    //let f0_1 = f0.next().ok_or(ParseObjError::NoPoint(1, num))?.parse::<usize>().or_else(|e| Err(ParseObjError::Int(e, num)))?;
                    //let f0_2 = f0.next().ok_or(ParseObjError::NoPoint(2, num))?.parse::<usize>().or_else(|e| Err(ParseObjError::Int(e, num)))?;
                    let mut f1 = words
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Face Index 1",
                            line: num,
                        })?
                        .split('/');
                    let f1_0 = f1
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Face Vertex 1",
                            line: num,
                        })?
                        .parse::<usize>()
                        .or_else(|e| Err(ParseObjError::FailedToParseInt(e, num)))?
                        - 1;
                    //let f1_1 = f1.next().ok_or(ParseObjError::NoPoint(1, num))?.parse::<usize>().or_else(|e| Err(ParseObjError::Int(e, num)))?;
                    //let f1_2 = f1.next().ok_or(ParseObjError::NoPoint(2, num))?.parse::<usize>().or_else(|e| Err(ParseObjError::Int(e, num)))?;

                    let mut f2 = words
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Face Index 2",
                            line: num,
                        })?
                        .split('/');
                    let f2_0 = f2
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Face Vertex 2",
                            line: num,
                        })?
                        .parse::<usize>()
                        .or_else(|e| Err(ParseObjError::FailedToParseInt(e, num)))?
                        - 1;
                    //let f2_1 = f2.next().ok_or(ParseObjError::NoPoint(1, num))?.parse::<usize>().or_else(|e| Err(ParseObjError::Int(e, num)))?;
                    //let f2_2 = f2.next().ok_or(ParseObjError::NoPoint(2, num))?.parse::<usize>().or_else(|e| Err(ParseObjError::Int(e, num)))?;
                    let vertex = Vertex {
                        position: points[f0_0],
                        uv: current_color,
                    };
                    let id = *vertex_map.entry(vertex).or_insert_with(|| {
                        let id = current_mesh.vertexs.len() as u32;
                        current_mesh.vertexs.push(vertex);
                        id
                    });
                    current_mesh.indices.push(id);
                    let vertex = Vertex {
                        position: points[f1_0],
                        uv: current_color,
                    };
                    let id = *vertex_map.entry(vertex).or_insert_with(|| {
                        let id = current_mesh.vertexs.len() as u32;
                        current_mesh.vertexs.push(vertex);
                        id
                    });
                    current_mesh.indices.push(id);
                    let vertex = Vertex {
                        position: points[f2_0],
                        uv: current_color,
                    };
                    let id = *vertex_map.entry(vertex).or_insert_with(|| {
                        let id = current_mesh.vertexs.len() as u32;
                        current_mesh.vertexs.push(vertex);
                        id
                    });
                    current_mesh.indices.push(id);
                }
                "v" => {
                    let x = words
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Vertex x",
                            line: num,
                        })?
                        .parse::<P>()
                        .or_else(|_| Err(ParseObjError::FailedToParse("Vertex x", num)))?;
                    let y = words
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Vertex y",
                            line: num,
                        })?
                        .parse::<P>()
                        .or_else(|_| Err(ParseObjError::FailedToParse("Vertex y", num)))?;
                    let z = words
                        .next()
                        .ok_or(ParseObjError::ExpectedSymbol {
                            expect: "Vertex z",
                            line: num,
                        })?
                        .parse::<P>()
                        .or_else(|_| Err(ParseObjError::FailedToParse("Vertex z", num)))?;
                    points.push(RVec3::new(x, y, z));
                }
                "o" => {
                    if !first {
                        meshs.insert(current_name, current_mesh);
                    } else {
                        first = false;
                    }
                    current_mesh = WaveMesh::new();
                    current_name = words.next().ok_or(ParseObjError::NoName(num))?.to_string();
                    vertex_map.clear();
                }
                w => {
                    return Err(ParseObjError::UnknownSymbol(w.to_string(), num));
                }
            }
        }
        if !first {
            meshs.insert(current_name, current_mesh);
        }
        if meshs.len() == 0 {
            Err(ParseObjError::NoMeshs)
        } else {
            Ok(meshs)
        }
    }
}

#[cfg(feature = "bevy")]
impl<T: VertexPosition, UV: VertexUV> bevy::reflect::TypeUuid for WaveMesh<T, UV> {
    const TYPE_UUID: bevy::utils::Uuid = uuid!("c222c5a0-c488-4642-923d-d9b6eda4b7d3");
}

pub struct WaveBuilder<P: VertexPosition, UV: VertexUV> {
    vertexs: Vec<Vertex<P, UV>>,
    indices: Vec<u32>,
    map: HashMap<Vertex<P, UV>, u32>,
}

impl<P: VertexPosition, UV: VertexUV + Hash> WaveBuilder<P, UV> {
    /// Add a wavemesh into the main mesh
    /// This will not combine duplicate vertex nor will it add them to the map
    /// Use this to add compleate structeres to a mesh such as rocks or trees
    pub fn add(&mut self, offset: RVec3<P>, mesh: &WaveMesh<P, UV>) -> Result<(), BakeError> {
        let indices_offset = self.vertexs.len() as u32;
        self.vertexs.extend(
            mesh.vertexs
                .iter()
                .map(|v| Vertex::new(v.position + offset, v.uv)),
        );
        self.indices
            .extend(mesh.indices.iter().map(|i| *i + indices_offset));
        Ok(())
    }

    /// Bake a wavemesh into the main mesh combining any duplicate vertexes along the way
    /// Use this to add partuals structure or connections to the mesh such as walls or cells
    pub fn bake(&mut self, offset: RVec3<P>, mesh: &WaveMesh<P, UV>) -> Result<(), BakeError> {
        let mut vertexs = Vec::with_capacity(mesh.vertexs.len());
        vertexs.extend(mesh.vertexs.iter().map(|vertex| {
            let point = vertex.position + offset;
            let vertex = Vertex::new(point, vertex.uv);
            *self.map.entry(vertex).or_insert_with(|| {
                let id = self.vertexs.len() as u32;
                self.vertexs.push(vertex);
                id
            })
        }));
        self.indices
            .extend(mesh.indices.iter().map(|i| vertexs[*i as usize]));
        Ok(())
    }

    pub fn new() -> WaveBuilder<P, UV> {
        WaveBuilder {
            vertexs: Vec::new(),
            indices: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.vertexs.clear();
        self.map.clear();
    }

    pub fn extract(&self) -> (Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>) {
        let mut vertexs = Vec::with_capacity(self.vertexs.len());
        let mut uvs = Vec::with_capacity(self.vertexs.len());
        for Vertex { position, uv } in self.vertexs.iter() {
            vertexs.push(position.to_f32x3());
            uvs.push(uv.to_f32x2());
        }
        (vertexs, uvs, self.indices.clone())
    }

    #[cfg(feature = "with_bevy")]
    pub fn extract_mesh(
        &self,
        topology: bevy::render::render_resource::PrimitiveTopology,
    ) -> bevy::prelude::Mesh {
        use bevy::prelude::Mesh;
        let mut mesh = Mesh::new(topology);
        let (vertexs, uvs, indices) = self.extract();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertexs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
        mesh
    }

    pub fn vertex_len(&self) -> usize {
        self.vertexs.len()
    }

    pub fn indices_len(&self) -> usize {
        self.indices.len()
    }
}

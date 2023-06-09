use std::fmt::Debug;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BakeError {
    #[error("Mesh {mesh} for {obj} not found in Assets")]
    MeshNotFound{mesh: &'static str, obj: &'static str },
    #[error("Mesh {mesh} not set for {obj}")]
    MeshNotSet{mesh: &'static str, obj: &'static str },
}

#[derive(Debug, Error)]
pub enum ParseObjError {
    #[error("The File Contained No Meshes")]
    NoMeshs,
    #[error("No Name Followed the O in on line {0}")]
    NoName(usize),
    #[error("Found Unknown Simble {0} on line {1}")]
    UnknownSymbol(String, usize),
    #[error("Expected {expect} Symbol on line {line}")]
    ExpectedSymbol { expect: &'static str, line: usize },
    #[error("Failed to Parse Int on line {1}")]
    FailedToParseInt(core::num::ParseIntError, usize),
    #[error("Failed to Parse Float on line {1}")]
    FailedToParseFloat(core::num::ParseFloatError, usize),
    #[error("Failed to Parse {0} on line {1}")]
    FailedToParse(&'static str, usize),
}

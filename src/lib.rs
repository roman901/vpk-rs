pub mod entry;
pub mod structs;
pub mod vpk;


pub use crate::vpk::VPK;

use thiserror::Error;
use std::path::Path;
use std::string::FromUtf8Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error while trying to read data: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Error while trying to read data: {0}")]
    BinReadError(#[from] binread::Error),
    #[error("Invalid signature, provided file is not a VPK file")]
    InvalidSignature,
    #[error("Unsupported VPK version({0}), only version 2 and low")]
    UnsupportedVersion(u32),
    #[error("Mismatched size for hashes section")]
    HashSizeMismatch,
    #[error("Malformed index encountered while parsing")]
    MalformedIndex,
    #[error("Invalid utf8 string found while parsing")]
    Utf(#[from] FromUtf8Error)
}

pub fn from_path(path: &str) -> Result<VPK, Error> {
    let path = Path::new(path);
    let vpk = VPK::read(&path)?;

    Ok(vpk)
}

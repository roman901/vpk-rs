pub mod entry;
pub mod structs;
pub mod vpk;

use thiserror::Error;

pub use crate::vpk::VPK;

use std::path::Path;
use std::str::Utf8Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Error while trying to read data: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Error while trying to read data: {0}")]
    BinReadError(#[from] binrw::Error),
    #[error("Invalid signature, provided file is not a VPK file")]
    InvalidSignature,
    #[error("Unsupported VPK version({0}), only version 2 and low")]
    UnsupportedVersion(u32),
    #[error("Mismatched size for hashes section")]
    HashSizeMismatch,
    #[error("Malformed index encountered while parsing")]
    MalformedIndex,
    #[error("Invalid utf8 string found while parsing")]
    Utf(#[from] Utf8Error),
    #[error("Filename not available")]
    FilenameNotAvailable,
    #[error("Filename not representable as str")]
    FilenameNotRepresentableAsStr,
}

/// Read the [`VPK`] the given [`Path`].
#[doc(alias = "read")]
pub fn from_path(path: impl AsRef<Path>) -> Result<VPK, Error> {
    VPK::read(path)
}

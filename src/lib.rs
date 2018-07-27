extern crate byteorder;

mod vpk;
mod header;
mod index;

use vpk::VPK;

use std::io::Error;
use std::path::Path;
use std::fs::File;

pub fn from_path(path: &str) -> Result<VPK, Error> {
    let path = Path::new(path);
    let f = File::open(&path)?;

    from_file(f)
}

pub fn from_file(file: File) -> Result<VPK, Error> {
    let vpk = VPK::read(file)?;
    Ok(vpk)
}
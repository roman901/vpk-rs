mod vpk;
mod structs;
mod entry;

use vpk::VPK;

use std::io::Error;
use std::path::Path;

pub fn from_path(path: &str) -> Result<VPK, Error> {
    let path = Path::new(path);
    let vpk = VPK::read(&path)?;

    Ok(vpk)
}

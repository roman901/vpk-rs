extern crate byteorder;
extern crate core;

mod reader;
mod header;
mod bundle;

use bundle::VPKBundle;
use reader::VPKReader;
use std::io::Error;

use std::path::Path;
use std::fs::File;

const VPK_SIGNATURE: u32 = 0x55aa1234;

pub fn open(vpk_file: &String) -> Result<VPKBundle, Error> {
    let p = Path::new(vpk_file);
    let f = File::open(&p)?;

    // Create VPKReader and read header
    let mut r = VPKReader::new(&f);
    let header = match r.read_header() {
        Ok(header) => header,
        Err(e) => panic!("Error while reading header: {}", e)
    };

    if header.signature != VPK_SIGNATURE {
        panic!("Specified file is not vpk dir file!");
    }

    Ok(VPKBundle::new(header))
}

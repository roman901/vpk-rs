extern crate byteorder;
extern crate core;

mod reader;
mod header;

use reader::VPKReader;
use header::VPKHeader;

use std::path::Path;
use std::fs::File;
use std::mem;
use std::slice;
use std::io::{Read, Error};
use std::io::BufReader;

const VPK_SIGNATURE: u32 = 0x55aa1234;

#[derive(Debug)]
pub struct VPKBundle {
    header: VPKHeader
}

pub fn open(vpk_file: &String) -> Result<VPKBundle, Error> {
    let p = Path::new(vpk_file);
    let f = File::open(&p)?;

    // Create VPKReader and read header
    let r = VPKReader::new(f);
    let header = match r.read_header() {
        Ok(header) => header,
        Err(e) => panic!("Error while reading header: {}", e)
    };

    if header.signature != VPK_SIGNATURE {
        panic!("Specified file is not vpk dir file!");
    }

    Ok(VPKBundle {header})
}

mod header;

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

    // Read header of file
    let header = read_header(f);
    if header.signature != VPK_SIGNATURE {
        panic!("Specified file is not vpk dir file!");
    }

    Ok(VPKBundle {header})
}

fn read_header(f: File) -> VPKHeader {
    let mut header: VPKHeader = unsafe { mem::uninitialized() };
    let mut reader = BufReader::new(f);

    unsafe {
        let dst_ptr = &mut header as *mut VPKHeader as *mut u8;
        let mut slice = slice::from_raw_parts_mut(dst_ptr, mem::size_of::<VPKHeader>());

        reader.read_exact(slice);
    }
    header
}
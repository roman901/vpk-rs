use header::VPKHeader;

use std::fs::File;
use std::mem;
use byteorder::{ReadBytesExt, LittleEndian};
use std::io::{BufReader, Read};
use std::slice;
use std::io::Error;

pub struct VPKReader {
    file: File,
    pos: u32
}

impl VPKReader {
    pub fn new(file: File) -> VPKReader {
        VPKReader { file, pos: 0 }
    }

    pub fn read_header(&self) -> Result<VPKHeader, Error> {
        let mut header: VPKHeader = unsafe { mem::uninitialized() };
        let mut reader = BufReader::new(&self.file);

        unsafe {
            let dst_ptr = &mut header as *mut VPKHeader as *mut u8;
            let slice = slice::from_raw_parts_mut(dst_ptr, mem::size_of::<VPKHeader>());

            reader.read_exact(slice)?;
        }
        Ok(header)
    }
}

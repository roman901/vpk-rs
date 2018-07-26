use header::VPKHeader;

use std::fs::File;
use std::mem;
use std::io::{BufReader, Read};
use std::slice;
use std::io::Error;

pub struct VPKReader {
    reader: BufReader<File>,
    pos: u32
}

impl VPKReader {
    pub fn new(file: File) -> VPKReader {
        let reader = BufReader::new(file);

        VPKReader { reader, pos: 0 }
    }

    pub fn read_header(&mut self) -> Result<VPKHeader, Error> {
        let mut header: VPKHeader = unsafe { mem::uninitialized() };

        unsafe {
            let dst_ptr = &mut header as *mut VPKHeader as *mut u8;
            let slice = slice::from_raw_parts_mut(dst_ptr, mem::size_of::<VPKHeader>());

            self.reader.read_exact(slice)?;
        }

        Ok(header)
    }

    fn read_ctring(&self) -> String {
        "test".parse().unwrap()
    }
}

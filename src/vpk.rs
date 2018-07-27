use header::{VPKHeader, VPKHeaderV2, VPKHeaderV2Checksum};
use index::VPKIndex;

use std::fs::File;
use std::mem;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::slice;
use std::io::Error;

const VPK_SIGNATURE: u32 = 0x55aa1234;
const VPK_SELF_HASHES_LENGTH: u32 = 48;

#[derive(Debug)]
pub struct VPK {
    header: VPKHeader,
    header_v2: Option<VPKHeaderV2>,
    header_v2_checksum: Option<VPKHeaderV2Checksum>
}

impl VPK {
    pub fn read(file: File) -> Result<VPK, Error> {
        let mut reader = BufReader::new(file);

        // Read main VPK header
        let mut header: VPKHeader = unsafe { mem::uninitialized() };
        unsafe {
            let dst_ptr = &mut header as *mut VPKHeader as *mut u8;
            let slice = slice::from_raw_parts_mut(dst_ptr, mem::size_of::<VPKHeader>());

            reader.read_exact(slice)?;
        }

        assert_eq!(header.signature, VPK_SIGNATURE, "Specified file is not VPK _dir file");
        assert!(header.version <= 2, "Unsupported version of VPK bundle");

        if header.version == 2 {
            let mut header_v2: VPKHeaderV2 = unsafe { mem::uninitialized() };
            unsafe {
                let dst_ptr = &mut header_v2 as *mut VPKHeaderV2 as *mut u8;
                let slice = slice::from_raw_parts_mut(dst_ptr, mem::size_of::<VPKHeaderV2>());

                reader.read_exact(slice)?;
            }

            assert_eq!(header_v2.self_hashes_length, VPK_SELF_HASHES_LENGTH, "Self hashes section size mismatch");

            return Ok(VPK {
                header,
                header_v2: Some(header_v2),
                header_v2_checksum: None
            })
        }

        Ok(VPK {
            header,
            header_v2: None,
            header_v2_checksum: None
        })
    }

    fn read_cstring(&self) -> String {
        "test".parse().unwrap()
    }
}

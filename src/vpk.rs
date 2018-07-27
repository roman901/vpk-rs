use header::{VPKHeader, VPKHeaderV2, VPKHeaderV2Checksum, VPKDirectoryEntry};

use std::fs::File;
use std::mem;
use std::io::{BufReader, Read, Seek, SeekFrom, Error};
use std::slice;
use std::collections::HashMap;

const VPK_SIGNATURE: u32 = 0x55aa1234;
const VPK_SELF_HASHES_LENGTH: u32 = 48;

#[derive(Debug)]
pub struct VPK {
    header: VPKHeader,
    header_v2: Option<VPKHeaderV2>,
    header_v2_checksum: Option<VPKHeaderV2Checksum>,
    tree: HashMap<String, VPKDirectoryEntry>
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

        let mut vpk = VPK {
            header,
            header_v2: None,
            header_v2_checksum: None,
            tree: HashMap::new()
        };

        if vpk.header.version == 2 {
            let mut header_v2: VPKHeaderV2 = unsafe { mem::uninitialized() };
            unsafe {
                let dst_ptr = &mut header_v2 as *mut VPKHeaderV2 as *mut u8;
                let slice = slice::from_raw_parts_mut(dst_ptr, mem::size_of::<VPKHeaderV2>());

                reader.read_exact(slice)?;
            }

            assert_eq!(header_v2.self_hashes_length, VPK_SELF_HASHES_LENGTH, "Self hashes section size mismatch");

            let checksum_offset: u32 = vpk.header.tree_length + header_v2.embed_chunk_length + header_v2.chunk_hashes_length;
            reader.seek(SeekFrom::Current(checksum_offset as i64))?;

            let mut header_v2_checksum: VPKHeaderV2Checksum = unsafe { mem::uninitialized() };
            unsafe {
                let dst_ptr = &mut header_v2_checksum as *mut VPKHeaderV2Checksum as *mut u8;
                let slice = slice::from_raw_parts_mut(dst_ptr, mem::size_of::<VPKHeaderV2Checksum>());

                reader.read_exact(slice)?;
            }

            vpk.header_v2 = Some(header_v2);
            vpk.header_v2_checksum = Some(header_v2_checksum);

            // Return seek to initial position - after header
            let header_length = mem::size_of::<VPKHeader>() + mem::size_of::<VPKHeaderV2>();
            reader.seek(SeekFrom::Start(header_length as u64))?;
        }

        // Read index tree
        let mut ext: String = String::new();
        loop {
            ext = read_cstring(&mut reader)?;
            if ext == "" {
                break;
            }

            println!("{}", ext);
        }


        Ok(vpk)
    }
}

fn read_cstring(reader: &mut BufReader<File>) -> Result<String, Error> {
    let mut string: String = String::new();

    let mut buf = [0u8];
    loop {
        reader.by_ref().read_exact(&mut buf)?;

        if buf[0] == 0 {
            return Ok(string);
        } else {
            string.push(buf[0] as char);
        }
    }
}
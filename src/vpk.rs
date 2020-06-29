use crate::entry::*;
use crate::structs::*;
use crate::Error;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem;
use std::path::Path;
use binread::BinReaderExt;

const VPK_SIGNATURE: u32 = 0x55aa1234;
const VPK_SELF_HASHES_LENGTH: u32 = 48;

#[derive(Debug)]
pub struct VPK {
    pub header_length: u32,
    pub header: VPKHeader,
    pub header_v2: Option<VPKHeaderV2>,
    pub header_v2_checksum: Option<VPKHeaderV2Checksum>,
    pub tree: HashMap<String, VPKEntry>,
}

impl VPK {
    pub fn read(dir_path: &Path) -> Result<VPK, Error> {
        let file = File::open(dir_path)?;

        let mut reader = BufReader::new(file);

        // Read main VPK header
        let header: VPKHeader = reader.read_le()?;

        if header.signature != VPK_SIGNATURE {
            return Err(Error::InvalidSignature);
        }
        if header.version > 2 {
            return Err(Error::UnsupportedVersion(header.version));
        }

        let mut vpk = VPK {
            header_length: 4 * 3,
            header,
            header_v2: None,
            header_v2_checksum: None,
            tree: HashMap::new(),
        };

        if vpk.header.version == 2 {
            let header_v2: VPKHeaderV2 = reader.read_le()?;

            if header_v2.self_hashes_length != VPK_SELF_HASHES_LENGTH {
                return Err(Error::HashSizeMismatch);
            }
            vpk.header_length += 4 * 4;

            let checksum_offset: u32 = vpk.header.tree_length
                + header_v2.embed_chunk_length
                + header_v2.chunk_hashes_length;
            reader.seek(SeekFrom::Current(checksum_offset as i64))?;

            let header_v2_checksum: VPKHeaderV2Checksum = reader.read_le()?;

            vpk.header_v2 = Some(header_v2);
            vpk.header_v2_checksum = Some(header_v2_checksum);

            // Return seek to initial position - after header
            let header_length = mem::size_of::<VPKHeader>() + mem::size_of::<VPKHeaderV2>();
            reader.seek(SeekFrom::Start(header_length as u64))?;
        }

        // Read index tree
        loop {
            let ext = read_cstring(&mut reader)?;
            if ext == "" {
                break;
            }

            loop {
                let mut path = read_cstring(&mut reader)?;
                if path == "" {
                    break;
                }
                if path != " " {
                    path += "/";
                } else {
                    path = "".to_owned();
                }

                loop {
                    let name = read_cstring(&mut reader)?;
                    if name == "" {
                        break;
                    }

                    let mut dir_entry: VPKDirectoryEntry = reader.read_le()?;

                    if dir_entry.suffix != 0xffff {
                        return Err(Error::MalformedIndex);
                    }

                    if dir_entry.archive_index == 0x7fff {
                        dir_entry.archive_offset =
                            vpk.header_length + vpk.header.tree_length + dir_entry.archive_offset;
                    }

                    let preload_length = dir_entry.preload_length;
                    let _dir_path = dir_path.to_str().unwrap();
                    let archive_path =
                        _dir_path.replace("dir.", &format!("{:03}.", dir_entry.archive_index));
                    let mut vpk_entry = VPKEntry {
                        dir_entry,
                        archive_path,
                        preload_data: vec![0u8; preload_length as usize],
                    };

                    reader
                        .by_ref()
                        .take(vpk_entry.dir_entry.preload_length as u64)
                        .read_exact(&mut vpk_entry.preload_data)?;

                    vpk.tree
                        .insert(format!("{}{}.{}", path, name, ext), vpk_entry);
                }
            }
        }

        Ok(vpk)
    }
}

fn read_cstring(reader: &mut BufReader<File>) -> Result<String, Error> {
    let mut string: String = String::new();

    let mut buf = [0u8];
    loop {
        reader.by_ref().read_exact(&mut buf)?;
        //println!("{:?}", buf[0]);
        if buf[0] == 0 {
            break;
        } else {
            string.push(buf[0] as char);
        }
    }

    return Ok(string);
}

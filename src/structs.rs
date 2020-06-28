use std::io::{self, Read};
use byteorder::{ReadBytesExt, LittleEndian};

#[derive(Debug)]
pub struct VPKHeader {
    pub signature: u32,
    pub version: u32,
    pub tree_length: u32,
}

impl VPKHeader {
    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        Ok(VPKHeader {
            signature: reader.read_u32::<LittleEndian>()?,
            version: reader.read_u32::<LittleEndian>()?,
            tree_length: reader.read_u32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
pub struct VPKHeaderV2 {
    pub embed_chunk_length: u32,
    pub chunk_hashes_length: u32,
    pub self_hashes_length: u32,
    pub signature_length: u32,
}

impl VPKHeaderV2 {
    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        Ok(VPKHeaderV2 {
            embed_chunk_length: reader.read_u32::<LittleEndian>()?,
            chunk_hashes_length: reader.read_u32::<LittleEndian>()?,
            self_hashes_length: reader.read_u32::<LittleEndian>()?,
            signature_length: reader.read_u32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
pub struct VPKHeaderV2Checksum {
    pub tree_checksum: u128,
    pub chunk_hashes_checksum: u128,
    pub file_checksum: u128,
}

impl VPKHeaderV2Checksum {
    pub fn read(reader: &mut impl Read) -> io::Result<Self> {
        Ok(VPKHeaderV2Checksum {
            tree_checksum: reader.read_u128::<LittleEndian>()?,
            chunk_hashes_checksum: reader.read_u128::<LittleEndian>()?,
            file_checksum: reader.read_u128::<LittleEndian>()?,
        })
    }
}
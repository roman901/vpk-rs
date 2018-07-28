#[derive(Debug)]
pub struct VPKHeader {
    pub signature: u32,
    pub version: u32,
    pub tree_length: u32
}

#[derive(Debug)]
pub struct VPKHeaderV2 {
    pub embed_chunk_length: u32,
    pub chunk_hashes_length: u32,
    pub self_hashes_length: u32,
    pub signature_length: u32
}

#[derive(Debug)]
pub struct VPKHeaderV2Checksum {
    pub tree_checksum: u128,
    pub chunk_hashes_checksum: u128,
    pub file_checksum: u128
}

#[derive(Debug)]
pub struct VPKEntry {
    pub dir_entry: VPKDirectoryEntry,
    pub preload_data: Vec<u8>
}

#[derive(Debug)]
#[repr(packed)]
pub struct VPKDirectoryEntry {
    pub crc32: u32,
    pub preload_length: u16,
    pub archive_index: u16,
    pub archive_offset: u32,
    pub file_length: u32,
    pub suffix: u16
}
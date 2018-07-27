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
    tree_checksum: u16,
    chunk_hashes_checksum: u16,
    file_checksum: u16
}

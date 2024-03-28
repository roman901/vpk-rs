use binread::BinRead;

#[derive(Debug, BinRead)]
pub struct VPKHeader {
    pub signature: u32,
    pub version: u32,
    pub tree_length: u32,
}

#[derive(Debug, BinRead)]
pub struct VPKHeaderV2 {
    pub embed_chunk_length: u32,
    pub chunk_hashes_length: u32,
    pub self_hashes_length: u32,
    pub signature_length: u32,
}

#[derive(Debug, BinRead)]
pub struct VPKHeaderV2Checksum {
    pub tree_checksum: u128,
    pub chunk_hashes_checksum: u128,
    pub file_checksum: u128,
}

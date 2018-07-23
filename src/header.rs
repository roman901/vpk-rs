#[derive(Debug)]
pub struct VPKHeader {
    pub signature: u32,
    pub version: u32,

    pub tree_size: u32,
    pub file_data_section_size: u32,
    pub archive_md5_section_size: u32,
    pub other_md5_section_size: u32,
    pub signature_section_size: u32
}

impl VPKHeader {
    pub fn new() {

    }
}
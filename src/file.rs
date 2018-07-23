#[derive(Debug)]
pub struct VPKFile {
    name: String,
    size: u32
}

// Native Valve struct
#[derive(Debug)]
pub struct VPKDirectoryEntry {
    crc: u32,
    preload_bytes: u16,
    archive_index: u16,
    entry_offset: u32,
    entry_length: u32,
    terminator: u16
}
use std::fs::File;

struct VPKReader {
    file: File,
    pos: u32
}

impl VPKReader {
    pub fn new(file: File) -> VPKReader {
        VPKReader { file, pos: 0 }
    }
}

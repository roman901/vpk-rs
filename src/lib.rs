use std::path::Path;
use std::fs::File;

struct VPKHeader {
    signature: u32,
    version: u32
}

struct VPKFile {

}

fn open(vpk_file: &str) -> VPKFile {
    let path = Path::new(vpk_file);
    let file = File::open(path);

    VPKFile {}
}

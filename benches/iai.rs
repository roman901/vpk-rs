use iai::black_box;
use vpk::VPK;

fn parse_dir() {
    VPK::read(black_box("tf2_misc_dir.vpk".as_ref())).unwrap();
}

iai::main!(parse_dir);

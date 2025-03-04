use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vpk::VPK;

fn parse_vpk(c: &mut Criterion) {
    c.bench_function("parse vpk", |b| {
        b.iter(|| VPK::read(black_box("tf2_misc_dir.vpk")).unwrap())
    });
}

criterion_group!(benches, parse_vpk);
criterion_main!(benches);

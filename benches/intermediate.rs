use criterion::{black_box, criterion_group, criterion_main, Criterion};
use move_gen_analysis::bitboard::Bitboard;

pub fn bench_ilog2(c: &mut Criterion) {
    let bb = Bitboard::D5;
    c.bench_function("ilog2_bench", |b| b.iter(|| black_box(bb.index())));
}

criterion_group!(benches, bench_ilog2);
criterion_main!(benches);

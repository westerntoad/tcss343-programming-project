use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use move_gen_analysis::bitboard::Bitboard;
use move_gen_analysis::square::Square;
use move_gen_analysis::move_gen::lookup::{
    n_gen_shift,
    n_gen_match,
    n_gen_lookup
};

pub fn full_bench_knights(c: &mut Criterion) {
    let mut group = c.benchmark_group("Knight");
    let iter = (0..u64::MAX).map(|n| 2u64 << n);
    
    for i in iter {
        let bb = Bitboard::new(i);

        group.bench_with_input(BenchmarkId::new("Calculation", i), &i, 
            |b, _i| b.iter(|| n_gen_shift(bb)));
        group.bench_with_input(BenchmarkId::new("Lookup", i), &i, 
            |b, _i| b.iter(|| n_gen_lookup(bb)));
    }

    group.finish()
}

pub fn bench_knights(c: &mut Criterion) {
    let mut group = c.benchmark_group("Knight");

    let mut bb = Bitboard::A1;
    let mut category = "Corner";
    group.bench_with_input(BenchmarkId::new("Calculation", category), &bb,
        |b, i| b.iter(|| n_gen_shift(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup (Match)", category), &bb,
        |b, i| b.iter(|| n_gen_match(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup (Array)", category), &bb,
        |b, i| b.iter(|| n_gen_lookup(*i)));

    bb = Bitboard::H4;
    category = "Edge";
    group.bench_with_input(BenchmarkId::new("Calculation", category), &bb,
        |b, i| b.iter(|| n_gen_shift(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup (Match)", category), &bb,
        |b, i| b.iter(|| n_gen_match(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup (Array)", category), &bb,
        |b, i| b.iter(|| n_gen_lookup(*i)));

    bb = Bitboard::D5;
    category = "Center";
    group.bench_with_input(BenchmarkId::new("Calculation", category), &bb,
        |b, i| b.iter(|| n_gen_shift(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup (Match)", category), &bb,
        |b, i| b.iter(|| n_gen_match(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup (Array)", category), &bb,
        |b, i| b.iter(|| n_gen_lookup(*i)));
    
    group.finish()
}

criterion_group!(benches, bench_knights);
criterion_main!(benches); 


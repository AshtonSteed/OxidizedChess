use criterion::{black_box, criterion_group, criterion_main, Criterion};



pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rook move slicing", |b| b.iter(|| get_rook_attacks(black_box(35), black_box(8796229337088))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
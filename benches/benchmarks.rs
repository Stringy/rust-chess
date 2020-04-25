use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chess::bitboard::Bitboard;

fn bench_bitboard_iter_lsb(c: &mut Criterion) {
   c.bench_function("u64 MAX iter LSB", |b| {
      b.iter(|| {
         Bitboard(black_box(std::u64::MAX)).iter().count();
      })
   });
}

fn bench_bitboard_iter_msb(c: &mut Criterion) {
   c.bench_function("u64 MAX iter LSB", |b| {
      b.iter(|| {
         Bitboard(black_box(std::u64::MAX)).iter().rev().count();
      })
   });
}

criterion_group!(
   benches,
   bench_bitboard_iter_lsb,
   bench_bitboard_iter_msb
);

criterion_main!(benches);
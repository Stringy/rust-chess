use chess::bitboard::Bitboard;
use chess::board::{Board, Layout};
use chess::generate::MoveGenerator;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

fn bench_white_pawn_move_generation(c: &mut Criterion) {
    c.bench_function("white pawn moves", |b| {
        b.iter(|| {
            let board = Board::new(Layout::Classic);
            let mg = MoveGenerator::new(&board);
            mg.white_pawns().iter().count();
        })
    });
}

fn bench_all_moves_generation(c: &mut Criterion) {
    c.bench_function("all moves", |b| {
        b.iter(|| {
            let board = Board::new(Layout::Classic);
            let mg = MoveGenerator::new(&board);
            mg.all().iter().count();
        });
    });
}

criterion_group!(
    benches,
    bench_bitboard_iter_lsb,
    bench_bitboard_iter_msb,
    bench_white_pawn_move_generation,
    bench_all_moves_generation
);

criterion_main!(benches);

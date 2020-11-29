use criterion::{black_box, criterion_group, criterion_main, Criterion};

use randomness::prelude::*;

fn bench_random_init_with_no_seed(c: &mut Criterion) {
    c.bench_function("random_init_with_no_seed", |b| b.iter(|| new_random()));
}

fn bench_random_init_with_seed(c: &mut Criterion) {
    c.bench_function("random_init_with_seed", |b| b.iter(|| new_random_from_seed(black_box(0))));
}

fn bench_random_next_functions(c: &mut Criterion) {
    let mut rnd = new_random_from_seed(black_box(0));
    
    c.bench_function("random_next_bool", |b| b.iter(|| rnd.next_bool()));
    c.bench_function("random_next_u8", |b| b.iter(|| rnd.next_u8()));
    c.bench_function("random_next_i8", |b| b.iter(|| rnd.next_i8()));
    c.bench_function("random_next_u16", |b| b.iter(|| rnd.next_u16()));
    c.bench_function("random_next_i16", |b| b.iter(|| rnd.next_i16()));
    c.bench_function("random_next_u32", |b| b.iter(|| rnd.next_u32()));
    c.bench_function("random_next_i32", |b| b.iter(|| rnd.next_i32()));
    c.bench_function("random_next_u64", |b| b.iter(|| rnd.next_u64()));
    c.bench_function("random_next_i64", |b| b.iter(|| rnd.next_i64()));
    c.bench_function("random_next_u128", |b| b.iter(|| rnd.next_u128()));
    c.bench_function("random_next_i128", |b| b.iter(|| rnd.next_i128()));
    c.bench_function("random_next_f32", |b| b.iter(|| rnd.next_f32()));
    c.bench_function("random_next_f64", |b| b.iter(|| rnd.next_f64()));
}

criterion_group!(init_random_benches, bench_random_init_with_no_seed, bench_random_init_with_seed);
criterion_group!(generate_random_benches, bench_random_next_functions);

criterion_main!(init_random_benches, generate_random_benches);
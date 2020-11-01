use criterion::{black_box, criterion_group, criterion_main, Criterion};

use random_numbers;
use random_numbers::RandomNumber;

fn bench_init_with_no_seed(c: &mut Criterion) {
    c.bench_function("lib_default_init_with_no_seed", |b| b.iter(|| random_numbers::new_default()));
}

fn bench_init_with_seed(c: &mut Criterion) {
    c.bench_function("lib_default_init_with_seed", |b| b.iter(|| random_numbers::from_seed_default(black_box(0))));
}

fn bench_get_randoms(c: &mut Criterion) {
    let mut rnd = random_numbers::from_seed_default(black_box(0));
    
    c.bench_function("lib_default_get_random_bools", |b| b.iter(|| rnd.next_bool()));
    c.bench_function("lib_default_get_random_u8s", |b| b.iter(|| rnd.next_u8()));
    c.bench_function("lib_default_get_random_i8s", |b| b.iter(|| rnd.next_i8()));
    c.bench_function("lib_default_get_random_u16s", |b| b.iter(|| rnd.next_u16()));
    c.bench_function("lib_default_get_random_i16s", |b| b.iter(|| rnd.next_i16()));
    c.bench_function("lib_default_get_random_u32s", |b| b.iter(|| rnd.next_u32()));
    c.bench_function("lib_default_get_random_i32s", |b| b.iter(|| rnd.next_i32()));
    c.bench_function("lib_default_get_random_u64s", |b| b.iter(|| rnd.next_u64()));
    c.bench_function("lib_default_get_random_i64s", |b| b.iter(|| rnd.next_i64()));
    c.bench_function("lib_default_get_random_u128s", |b| b.iter(|| rnd.next_u128()));
    c.bench_function("lib_default_get_random_i128s", |b| b.iter(|| rnd.next_i128()));
    c.bench_function("lib_default_get_random_f32s", |b| b.iter(|| rnd.next_f32()));
    c.bench_function("lib_default_get_random_f64s", |b| b.iter(|| rnd.next_f64()));
}

criterion_group!(init_random_number_benches, bench_init_with_no_seed, bench_init_with_seed);
criterion_group!(generate_random_number_benches, bench_get_randoms);

criterion_main!(init_random_number_benches, generate_random_number_benches);

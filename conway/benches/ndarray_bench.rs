use criterion::{black_box, criterion_group, criterion_main, Criterion};
use conway::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut vec1d: Vec<usize> = vec![1; WIDTH * HEIGHT];
    let mut vec1d_alt: Vec<usize> = vec![1; WIDTH * HEIGHT];
    
    let mut vec2d: Vec<Vec<usize>>;
    let arr1d: [usize; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let arr2d: [[usize; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];

    c.bench_function("zeroing vec1d flatly", |b| b.iter(|| zero_vec1d_flat(black_box(&mut vec1d), WIDTH, HEIGHT)));
    c.bench_function("zeroing vec1d nestedly", |b| b.iter(|| zero_vec1d_nested(black_box(&mut vec1d_alt), WIDTH, HEIGHT)));
    // c.bench_function("zeroing vec2d", |b| b.iter(|| zero_vec2d(black_box(&mut vec2d))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

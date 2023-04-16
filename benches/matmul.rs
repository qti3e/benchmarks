use criterion::*;
use ndarray::Array;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn bench_matmul(c: &mut Criterion) {
    let mut g = c.benchmark_group("matmul");
    g.sample_size(20);

    for size in [10, 100, 500, 1000] {
        g.bench_with_input(BenchmarkId::new("ndarray", size), &size, |b, size| {
            let v1 = Array::random((*size, *size), Uniform::new(0., 10.));
            let v2 = Array::random((*size, *size), Uniform::new(0., 10.));

            b.iter(|| {
                let r = &v1 * &v2;
                black_box(r);
            })
        });
    }
}

criterion_group!(benches, bench_matmul);
criterion_main!(benches);

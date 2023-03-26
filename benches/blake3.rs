use benchmarks::*;
use criterion::*;

fn bench_blake3(c: &mut Criterion) {
    let data = random_vec(256 * KB);

    let mut g = c.benchmark_group("Blake3-256KiB-Chunked");
    g.sample_size(10);

    for (size, label) in SizeIterator::new(KB, 256 * KB) {
        g.throughput(Throughput::Bytes(256 * KB as u64));

        g.bench_with_input(BenchmarkId::new("update", &label), &size, |b, _i| {
            b.iter(|| {
                let mut hasher = blake3::Hasher::new();
                let mut iter = data.chunks_exact(size);

                for chunk in &mut iter {
                    let buffer = &chunk[0..size];
                    hasher.update(buffer);
                }

                if !iter.remainder().is_empty() {
                    hasher.update(iter.remainder());
                }

                let hash = hasher.finalize();
                black_box(hash);
            })
        });

        g.bench_with_input(BenchmarkId::new("update_rayon", &label), &size, |b, _i| {
            b.iter(|| {
                let mut hasher = blake3::Hasher::new();
                let mut iter = data.chunks_exact(size);

                for chunk in &mut iter {
                    let buffer = &chunk[0..size];
                    hasher.update_rayon(buffer);
                }

                if !iter.remainder().is_empty() {
                    hasher.update_rayon(iter.remainder());
                }

                let hash = hasher.finalize();
                black_box(hash);
            })
        });
    }

    g.finish();
}

criterion_group!(benches, bench_blake3);
criterion_main!(benches);

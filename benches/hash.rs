use benchmarks::*;
use criterion::*;
use sha2::{Digest, Sha256, Sha384, Sha512};

fn bench_hash(c: &mut Criterion) {
    let data = random_vec(GB);

    let mut g = c.benchmark_group("Hash");
    g.sample_size(10);

    for (size, label) in size(GB) {
        g.bench_with_input(BenchmarkId::new("Sha256", &label), &size, |b, i| {
            b.iter(|| {
                let mut sha = Sha256::new();
                sha.update(&data[0..*i]);
                let hash: [u8; 32] = sha.finalize().into();
                black_box(hash);
            })
        });

        g.bench_with_input(BenchmarkId::new("Sha384", &label), &size, |b, i| {
            b.iter(|| {
                let mut sha = Sha384::new();
                sha.update(&data[0..*i]);
                let hash: [u8; 48] = sha.finalize().into();
                black_box(hash);
            })
        });

        g.bench_with_input(BenchmarkId::new("Sha512", &label), &size, |b, i| {
            b.iter(|| {
                let mut sha = Sha512::new();
                sha.update(&data[0..*i]);
                let hash: [u8; 64] = sha.finalize().into();
                black_box(hash);
            })
        });

        g.bench_with_input(BenchmarkId::new("Blake3", &label), &size, |b, i| {
            b.iter(|| {
                let mut hasher = blake3::Hasher::new();
                hasher.update(&data[0..*i]);
                let hash: [u8; 32] = hasher.finalize().into();
                black_box(hash);
            })
        });

        g.bench_with_input(BenchmarkId::new("Blake3-Rayon", &label), &size, |b, i| {
            b.iter(|| {
                let mut hasher = blake3::Hasher::new();
                hasher.update_rayon(&data[0..*i]);
                let hash: [u8; 32] = hasher.finalize().into();
                black_box(hash);
            })
        });
    }

    g.finish();
}

criterion_group!(benches, bench_hash);
criterion_main!(benches);

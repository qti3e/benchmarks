use criterion::*;
use rand_core::OsRng;

use ark_ec::{pairing::Pairing, *};
use ark_std::{ops::Mul, UniformRand, Zero};

use elliptic_curve::ff::Field;
use elliptic_curve::ops::LinearCombination;
use ff::Field as ff13Field;

fn bench_ec(c: &mut Criterion) {
    let mut g = c.benchmark_group("Elliptic Curve");
    g.sample_size(10);

    // ---- Mul

    g.bench_function(BenchmarkId::new("ark-secp256k1", "Mul"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-secp256r1", "Mul"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-bls12-381/G1", "Mul"), |b| {
        let g = ark_bls12_381::G1Affine::generator();
        let s = ark_bls12_381::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-bls12-381/G2", "Mul"), |b| {
        let g = ark_bls12_381::G2Affine::generator();
        let s = ark_bls12_381::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("k256", "Mul"), |b| {
        let g = k256::AffinePoint::GENERATOR;
        let s = k256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("p256", "Mul"), |b| {
        let g = p256::AffinePoint::GENERATOR;
        let s = p256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G1", "Mul"), |b| {
        let g = bls12_381::G1Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G2", "Mul"), |b| {
        let g = bls12_381::G2Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    // ---- Mul Projective

    g.bench_function(BenchmarkId::new("ark-secp256k1", "Mul-Projective"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-secp256r1", "Mul-Projective"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(
        BenchmarkId::new("ark-bls12-381/G1", "Mul-Projective"),
        |b| {
            let s = ark_bls12_381::Fr::rand(&mut OsRng);
            let g = ark_bls12_381::G1Affine::generator();
            let u = g.mul(s);

            b.iter(|| {
                let r = u.mul(s);
                let _ = black_box(r);
            })
        },
    );

    g.bench_function(
        BenchmarkId::new("ark-bls12-381/G2", "Mul-Projective"),
        |b| {
            let s = ark_bls12_381::Fr::rand(&mut OsRng);
            let g = ark_bls12_381::G2Affine::generator();
            let u = g.mul(s);

            b.iter(|| {
                let r = u.mul(s);
                let _ = black_box(r);
            })
        },
    );

    g.bench_function(BenchmarkId::new("k256", "Mul-Projective"), |b| {
        let g = k256::AffinePoint::GENERATOR;
        let s = k256::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("p256", "Mul-Projective"), |b| {
        let g = p256::AffinePoint::GENERATOR;
        let s = p256::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G1", "Mul-Projective"), |b| {
        let g = bls12_381::G1Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G2", "Mul-Projective"), |b| {
        let g = bls12_381::G2Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    // ---- Add

    g.bench_function(BenchmarkId::new("ark-secp256k1", "Add"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-secp256r1", "Add"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-bls12-381/G1", "Add"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-bls12-381/G2", "Add"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("k256", "Add"), |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let g = k256::AffinePoint::GENERATOR;
        let u = k256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("p256", "Add"), |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let g = p256::AffinePoint::GENERATOR;
        let u = p256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G1", "Add"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let g = bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G2", "Add"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let g = bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    // ---- To Affine

    g.bench_function(BenchmarkId::new("ark-secp256k1", "ToAffine"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-secp256r1", "ToAffine"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-bls12-381/G1", "ToAffine"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-bls12-381/G2", "ToAffine"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("k256", "ToAffine"), |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let u = k256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u.to_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("p256", "ToAffine"), |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let u = p256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u.to_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G1", "ToAffine"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G1Affine::generator().mul(s);

        b.iter(|| {
            let r = bls12_381::G1Affine::from(&u);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G2", "ToAffine"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G2Affine::generator().mul(s);

        b.iter(|| {
            let r = bls12_381::G2Affine::from(&u);
            let _ = black_box(r);
        })
    });

    // ---- Mul by zero

    g.bench_function(BenchmarkId::new("ark-secp256k1", "MulByZero"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_secp256k1::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-secp256r1", "MulByZero"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_secp256r1::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-bls12-381/G1", "MulByZero"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_bls12_381::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-bls12-381/G2", "MulByZero"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_bls12_381::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("k256", "MulByZero"), |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let u = k256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u * k256::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("p256", "MulByZero"), |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let u = p256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u * p256::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G1", "MulByZero"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G1Affine::generator().mul(s);

        b.iter(|| {
            let r = u * bls12_381::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381/G2", "MulByZero"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G2Affine::generator().mul(s);

        b.iter(|| {
            let r = u * bls12_381::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    // ---- Pedersen Commitment

    g.bench_function(BenchmarkId::new("ark-secp256k1", "Pedersen"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let h = g.mul(s);
        let r = ark_secp256k1::Fr::rand(&mut OsRng);
        let m = ark_secp256k1::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = h * m + g * r;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ark-secp256r1", "Pedersen"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let h = g.mul(s);
        let r = ark_secp256r1::Fr::rand(&mut OsRng);
        let m = ark_secp256r1::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = h * m + g * r;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("k256", "Pedersen"), |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let g = k256::ProjectivePoint::GENERATOR;
        let h = g.mul(s);
        let r = k256::Scalar::random(&mut OsRng);
        let m = k256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = k256::ProjectivePoint::lincomb(&h, &m, &g, &r);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("p256", "Pedersen"), |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let g = p256::ProjectivePoint::GENERATOR;
        let h = g.mul(s);
        let r = p256::Scalar::random(&mut OsRng);
        let m = p256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = p256::ProjectivePoint::lincomb(&h, &m, &g, &r);
            let _ = black_box(r);
        })
    });

    // BLS pairing

    g.bench_function(BenchmarkId::new("ark-bls12-381", "Pairing"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = ark_bls12_381::G2Affine::generator().mul(&s).into_affine();

        b.iter(|| {
            let r = ark_bls12_381::Bls12_381::pairing(g, u);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("bls12-381", "Pairing"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let g = bls12_381::G1Affine::generator();
        let u = bls12_381::G2Affine::from(&bls12_381::G2Affine::generator().mul(s));

        b.iter(|| {
            let r = bls12_381::pairing(&g, &u);
            let _ = black_box(r);
        })
    });

    g.finish();
}

criterion_group!(benches, bench_ec);
criterion_main!(benches);

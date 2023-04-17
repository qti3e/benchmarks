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

    g.bench_function(BenchmarkId::new("Mul", "ark-secp256k1"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-secp256r1"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-curve25519"), |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-bls12-381/G1"), |b| {
        let g = ark_bls12_381::G1Affine::generator();
        let s = ark_bls12_381::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-bls12-381/G2"), |b| {
        let g = ark_bls12_381::G2Affine::generator();
        let s = ark_bls12_381::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "k256"), |b| {
        let g = k256::AffinePoint::GENERATOR;
        let s = k256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "p256"), |b| {
        let g = p256::AffinePoint::GENERATOR;
        let s = p256::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "bls12-381/G1"), |b| {
        let g = bls12_381::G1Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "bls12-381/G2"), |b| {
        let g = bls12_381::G2Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);

        b.iter(|| {
            let r = g.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "alkali/ed25519"), |b| {
        let n = alkali::curve::ed25519::Scalar::generate().unwrap();

        b.iter(|| {
            let r = alkali::curve::ed25519::scalar_mult_base(&n).unwrap();
            let _ = black_box(r);
        })
    });

    // ---- Mul Projective

    g.bench_function(BenchmarkId::new("Mul", "ark-secp256k1/Projective"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-secp256r1/Projective"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-curve25519/Projective"), |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(
        BenchmarkId::new("Mul", "ark-bls12-381/G1/Projective"),
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
        BenchmarkId::new("Mul", "ark-bls12-381/G2/Projective"),
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

    g.bench_function(BenchmarkId::new("Mul", "k256/Projective"), |b| {
        let g = k256::AffinePoint::GENERATOR;
        let s = k256::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "p256/Projective"), |b| {
        let g = p256::AffinePoint::GENERATOR;
        let s = p256::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "bls12-381/G1/Projective"), |b| {
        let g = bls12_381::G1Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "bls12-381/G2/Projective"), |b| {
        let g = bls12_381::G2Affine::generator();
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = g.mul(s);

        b.iter(|| {
            let r = u.mul(s);
            let _ = black_box(r);
        })
    });

    // ---- Mul by zero

    g.bench_function(BenchmarkId::new("Mul", "ark-secp256k1/Zero"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_secp256k1::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-secp256r1/Zero"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_secp256r1::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-curve25519/Zero"), |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_curve25519::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-bls12-381/G1/Zero"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_bls12_381::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "ark-bls12-381/G2/Zero"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u * ark_bls12_381::Fr::zero();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "k256/Zero"), |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let u = k256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u * k256::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "p256/Zero"), |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let u = p256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u * p256::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "bls12-381/G1/Zero"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G1Affine::generator().mul(s);

        b.iter(|| {
            let r = u * bls12_381::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Mul", "bls12-381/G2/Zero"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G2Affine::generator().mul(s);

        b.iter(|| {
            let r = u * bls12_381::Scalar::ZERO;
            let _ = black_box(r);
        })
    });

    // ---- Add

    g.bench_function(BenchmarkId::new("Add", "ark-secp256k1"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Add", "ark-secp256r1"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Add", "ark-curve25519"), |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Add", "ark-bls12-381/G1"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Add", "ark-bls12-381/G2"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = g + u;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Add", "k256"), |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let g = k256::AffinePoint::GENERATOR;
        let u = k256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Add", "p256"), |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let g = p256::AffinePoint::GENERATOR;
        let u = p256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Add", "bls12-381/G1"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let g = bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Add", "bls12-381/G2"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let g = bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u + g;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Add", "alkali/ed25519"), |b| {
        let n = alkali::curve::ed25519::Scalar::generate().unwrap();
        let q = alkali::curve::ed25519::scalar_mult_base(&n).unwrap();

        let n = alkali::curve::ed25519::Scalar::generate().unwrap();
        let p = alkali::curve::ed25519::scalar_mult_base(&n).unwrap();

        b.iter(|| {
            let r = q.add(&p);
            let _ = black_box(r);
        })
    });

    // ---- To Affine

    g.bench_function(BenchmarkId::new("ToAffine", "ark-secp256k1"), |b| {
        let s = ark_secp256k1::Fr::rand(&mut OsRng);
        let g = ark_secp256k1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ToAffine", "ark-secp256r1"), |b| {
        let s = ark_secp256r1::Fr::rand(&mut OsRng);
        let g = ark_secp256r1::Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ToAffine", "ark-curve25519"), |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ToAffine", "ark-bls12-381/G1"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ToAffine", "ark-bls12-381/G2"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G2Affine::generator();
        let u = g.mul(s);

        b.iter(|| {
            let r = u.into_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ToAffine", "k256"), |b| {
        let s = k256::Scalar::random(&mut OsRng);
        let u = k256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u.to_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ToAffine", "p256"), |b| {
        let s = p256::Scalar::random(&mut OsRng);
        let u = p256::AffinePoint::GENERATOR.mul(s);

        b.iter(|| {
            let r = u.to_affine();
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ToAffine", "bls12-381/G1"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G1Affine::generator().mul(s);

        b.iter(|| {
            let r = bls12_381::G1Affine::from(&u);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("ToAffine", "bls12-381/G2"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let u = bls12_381::G2Affine::generator().mul(s);

        b.iter(|| {
            let r = bls12_381::G2Affine::from(&u);
            let _ = black_box(r);
        })
    });

    // ---- Pedersen Commitment

    g.bench_function(BenchmarkId::new("Pedersen", "ark-secp256k1"), |b| {
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

    g.bench_function(BenchmarkId::new("Pedersen", "ark-secp256r1"), |b| {
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

    g.bench_function(BenchmarkId::new("Pedersen", "ark-curve25519"), |b| {
        let s = ark_curve25519::Fr::rand(&mut OsRng);
        let g = ark_curve25519::EdwardsAffine::generator();
        let h = g.mul(s);
        let r = ark_curve25519::Fr::rand(&mut OsRng);
        let m = ark_curve25519::Fr::rand(&mut OsRng);

        b.iter(|| {
            let r = h * m + g * r;
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Pedersen", "k256"), |b| {
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

    g.bench_function(BenchmarkId::new("Pedersen", "p256"), |b| {
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

    g.bench_function(BenchmarkId::new("Pairing", "ark-bls12-381"), |b| {
        let s = ark_bls12_381::Fr::rand(&mut OsRng);
        let g = ark_bls12_381::G1Affine::generator();
        let u = ark_bls12_381::G2Affine::generator().mul(&s).into_affine();

        b.iter(|| {
            let r = ark_bls12_381::Bls12_381::pairing(g, u);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Pairing", "bls12-381"), |b| {
        let s = bls12_381::Scalar::random(&mut OsRng);
        let g = bls12_381::G1Affine::generator();
        let u = bls12_381::G2Affine::from(&bls12_381::G2Affine::generator().mul(s));

        b.iter(|| {
            let r = bls12_381::pairing(&g, &u);
            let _ = black_box(r);
        })
    });

    g.bench_function(BenchmarkId::new("Pairing", "blst"), |b| {
        let g = unsafe { blst::BLS12_381_G1 };
        let u = unsafe { blst::BLS12_381_G2 };

        b.iter(|| {
            let mut pairing = blst::Pairing::new(false, &[]);
            pairing.raw_aggregate(&u, &g);
            let r = pairing.as_fp12();
            let _ = black_box(r);
        })
    });

    g.finish();
}

criterion_group!(benches, bench_ec);
criterion_main!(benches);

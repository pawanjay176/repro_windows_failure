use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::ThreadRng, Rng};
use repro_windows_failure::*;

fn generate_random_bytes48(rng: &mut ThreadRng) -> Bytes48 {
    let mut arr = [0u8; 48];
    rng.fill(&mut arr[..]);
    arr[0] = 0;
    arr.into()
}

fn generate_random_blob(rng: &mut ThreadRng) -> Blob {
    let mut arr = [0u8; BYTES_PER_BLOB];
    rng.fill(&mut arr[..]);
    // Ensure that the blob is canonical by ensuring that
    // each field element contained in the blob is < BLS_MODULUS
    for i in 0..4096 {
        arr[i * 32] = 0;
    }
    arr.into()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let blob = generate_random_blob(&mut rng);
    let commitment = generate_random_bytes48(&mut rng);
    let proof = generate_random_bytes48(&mut rng);

    c.bench_function("verify_blob", |b| {
        b.iter(|| verify_blob_safe(blob, commitment, proof))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#![cfg_attr(tarpaulin, skip)]

#[macro_use]
extern crate criterion;
extern crate bsdiff_rs;
extern crate rand_pcg;

use criterion::{black_box, BenchmarkId};
use criterion::Criterion;

use rand::Rng;
use std::io;
use std::io::Write;
use bsdiff_rs::{BsDiff, Backend};
use bsdiff_rs::c::CBackend;
use bsdiff_rs::rust::RustBackend;

struct WriteDummy;

impl Write for WriteDummy {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let len = buf.len();
        black_box(buf);
        Ok(len)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand_pcg::Pcg64Mcg::new(5);
    let test_data_1: Vec<u8> = (&mut rng)
        .sample_iter(rand::distributions::Standard)
        .take(10_000)
        .collect();
    let test_data_2: Vec<u8> = (&mut rng)
        .sample_iter(rand::distributions::Standard)
        .take(10_000)
        .collect();
    let mut write_dummy = WriteDummy {};

    let mut group = c.benchmark_group("BsDiff");
    group.bench_function(BenchmarkId::new("C Backend", ""), |b| {
        b.iter(|| BsDiff::<CBackend>::bsdiff_raw(&test_data_1[..2_000], &test_data_2[..2_000], &mut write_dummy));
    });
    group.bench_function(BenchmarkId::new("Rust Backend", ""), |b| {
        b.iter(|| BsDiff::<RustBackend>::bsdiff_raw(&test_data_1[..2_000], &test_data_2[..2_000], &mut write_dummy));
    });
    group.finish();

    let mut patch_1 = Vec::new();
    BsDiff::<CBackend>::bsdiff_raw(&test_data_1[..], &test_data_2[..], &mut patch_1).unwrap();
    let testout = &mut [0u8; 10_000];

    let mut group = c.benchmark_group("BsPatch");
    group.bench_function(BenchmarkId::new("C Backend", ""), |b| {
        b.iter(|| BsDiff::<CBackend>::bspatch_raw(&test_data_1[..], testout, &mut &patch_1[..]))
    });
    group.bench_function(BenchmarkId::new("Rust Backend", ""), |b| {
        b.iter(|| BsDiff::<RustBackend>::bspatch_raw(&test_data_1[..], testout, &mut &patch_1[..]))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

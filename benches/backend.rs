#[macro_use]
extern crate criterion;
extern crate bsdiff_rs;
extern crate rand_pcg;

use criterion::black_box;
use criterion::Criterion;

use bsdiff_rs::raw::{bsdiff_raw, bspatch_raw};
use rand::Rng;
use std::io;
use std::io::Write;

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
    c.bench_function("bsdiff", |b| {
        b.iter(|| bsdiff_raw(&test_data_1[..], &test_data_2[..], &mut write_dummy))
    });
    let mut patch_1 = Vec::new();
    bsdiff_raw(&test_data_1[..], &test_data_2[..], &mut patch_1).unwrap();
    let testout = &mut [0u8; 10_000];
    c.bench_function("bspatch", |b| {
        b.iter(|| bspatch_raw(&test_data_1[..], testout, &mut &patch_1[..]))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

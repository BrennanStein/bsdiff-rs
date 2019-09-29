pub use super::raw::bsdiff_raw;
pub use super::raw::bspatch_raw;
use std::io::{Read, Write};

use rand::Rng;
use crate::BsDiffResult;

type BsDiffFn = fn(old: &[u8], new: &[u8], patch: &mut dyn Write) -> BsDiffResult;
type BsPatchFn = fn(old: &[u8], new: &mut [u8], patch: &mut dyn Read) -> BsDiffResult;

macro_rules! backend_tests {
    ($d:ident, $p:ident) => {
        use crate::test::{hardcoded_data_t, long_random_data_t};
        #[test]
        fn hardcoded_data() {
            hardcoded_data_t($d, $p);
        }

        #[test]
        fn long_random_data() {
            long_random_data_t($d, $p)
        }
    };
}

pub fn hardcoded_data_t(bsdiff_raw: BsDiffFn, bspatch_raw: BsPatchFn) {
    let old_data = &[0u8, 1u8, 2u8, 3u8, 4u8, 10u8, 9u8, 255u8, 0u8, 72u8];
    let new_data = &[1u8, 0u8, 2u8, 3u8, 4u8, 10u8, 90u8, 0u8, 0u8, 255u8];

    let mut patch: Vec<u8> = Vec::new();

    bsdiff_raw(old_data, new_data, &mut patch).unwrap();

    let mut output_data: [u8; 10] = [0u8; 10];
    bspatch_raw(old_data, &mut output_data, &mut &patch[..]).unwrap();

    assert_eq!(&output_data, new_data);
}

pub fn long_random_data_t(bsdiff_raw: BsDiffFn, bspatch_raw: BsPatchFn) {
    for _ in 0..10 {
        let old_data = rand::thread_rng()
            .sample_iter(rand::distributions::Standard)
            .take(1000)
            .collect::<Vec<u8>>();
        let new_data = rand::thread_rng()
            .sample_iter(rand::distributions::Standard)
            .take(99)
            .collect::<Vec<u8>>();

        let mut patch = Vec::<u8>::new();
        bsdiff_raw(&old_data[..], &new_data[..], &mut patch).unwrap();

        let mut output_data = vec![0u8; 99];
        bspatch_raw(&old_data[..], &mut output_data[..], &mut &patch[..]).unwrap();

        assert_eq!(&output_data[..], &new_data[..])
    }
}

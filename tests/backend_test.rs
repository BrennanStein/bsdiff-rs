use bsdiff_rs::{bsdiff_raw, bspatch_raw};
use rand::Rng;

#[macro_use]
extern crate lazy_static;

macro_rules! data_cases {
    ($($name:ident: $tuple:expr),*) => {
    $(
        mod $name {
            use super::*;
            #[test]
            fn diff_patch_test_t() {
                let (old, new) = &$tuple;
                diff_patch_test(old, new);
            }
        }
    )*
    }
}

pub fn diff_patch_test(old: &[u8], new: &[u8]) {
    let mut patch: Vec<u8> = Vec::new();

    bsdiff_raw(old, new, &mut patch).expect("Failed to diff");

    let mut generated: Vec<u8> = vec![0; new.len()];
    bspatch_raw(old, &mut generated, &patch[..]).expect("Failed to patch");

    assert_eq!(&generated[..], new);
}

const HARDCODED_DATA: (&[u8], &[u8]) = (
    &[0u8, 1u8, 2u8, 3u8, 4u8, 10u8, 9u8, 255u8, 0u8, 72u8],
    &[1u8, 0u8, 2u8, 3u8, 4u8, 10u8, 90u8, 0u8, 0u8, 255u8],
);

pub fn generate_data(seed: u128, length: usize) -> Vec<u8> {
    rand_pcg::Pcg64Mcg::new(seed)
        .sample_iter(rand::distributions::Standard)
        .take(length)
        .collect()
}

pub fn insert_bits(old: Vec<u8>, seed: u128) -> (Vec<u8>, Vec<u8>) {
    let middle = old.len() / 2;
    let prepend = generate_data(seed, 20).into_iter();
    let first_half = old[..middle].iter().cloned();
    let insert = generate_data(seed + 1, 17).into_iter();
    let second_half = old[middle..].iter().cloned();
    let append = generate_data(seed + 2, 23).into_iter();

    let new: Vec<u8> = prepend
        .chain(first_half)
        .chain(insert)
        .chain(second_half)
        .chain(append)
        .collect();
    (old, new)
}

pub fn delete_bits(old: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let middle = old.len() / 2;
    let first_half = old[10..(middle - 15)].iter().cloned();
    let second_half = old[middle..].iter().cloned();
    let new: Vec<u8> = first_half.chain(second_half).collect();
    (old, new)
}

lazy_static! {
    static ref RANDOM_DATA: (Vec<u8>, Vec<u8>) = (generate_data(0, 10000), generate_data(1, 10000));
    static ref NEW_SMALLER: (Vec<u8>, Vec<u8>) = (generate_data(2, 10000), generate_data(3, 5000));
    static ref NEW_BIGGER: (Vec<u8>, Vec<u8>) = (generate_data(3, 5000), generate_data(4, 10000));
    static ref INSERT_BITS: (Vec<u8>, Vec<u8>) = insert_bits(generate_data(5, 10000), 6);
    static ref DELETE_BITS: (Vec<u8>, Vec<u8>) = delete_bits(generate_data(9, 10000));
}

data_cases! {
    hardcoded_data: HARDCODED_DATA,
    random_data: (generate_data(0, 10000), generate_data(1, 10000)),
    new_smaller: (generate_data(2, 10000), generate_data(3, 5000)),
    new_bigger: (generate_data(3, 5000), generate_data(4, 10000)),
    insert_bits: insert_bits(generate_data(5, 10000), 6),
    delete_bits: delete_bits(generate_data(9, 10000))
}

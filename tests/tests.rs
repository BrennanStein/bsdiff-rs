use bsdiff_rs::{bsdiff_43, bspatch_raw};
use bsdiff_rs;
use rand::Rng;
use std::io::{Cursor, Read, Write};
use bsdiff_rs::bsdiff_43::bsdiff;

const BSDIFF43_PATCH_1: &[u8] = include_bytes!("bsdiff43_patch_1");
const BSDIFF43_PATCH_2: &[u8] = include_bytes!("bsdiff43_patch_2");

#[test]
fn rust_rewrite_test() {
    let mut patch = Vec::<u8>::new();
    bsdiff_rs::rust::bsdiff::bsdiff(DUMMY_OLD_1, DUMMY_NEW_1, &mut patch);
    assert_eq!(&patch[..], BSDIFF43_PATCH_1);

    let mut output = Vec::<u8>::new();
    bsdiff_rs::rust::bspatch::bspatch(DUMMY_OLD_1, &mut output, &mut &patch[..]).unwrap();

    let mut patch2 = Vec::<u8>::new();
    bsdiff_rs::rust::bsdiff::bsdiff(DUMMY_OLD_2, DUMMY_NEW_2, &mut patch2);
    assert_eq!(&patch2[..], BSDIFF43_PATCH_2);

    let mut output2 = Vec::<u8>::new();
    bsdiff_rs::rust::bspatch::bspatch(DUMMY_OLD_2, &mut output2, &mut &patch2[..]).unwrap();
    assert_eq!(output2, DUMMY_NEW_2);
}

#[test]
fn hardcoded_data() {
    let old_data = &[0u8, 1u8, 2u8, 3u8, 4u8, 10u8, 9u8, 255u8, 0u8, 72u8];
    let new_data = &[1u8, 0u8, 2u8, 3u8, 4u8, 10u8, 90u8, 0u8, 0u8, 255u8];

    let mut patch: Vec<u8> = Vec::new();

    bsdiff_rs::bsdiff_raw(old_data, new_data, &mut patch);

    let mut output_data: [u8; 10] = [0u8; 10];
    bsdiff_rs::bspatch_raw(old_data, &mut output_data, &mut &patch[..]);

    assert_eq!(&output_data, new_data);
}

#[test]
fn long_random_data() {
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
        bsdiff_rs::bsdiff_raw(&old_data[..], &new_data[..], &mut patch);

        println!("{:X?}", &patch[..50]);

        let mut output_data = vec![0u8; 99];
        bsdiff_rs::bspatch_raw(&old_data[..], &mut output_data[..], &mut &patch[..]);

        assert_eq!(&output_data[..], &new_data[..])
    }
}

const DUMMY_OLD_1: &[u8] = include_bytes!("dummy_old_1");
const DUMMY_OLD_2: &[u8] = include_bytes!("dummy_old_2");
const DUMMY_NEW_1: &[u8] = include_bytes!("dummy_new_1");
const DUMMY_NEW_2: &[u8] = include_bytes!("dummy_new_2");

#[test]
fn bsdiff_43_bspatch() {
    let patch_1: &[u8] = include_bytes!("bsdiff43_patch_1");
    let mut output_1 = Vec::new();
    bsdiff_43::bspatch(DUMMY_OLD_1, &mut output_1, &mut &*patch_1);
    assert_eq!(output_1.as_slice(), DUMMY_NEW_1);

    let patch_2: &[u8] = include_bytes!("bsdiff43_patch_2");
    let mut output_2 = Vec::new();
    bsdiff_43::bspatch(DUMMY_OLD_2, &mut output_2, &mut &*patch_2);
    assert_eq!(output_2.as_slice(), DUMMY_NEW_2);
}

#[test]
fn bsdiff_43_bsdiff() {
    let mut patch_1 = Vec::new();
    bsdiff_43::bsdiff(DUMMY_OLD_1, DUMMY_NEW_1, &mut patch_1);
    let mut output_1 = Vec::new();
    bsdiff_43::bspatch(DUMMY_OLD_1, &mut output_1, &mut patch_1.as_slice());
    assert_eq!(output_1.as_slice(), DUMMY_NEW_1);

    let mut patch_2 = Vec::new();
    bsdiff_43::bsdiff(DUMMY_OLD_2, DUMMY_NEW_2, &mut patch_2);
    let mut output_2 = Vec::new();
    bsdiff_43::bspatch(DUMMY_OLD_2, &mut output_2, &mut patch_2.as_slice());
    assert_eq!(output_2.as_slice(), DUMMY_NEW_2);
}

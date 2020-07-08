#![cfg(feature = "integration_test")]
use bsdiff_rs::{bsdiff43, bspatch43};
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use tempdir::TempDir;

#[macro_use]
extern crate lazy_static;

fn create_dummy_bytes(state: u128, len: usize) -> Vec<u8> {
    let rng = Pcg64Mcg::new(state);
    rng.sample_iter(rand::distributions::Standard)
        .take(len)
        .collect()
}

lazy_static! {
    static ref DUMMY_OLD_1: Vec<u8> = create_dummy_bytes(1, 1000000);
    static ref DUMMY_OLD_2: Vec<u8> = create_dummy_bytes(2, 1000);
    static ref DUMMY_OLD_3: Vec<u8> = create_dummy_bytes(3, 10000);
    static ref DUMMY_NEW_1: Vec<u8> = create_dummy_bytes(4, 1000);
    static ref DUMMY_NEW_2: Vec<u8> = create_dummy_bytes(5, 1000000);
    static ref DUMMY_NEW_3: Vec<u8> = create_dummy_bytes(6, 10000);
}

macro_rules! data_cases {
    ($($name:ident: ($old:ident, $new:ident)),*) => {
    $(
        mod $name {
            use super::*;
            #[test]
            fn bsdiff_43_t() {
                bsdiff_43_diff_patch(&$old, &$new);
            }

            #[test]
            fn check_patch_eq_t() {
                check_patch_eq(&$old, &$new).unwrap();
            }
        }
    )*
    }
}

data_cases! {
    test_1: (DUMMY_OLD_1, DUMMY_NEW_1),
    test_2: (DUMMY_OLD_2, DUMMY_NEW_2),
    test_3: (DUMMY_OLD_3, DUMMY_NEW_3)
}

fn bsdiff_43_diff_patch(old: &[u8], new: &[u8]) {
    let mut patch_1 = Vec::new();
    bsdiff43(old, new, &mut patch_1).expect("1");
    let mut output_1 = Vec::new();
    bspatch43(old, &mut output_1, &mut patch_1.as_slice()).expect("2");
    assert_eq!(output_1.as_slice(), new);
}

fn check_patch_eq(old: &[u8], new: &[u8]) -> io::Result<()> {
    const BSDIFF_EXECUTABLE_PATH: &str = "target/c/bsdiff";
    assert!(
        Path::new(BSDIFF_EXECUTABLE_PATH).exists(),
        "The C bsdiff executable cannot be found. (Have you run test_setup.sh?)"
    );
    let work_dir = TempDir::new("bsdiff-testing")?;
    // let work_dir = PathBuf::from("test_work");
    // DirBuilder::new().create(&work_dir).unwrap();
    let original_path = work_dir.path().join("original.bin");
    let mut original_file = File::create(&original_path)?;
    let new_path = work_dir.path().join("changed.bin");
    let mut new_file = File::create(&new_path)?;
    let patch_path = work_dir.path().join("delta.patch");

    original_file.write_all(old)?;
    new_file.write_all(new)?;
    let patch_status = Command::new(BSDIFF_EXECUTABLE_PATH)
        .arg(&original_path)
        .arg(&new_path)
        .arg(&patch_path)
        .status()
        .expect("Failed to run bsdiff");
    assert!(patch_status.success());

    let patch_file = File::open(patch_path)?;
    let expected_patch = patch_file.bytes().map(|r| r.unwrap()).collect::<Vec<_>>();
    assert_ne!(expected_patch.len(), 0);

    let mut actual_patch = Vec::new();
    bsdiff43(old, new, &mut actual_patch)?;
    assert_eq!(expected_patch, actual_patch);
    Ok(())
}

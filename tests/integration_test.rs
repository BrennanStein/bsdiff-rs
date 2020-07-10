#![cfg(feature = "integration_test")]
use bsdiff_rs::{bsdiff43, bspatch43, BsDiffResult};
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
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

trait Integration {
    fn diff<S: AsRef<OsStr>>(old_file: S, new_file: S, patch_file: S) -> bool;
    fn patch<S: AsRef<OsStr>>(old_file: S, new_file: S, patch_file: S) -> bool;

    fn rust_diff<W: Write>(old: &[u8], new: &[u8], patch: W) -> BsDiffResult<()>;
    fn rust_patch<W: Write, R: Read>(old: &[u8], new: W, patch: R) -> BsDiffResult<()>;
}

mod bsdiff_c {
    use super::Integration;
    use bsdiff_rs::{bsdiff43, bspatch43, BsDiffResult};
    use std::ffi::OsStr;
    use std::io::{Read, Write};
    use std::path::Path;
    use std::process::Command;

    const BSDIFF_C_EXECUTABLE_PATH: &str = "target/c/bsdiff";
    const BSPATCH_C_EXECUTABLE_PATH: &str = "target/c/bspatch";
    pub struct BsDiffC;

    impl Integration for BsDiffC {
        fn diff<S: AsRef<OsStr>>(old_file: S, new_file: S, patch_file: S) -> bool {
            assert!(
                Path::new(BSDIFF_C_EXECUTABLE_PATH).exists(),
                "The C bsdiff executable cannot be found. (Have you run test_setup.sh?)"
            );
            let patch_status = Command::new(BSDIFF_C_EXECUTABLE_PATH)
                .arg(&old_file)
                .arg(&new_file)
                .arg(&patch_file)
                .status()
                .expect("Failed to run bsdiff");
            patch_status.success()
        }

        fn patch<S: AsRef<OsStr>>(old_file: S, new_file: S, patch_file: S) -> bool {
            assert!(
                Path::new(BSPATCH_C_EXECUTABLE_PATH).exists(),
                "The C bspatch executable cannot be found. (Have you run test_setup.sh?)"
            );
            let patch_status = Command::new(BSPATCH_C_EXECUTABLE_PATH)
                .arg(&old_file)
                .arg(&new_file)
                .arg(&patch_file)
                .status()
                .expect("Failed to run bspatch");
            patch_status.success()
        }

        fn rust_diff<W: Write>(old: &[u8], new: &[u8], patch: W) -> BsDiffResult<()> {
            bsdiff43(old, new, patch)
        }

        fn rust_patch<W: Write, R: Read>(old: &[u8], new: W, patch: R) -> BsDiffResult<()> {
            bspatch43(old, new, patch)
        }
    }
}

#[cfg(not(feature = "c_backend"))]
mod bsdiff_java {
    use super::Integration;
    use bsdiff_rs::{jbsdiff40, jbspatch40, BsDiffResult};
    use std::ffi::OsStr;
    use std::io::{Read, Write};
    use std::path::Path;
    use std::process::Command;

    const BSDIFF_JAVA_EXECUTABLE_PATH: &str = "target/java/jbsdiff";
    pub struct BsDiffJava;

    impl Integration for BsDiffJava {
        fn diff<S: AsRef<OsStr>>(old_file: S, new_file: S, patch_file: S) -> bool {
            assert!(
                Path::new(BSDIFF_JAVA_EXECUTABLE_PATH).exists(),
                "The Java bsdiff executable cannot be found. (Have you run test_setup.sh?)"
            );
            let patch_status = Command::new("java")
                .arg("-jar")
                .arg(BSDIFF_JAVA_EXECUTABLE_PATH)
                .arg("diff")
                .arg(&old_file)
                .arg(&new_file)
                .arg(&patch_file)
                .status()
                .expect("Failed to run bsdiff");
            patch_status.success()
        }
        fn patch<S: AsRef<OsStr>>(old_file: S, new_file: S, patch_file: S) -> bool {
            assert!(
                Path::new(BSDIFF_JAVA_EXECUTABLE_PATH).exists(),
                "The Java bsdiff executable cannot be found. (Have you run test_setup.sh?)"
            );
            let patch_status = Command::new("java")
                .arg("-jar")
                .arg(BSDIFF_JAVA_EXECUTABLE_PATH)
                .arg("patch")
                .arg(&old_file)
                .arg(&new_file)
                .arg(&patch_file)
                .status()
                .expect("Failed to run bspatch");
            patch_status.success()
        }
        fn rust_diff<W: Write>(old: &[u8], new: &[u8], patch: W) -> BsDiffResult<()> {
            jbsdiff40(old, new, patch)
        }

        fn rust_patch<W: Write, R: Read>(old: &[u8], new: W, patch: R) -> BsDiffResult<()> {
            jbspatch40(old, new, patch)
        }
    }
}

macro_rules! data_cases {
    ($($name:ident: ($old:ident, $new:ident)),*) => {
    $(
        mod $name {
            use super::*;
            #[test]
            fn bsdiff_43_diff_patch_t() {
                bsdiff_43_diff_patch(&$old, &$new);
            }

            #[test]
            fn check_patch_eq_c() {
                check_patch_eq::<bsdiff_c::BsDiffC>(&$old, &$new);
            }

            #[cfg(not(feature = "c_backend"))]
            #[test]
            fn check_patch_eq_java() {
                check_patch_eq::<bsdiff_java::BsDiffJava>(&$old, &$new);
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

fn check_patch_eq<I: Integration>(old: &[u8], new: &[u8]) {
    let work_dir = TempDir::new("bsdiff-testing").expect("Unable to create tempdir");
    let result = std::panic::catch_unwind(|| {
        let original_path = work_dir.path().join("original.bin");
        let mut original_file =
            File::create(&original_path).expect("Unable to create original file");
        let new_path = work_dir.path().join("changed.bin");
        let mut new_file = File::create(&new_path).expect("Unable to create new file");
        let patch_path = work_dir.path().join("delta.patch");

        original_file
            .write_all(old)
            .expect("Unable to write to original file");
        new_file
            .write_all(new)
            .expect("Unable to write to new file");
        assert!(I::diff(&original_path, &new_path, &patch_path));

        let patch_file = File::open(&patch_path).expect("Unable to open patch file");
        let expected_patch = patch_file.bytes().map(|r| r.unwrap()).collect::<Vec<_>>();
        assert_ne!(expected_patch.len(), 0);

        let mut actual_patch = Vec::new();
        I::rust_diff(old, new, &mut actual_patch).expect("Rust Failed to Diff");

        let actual_patch_path = work_dir.path().join("actual.patch");
        let mut actual_patch_file =
            File::create(&actual_patch_path).expect("Unable to create actual patch file");
        actual_patch_file
            .write_all(&actual_patch)
            .expect("Unable to write to actual patch file");

        let integrate_generate_path = work_dir.path().join("integrate_generate.bin");
        assert!(I::patch(
            &original_path,
            &integrate_generate_path,
            &actual_patch_path
        ));
        let integrate_generate_file =
            File::open(integrate_generate_path).expect("Failed to open generated file");
        let rust_diff_result = integrate_generate_file
            .bytes()
            .map(|r| r.unwrap())
            .collect::<Vec<_>>();
        let mut rust_patch_result = Vec::new();
        I::rust_patch(&old, &mut rust_patch_result, &expected_patch[..])
            .expect("Rust failed to patch");

        if rust_diff_result != new {
            assert!(false, "Rust Diff implementation failed");
        }
        if rust_patch_result != new {
            assert!(false, "Rust Patch implementation failed");
        }
    });

    if result.is_err() {
        let path = work_dir.into_path();
        eprintln!("Workdir: {:?}", path);
        result.unwrap();
    }
}

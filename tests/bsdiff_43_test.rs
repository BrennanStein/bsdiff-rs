use bsdiff_rs;
use bsdiff_rs::{bsdiff_43, bspatch_43};

const DUMMY_OLD_1: &[u8] = include_bytes!("dummy_old_1");
const DUMMY_OLD_2: &[u8] = include_bytes!("dummy_old_2");
const DUMMY_NEW_1: &[u8] = include_bytes!("dummy_new_1");
const DUMMY_NEW_2: &[u8] = include_bytes!("dummy_new_2");

const BSDIFF43_PATCH_1: &[u8] = include_bytes!("bsdiff43_patch_1");
const BSDIFF43_PATCH_2: &[u8] = include_bytes!("bsdiff43_patch_2");

#[test]
fn bsdiff_43_bspatch() {
    let patch_1: &[u8] = BSDIFF43_PATCH_1;
    let mut output_1 = Vec::new();
    bspatch_43(DUMMY_OLD_1, &mut output_1, &mut &*patch_1).unwrap();
    assert_eq!(output_1.as_slice(), DUMMY_NEW_1);

    let patch_2: &[u8] = BSDIFF43_PATCH_2;
    let mut output_2 = Vec::new();
    bspatch_43(DUMMY_OLD_2, &mut output_2, &mut &*patch_2).unwrap();
    assert_eq!(output_2.as_slice(), DUMMY_NEW_2);
}

#[test]
fn bsdiff_43_bsdiff() {
    let mut patch_1 = Vec::new();
    bsdiff_43(DUMMY_OLD_1, DUMMY_NEW_1, &mut patch_1).unwrap();
    let mut output_1 = Vec::new();
    bspatch_43(DUMMY_OLD_1, &mut output_1, &mut patch_1.as_slice()).unwrap();
    assert_eq!(output_1.as_slice(), DUMMY_NEW_1);

    let mut patch_2 = Vec::new();
    bsdiff_43(DUMMY_OLD_2, DUMMY_NEW_2, &mut patch_2).unwrap();
    let mut output_2 = Vec::new();
    bspatch_43(DUMMY_OLD_2, &mut output_2, &mut patch_2.as_slice()).unwrap();
    assert_eq!(output_2.as_slice(), DUMMY_NEW_2);
}

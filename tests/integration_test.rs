use bsdiff_rs;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

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
    static ref DUMMY_NEW_1: Vec<u8> = create_dummy_bytes(3, 1000);
    static ref DUMMY_NEW_2: Vec<u8> = create_dummy_bytes(4, 1000000);
}

macro_rules! integration_test {
    ($d:ident) => {
        use bsdiff_rs::c::CBackend;
        use bsdiff_rs::rust::RustBackend;
        #[test]
        fn c_backend() {
            $d::<CBackend>()
        }

        #[test]
        fn rust_backend() {
            $d::<RustBackend>()
        }
    };
}

mod bsdiff_43 {
    use super::*;
    use bsdiff_rs::{Backend, BsDiff};

    fn bsdiff_43<B: Backend>() {
        let mut patch_1 = Vec::new();
        BsDiff::<B>::bsdiff43(&DUMMY_OLD_1[..], &DUMMY_NEW_1[..], &mut patch_1).expect("1");
        let mut output_1 = Vec::new();
        BsDiff::<B>::bspatch43(&DUMMY_OLD_1[..], &mut output_1, &mut patch_1.as_slice()).expect("2");
        assert_eq!(output_1.as_slice(), &DUMMY_NEW_1[..]);

        let mut patch_2 = Vec::new();
        BsDiff::<B>::bsdiff43(&DUMMY_OLD_2[..], &DUMMY_NEW_2[..], &mut patch_2).expect("3");
        let mut output_2 = Vec::new();
        BsDiff::<B>::bspatch43(&DUMMY_OLD_2[..], &mut output_2, &mut patch_2.as_slice()).expect("4");
        assert_eq!(output_2.as_slice(), &DUMMY_NEW_2[..]);
    }

    integration_test!(bsdiff_43);
}

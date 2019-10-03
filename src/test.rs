use crate::{Backend, BsDiff};
use rand::Rng;

macro_rules! backend_tests {
    ($d:ident) => {
        use crate::test::{hardcoded_data_t, long_random_data_t};
        #[test]
        fn hardcoded_data() {
            hardcoded_data_t::<$d>();
        }

        #[test]
        fn long_random_data() {
            long_random_data_t::<$d>()
        }
    };
}

pub fn hardcoded_data_t<B: Backend>() {
    let old_data = &[0u8, 1u8, 2u8, 3u8, 4u8, 10u8, 9u8, 255u8, 0u8, 72u8];
    let new_data = &[1u8, 0u8, 2u8, 3u8, 4u8, 10u8, 90u8, 0u8, 0u8, 255u8];

    let mut patch: Vec<u8> = Vec::new();

    BsDiff::<B>::bsdiff_raw(old_data, new_data, &mut patch).unwrap();

    let mut output_data: [u8; 10] = [0u8; 10];
    BsDiff::<B>::bspatch_raw(old_data, &mut output_data, &mut &patch[..]).unwrap();

    assert_eq!(&output_data, new_data);
}

pub fn long_random_data_t<B: Backend>() {
    let mut rng = rand_pcg::Mcg128Xsl64::new(0);
    for i in 0..10 {
        let old_data = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(10000 - i)
            .collect::<Vec<u8>>();
        let new_data = (&mut rng)
            .sample_iter(rand::distributions::Standard)
            .take(10 + 100 * i)
            .collect::<Vec<u8>>();

        let mut patch = Vec::<u8>::new();
        BsDiff::<B>::bsdiff_raw(&old_data[..], &new_data[..], &mut patch).unwrap();

        let mut output_data = vec![0u8; 10 + 100 * i];
        BsDiff::<B>::bspatch_raw(&old_data[..], &mut output_data[..], &mut &patch[..]).unwrap();

        assert_eq!(&output_data[..], &new_data[..])
    }
}

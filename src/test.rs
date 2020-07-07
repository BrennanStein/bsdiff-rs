use crate::{Backend, BsDiff};
use rand::Rng;

macro_rules! backend_tests {
    ($d:ident) => {
        use crate::test::{hardcoded_data_t, long_random_data_t, delta_data_t};
        #[test]
        fn hardcoded_data() {
            hardcoded_data_t::<$d>();
        }

        #[test]
        fn long_random_data() {
            long_random_data_t::<$d>()
        }

        #[test]
        fn delta_data() {
            delta_data_t::<$d>()
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

pub fn delta_data_t<B: Backend>() {
    let mut rng = rand_pcg::Mcg128Xsl64::new(0);
    let old_data = (&mut rng)
        .sample_iter(rand::distributions::Standard)
        .take(5000)
        .collect::<Vec<u8>>();
    let mut new_data = Vec::with_capacity(5000);
    new_data.extend_from_slice(&old_data[..2500]);
    (&mut rng)
        .sample_iter(rand::distributions::Standard)
        .take(500)
        .for_each(|b| new_data.push(b));
    &old_data[2500..].iter().map(|b| b.overflowing_add(1).0).for_each(|b| new_data.push(b));
    let mut patch = Vec::<u8>::new();
    B::bsdiff_raw(&old_data, &new_data, &mut patch).unwrap();

    let mut output = vec![0u8; new_data.len()];
    B::bspatch_raw(&old_data, &mut output, &mut &patch[..]).unwrap();

    assert_eq!(new_data, output);
}

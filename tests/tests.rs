use rand::Rng;
use rr_delta::{bsdiff_raw, bspatch_raw};

#[test]
fn hardcoded_data() {
    let old_data = &[0u8, 1u8, 2u8, 3u8, 4u8, 10u8, 9u8, 255u8, 0u8, 72u8];
    let new_data = &[1u8, 0u8, 2u8, 3u8, 4u8, 10u8, 90u8, 0u8, 0u8, 255u8];

    let mut patch: Vec<u8> = Vec::new();
    bsdiff_raw(old_data, new_data, &mut patch);

    println!("{:X?}", patch);

    let mut output_data: [u8; 10] = [0u8; 10];
    bspatch_raw(old_data, &mut output_data, &mut &patch[..]);

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
            .take(500)
            .collect::<Vec<u8>>();

        let mut patch = Vec::<u8>::new();
        bsdiff_raw(&old_data[..], &new_data[..], &mut patch);

        println!("{:X?}", &patch[..50]);

        let mut output_data = vec![0u8; 500];
        bspatch_raw(&old_data[..], &mut output_data[..], &mut &patch[..]);

        assert_eq!(&output_data[..], &new_data[..])
    }
}

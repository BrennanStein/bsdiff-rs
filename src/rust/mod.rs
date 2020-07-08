macro_rules! invalid_data {
    () => {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Patch Instructions Invalid",
        )
    };
}

mod bsdiff;
pub use bsdiff::bsdiff_raw;
mod bspatch;
pub use bspatch::bspatch_raw;

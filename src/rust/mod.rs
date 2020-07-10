macro_rules! invalid_data {
    () => {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Patch Instructions Invalid",
        )
    };
}

mod bsdiff;
pub use bsdiff::bsdiff_internal;
pub use bsdiff::bsdiff_raw;
pub use bsdiff::BsDiffRequest;
mod bspatch;
pub use bspatch::bspatch_internal;
pub use bspatch::bspatch_raw;
pub use bspatch::BsPatchRequest;

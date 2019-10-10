macro_rules! invalid_data {
    () => {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Patch Instructions Invalid",
        )
    };
}

mod bsdiff;
mod bspatch;

use crate::{Backend, BsDiffResult};
use std::io::{Read, Write};

pub struct RustBackend;

impl Backend for RustBackend {
    fn bsdiff_raw<W: Write>(old: &[u8], new: &[u8], patch: &mut W) -> BsDiffResult {
        bsdiff::bsdiff_raw(old, new, patch)
    }

    fn bspatch_raw<R: Read>(old: &[u8], new: &mut [u8], stream: &mut R) -> BsDiffResult {
        bspatch::bspatch_raw(old, new, stream)
    }
}

#[cfg(test)]
mod tests {
    use super::RustBackend;
    backend_tests!(RustBackend);
}

mod bsdiff;
mod bspatch;

pub use bsdiff::bsdiff_raw as bsdiff_raw;
pub use bspatch::bspatch_raw as bspatch_raw;

#[cfg(test)]
mod tests {
    use super::*;
    backend_tests!(bsdiff_raw, bspatch_raw);
}
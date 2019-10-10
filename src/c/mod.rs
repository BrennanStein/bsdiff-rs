use crate::BsDiffResult;
use std::io::{Read, Write};

mod bsdiff;
mod bspatch;

use crate::Backend;

pub struct CBackend;

impl Backend for CBackend {
    fn bsdiff_raw<W: Write>(old: &[u8], new: &[u8], patch: &mut W) -> BsDiffResult {
        bsdiff::bsdiff_raw(old, new, patch)
    }

    fn bspatch_raw<R: Read>(old: &[u8], new: &mut [u8], stream: &mut R) -> BsDiffResult {
        bspatch::bspatch_raw(old, new, stream)
    }
}

#[cfg(test)]
mod tests {
    use super::CBackend;
    backend_tests!(CBackend);
}

mod c_bindings {
    use std::os::raw::c_void;

    #[repr(C)]
    pub struct BsdiffStream {
        pub opaque: *mut c_void,
        pub malloc: unsafe extern "C" fn(size: usize) -> *mut c_void,
        pub free: unsafe extern "C" fn(ptr: *mut c_void),
        pub write: unsafe extern "C" fn(
            stream: *mut BsdiffStream,
            buffer: *const c_void,
            size: i32,
        ) -> i32,
    }

    #[repr(C)]
    pub struct BspatchStream {
        pub opaque: *mut c_void,
        pub read: unsafe extern "C" fn(
            stream: *const BspatchStream,
            buffer: *mut c_void,
            length: i32,
        ) -> i32,
    }

    #[link(name = "bsdiff")]
    extern "C" {
        pub fn bsdiff(
            old: *const u8,
            oldsize: i64,
            new: *const u8,
            newsize: i64,
            stream: *mut BsdiffStream,
        ) -> i32;
        pub fn bspatch(
            old: *const u8,
            oldsize: i64,
            new: *mut u8,
            newsize: i64,
            stream: *mut BspatchStream,
        ) -> i32;
    }
}

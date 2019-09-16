mod bsdiff;
mod bspatch;

pub use bsdiff::bsdiff_raw as bsdiff_raw;
pub use bspatch::bspatch_raw as bspatch_raw;

#[cfg(test)]
mod tests {
    use super::*;
    backend_tests!(bsdiff_raw, bspatch_raw);
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
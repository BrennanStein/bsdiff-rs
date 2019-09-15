//!
//! This crate is a wrapper around the bsdiff binary delta algorithm
//! It can generate a patch from old and new data, which can then be applied to the old data to generate the new data
//!
//! The original algorithm can be found here:
//! [https://github.com/mendsley/bsdiff](https://github.com/mendsley/bsdiff)
//!

extern crate libc;

use crate::bsdiff_c::{BsdiffStream, BspatchStream};
use std::io::{Read, Write};
use std::os::raw::c_void;

pub mod bsdiff_40;
pub mod bsdiff_43;
pub mod rust;

mod bsdiff_c {
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

///
/// Access to the raw bsdiff algorithm.
/// This function does not add any headers or lenght information to the patch.
///
pub fn bsdiff_raw(old: &[u8], new: &[u8], patch: &mut dyn Write) -> Result<(), i32> {
    let mut boxed_ptr = Box::from(patch);
    let raw_ptr = boxed_ptr.as_mut() as *mut &mut dyn Write;
    let mut config = BsdiffStream {
        opaque: raw_ptr as *mut c_void,
        malloc: libc::malloc,
        free: libc::free,
        write: bsdiff_write,
    };

    let exit_code = unsafe {
        bsdiff_c::bsdiff(
            old.as_ptr(),
            old.len() as i64,
            new.as_ptr(),
            new.len() as i64,
            &mut config as *mut BsdiffStream,
        )
    };

    if exit_code == 0 {
        Ok(())
    } else {
        Err(exit_code)
    }
}

unsafe extern "C" fn bsdiff_write(
    stream: *mut bsdiff_c::BsdiffStream,
    buffer: *const c_void,
    size: i32,
) -> i32 {
    let output: &mut dyn Write = *((*stream).opaque as *mut &mut dyn Write);
    let buffer: &[u8] = std::slice::from_raw_parts(buffer as *const u8, size as usize);
    match output.write_all(buffer) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{:?}", err);
            -1
        }
    }
}

///
/// Access to the raw bspatch algorithm.
/// This function does not read a header or any length information.
/// The output buffer must be the correct size.
///
pub fn bspatch_raw(old: &[u8], new: &mut [u8], patch: &mut dyn Read) -> Result<(), i32> {
    let mut boxed_ptr = Box::from(patch);
    let raw_ptr = boxed_ptr.as_mut() as *mut &mut dyn Read;
    let mut config = BspatchStream {
        opaque: raw_ptr as *mut c_void,
        read: bspatch_read,
    };

    let exit_code = unsafe {
        bsdiff_c::bspatch(
            old.as_ptr(),
            old.len() as i64,
            new.as_mut_ptr(),
            new.len() as i64,
            &mut config as *mut BspatchStream,
        )
    };

    if exit_code == 0 {
        Ok(())
    } else {
        Err(exit_code)
    }
}

unsafe extern "C" fn bspatch_read(
    stream: *const BspatchStream,
    buffer: *mut c_void,
    length: i32,
) -> i32 {
    let input: &mut dyn Read = *((*stream).opaque as *mut &mut dyn Read);
    let buffer: &mut [u8] = std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);
    match input.read_exact(buffer) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{:?}", err);
            -1
        }
    }
}

#[cfg(test)]
mod tests {}

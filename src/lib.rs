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
pub fn bsdiff_raw(old: &[u8], new: &[u8], patch: &mut Write) -> Result<(), i32> {
    let mut boxed_ptr = Box::from(patch);
    let raw_ptr = boxed_ptr.as_mut() as *mut &mut Write;
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

    match exit_code {
        0 => Ok(()),
        code => Err(code),
    }
}

unsafe extern "C" fn bsdiff_write(
    stream: *mut bsdiff_c::BsdiffStream,
    buffer: *const c_void,
    size: i32,
) -> i32 {
    let output: &mut Write = *((*stream).opaque as *mut &mut Write);
    let buffer: &[u8] = std::slice::from_raw_parts(buffer as *const u8, size as usize);
    match output.write(buffer) {
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
pub fn bspatch_raw(old: &[u8], new: &mut [u8], patch: &mut Read) -> Result<(), i32> {
    let mut boxed_ptr = Box::from(patch);
    let raw_ptr = boxed_ptr.as_mut() as *mut &mut Read;
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

    match exit_code {
        0 => Ok(()),
        code => Err(code),
    }
}

unsafe extern "C" fn bspatch_read(
    stream: *const BspatchStream,
    buffer: *mut c_void,
    length: i32,
) -> i32 {
    let input: &mut Read = *((*stream).opaque as *mut &mut Read);
    let buffer: &mut [u8] = std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);
    match input.read(buffer) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{:?}", err);
            -1
        }
    }
}

#[cfg(test)]
mod tests {}

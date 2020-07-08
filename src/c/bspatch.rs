use super::c_bindings;
use crate::BsDiffResult;
use std::io::{Error, ErrorKind, Read};
use std::os::raw::c_void;

///
/// Access to the raw bspatch algorithm.
/// This function does not read a header or any length information.
/// The output buffer must be the correct size.
///
pub fn bspatch_raw<R: Read>(old: &[u8], new: &mut [u8], mut patch: R) -> BsDiffResult {
    let patch = &mut patch as &mut dyn Read;
    let mut boxed_ptr = Box::from(patch);
    let raw_ptr = boxed_ptr.as_mut() as *mut &mut dyn Read;
    let mut config = c_bindings::BspatchStream {
        opaque: raw_ptr as *mut c_void,
        read: bspatch_read,
    };

    let exit_code = unsafe {
        c_bindings::bspatch(
            old.as_ptr(),
            old.len() as i64,
            new.as_mut_ptr(),
            new.len() as i64,
            &mut config as *mut c_bindings::BspatchStream,
        )
    };

    if exit_code == 0 {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("C code returned {}", exit_code),
        ))
    }
}

unsafe extern "C" fn bspatch_read(
    stream: *const c_bindings::BspatchStream,
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

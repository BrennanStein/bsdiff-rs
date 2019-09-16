use std::io::Write;
use std::os::raw::c_void;
use super::c_bindings;

///
/// Access to the raw bsdiff algorithm.
/// This function does not add any headers or lenght information to the patch.
///
pub fn bsdiff_raw(old: &[u8], new: &[u8], patch: &mut dyn Write) -> Result<(), i32> {
    let mut boxed_ptr = Box::from(patch);
    let raw_ptr = boxed_ptr.as_mut() as *mut &mut dyn Write;
    let mut config = c_bindings::BsdiffStream {
        opaque: raw_ptr as *mut c_void,
        malloc: libc::malloc,
        free: libc::free,
        write: bsdiff_write,
    };

    let exit_code = unsafe {
        c_bindings::bsdiff(
            old.as_ptr(),
            old.len() as i64,
            new.as_ptr(),
            new.len() as i64,
            &mut config as *mut c_bindings::BsdiffStream,
        )
    };

    if exit_code == 0 {
        Ok(())
    } else {
        Err(exit_code)
    }
}

unsafe extern "C" fn bsdiff_write(
    stream: *mut c_bindings::BsdiffStream,
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
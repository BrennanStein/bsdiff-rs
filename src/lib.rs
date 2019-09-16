//!
//! This crate is a wrapper around the bsdiff binary delta algorithm
//! It can generate a patch from old and new data, which can then be applied to the old data to generate the new data
//!
//! The original algorithm can be found here:
//! [https://github.com/mendsley/bsdiff](https://github.com/mendsley/bsdiff)
//!

extern crate libc;

use std::io::{Read, Write};
use byteorder::{WriteBytesExt, LittleEndian, ReadBytesExt};
use bzip2::Compression;
use bzip2::write::BzEncoder;
use bzip2::read::BzDecoder;

#[cfg_attr(all(features = "rust_backend", not(test)), path = "rust/mod.rs")]
#[cfg_attr(all(not(features = "rust_backend"), not(test)), path = "c/mod.rs")]
#[cfg_attr(test, path = "test_backend.rs")]
#[macro_use]
pub mod raw;

#[cfg(test)]
pub mod rust;

#[cfg(test)]
pub mod c;

const MAGIC_NUMBER_BSDIFF_43: &str = "ENDSLEY/BSDIFF43";

pub fn bsdiff_43<W: Write>(old: &[u8], new: &[u8], patch: &mut W) -> Result<(), i32> {
    patch.write_all(MAGIC_NUMBER_BSDIFF_43.as_bytes()).unwrap();
    patch.write_u64::<LittleEndian>(new.len() as u64).unwrap();
    let mut compress = BzEncoder::new(patch, Compression::Best);
    let exit_code = raw::bsdiff_raw(old, new, &mut compress);
    compress.finish().unwrap();
    exit_code
}

pub fn bspatch_43<W: Write, R: Read>(old: &[u8], new: &mut W, patch: &mut R) -> Result<(), i32> {
    let mut header = [0u8; 16];
    patch.read_exact(&mut header).unwrap();
    assert_eq!(&header, MAGIC_NUMBER_BSDIFF_43.as_bytes());
    let new_size = patch.read_u64::<LittleEndian>().unwrap();
    let mut new_buffer = vec![0u8; new_size as usize];
    let mut decompress = BzDecoder::new(patch);
    let stream_ptr: &mut dyn Read = &mut decompress;
    let exit_code = raw::bspatch_raw(old, &mut new_buffer[..], stream_ptr);
    if let Ok(()) = exit_code {
        new.write_all(&mut new_buffer[..]).unwrap();
    };
    exit_code
}

#[allow(dead_code)]
const MAGIC_NUMBER_BSDIFF_40: &str = "BSDIFF40";

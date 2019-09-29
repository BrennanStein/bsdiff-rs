//!
//! This crate is a wrapper around the bsdiff binary delta algorithm
//! It can generate a patch from old and new data, which can then be applied to the old data to generate the new data
//!
//! The original algorithm can be found here:
//! [https://github.com/mendsley/bsdiff](https://github.com/mendsley/bsdiff)
//!

extern crate libc;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use std::io::{Read, Write};

macro_rules! invalid_data {
    () => {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Patch Instructions Invalid",
        )
    };
}

#[cfg(test)]
#[macro_use]
pub mod test;

#[cfg_attr(not(feature = "rust_backend"), path = "c/mod.rs")]
#[cfg_attr(feature = "rust_backend", path = "rust/mod.rs")]
pub mod raw;

pub type BsDiffResult = std::io::Result<()>;

const MAGIC_NUMBER_BSDIFF_43: &str = "ENDSLEY/BSDIFF43";

pub fn bsdiff_43<W: Write>(old: &[u8], new: &[u8], patch: &mut W) -> BsDiffResult {
    patch.write_all(MAGIC_NUMBER_BSDIFF_43.as_bytes()).unwrap();
    patch.write_u64::<LittleEndian>(new.len() as u64).unwrap();
    let mut compress = BzEncoder::new(patch, Compression::Best);
    raw::bsdiff_raw(old, new, &mut compress)?;
    compress.finish()?;
    Ok(())
}

pub fn bspatch_43<W: Write, R: Read>(old: &[u8], new: &mut W, patch: &mut R) -> BsDiffResult {
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

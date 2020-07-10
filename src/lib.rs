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

#[path = "c/mod.rs"]
#[cfg(feature = "c_backend")]
mod backend;

#[path = "rust/mod.rs"]
#[cfg(not(feature = "c_backend"))]
mod backend;

#[inline]
pub fn bsdiff_raw<W: Write>(old: &[u8], new: &[u8], patch: W) -> BsDiffResult<()> {
    backend::bsdiff_raw(old, new, patch)
}

#[inline]
pub fn bspatch_raw<R: Read>(old: &[u8], new: &mut [u8], patch: R) -> BsDiffResult<()> {
    backend::bspatch_raw(old, new, patch)
}

#[cfg(not(feature = "c_backend"))]
use backend::{bsdiff_internal, bspatch_internal, BsDiffRequest, BsPatchRequest};

pub type BsDiffResult<D> = std::io::Result<D>;

pub fn bsdiff43<W: Write>(old: &[u8], new: &[u8], mut patch: W) -> BsDiffResult<()> {
    patch.write_all(MAGIC_NUMBER_BSDIFF_43.as_bytes()).unwrap();
    patch.write_u64::<LittleEndian>(new.len() as u64).unwrap();
    let mut compress = BzEncoder::new(patch, Compression::Best);
    bsdiff_raw(old, new, &mut compress)?;
    compress.finish()?;
    Ok(())
}

pub fn bsdiff43_vec(old: &[u8], new: &[u8]) -> BsDiffResult<Vec<u8>> {
    let mut patch = Vec::new();
    bsdiff43(old, new, &mut patch)?;
    Ok(patch)
}

const MAGIC_NUMBER_BSDIFF_43: &str = "ENDSLEY/BSDIFF43";

pub fn bspatch43<W: Write, R: Read>(old: &[u8], mut new: W, mut patch: R) -> BsDiffResult<()> {
    let mut header = [0u8; 16];
    patch.read_exact(&mut header).unwrap();
    assert_eq!(&header, MAGIC_NUMBER_BSDIFF_43.as_bytes());
    let new_size = patch.read_u64::<LittleEndian>().unwrap();
    let mut new_buffer = vec![0u8; new_size as usize];
    let mut decompress = BzDecoder::new(patch);
    let exit_code = bspatch_raw(old, &mut new_buffer[..], &mut decompress);
    if let Ok(()) = exit_code {
        new.write_all(&mut new_buffer[..]).unwrap();
    };
    exit_code
}

pub fn bspatch43_vec<R: Read>(old: &[u8], patch: R) -> BsDiffResult<Vec<u8>> {
    let mut new = Vec::new();
    bspatch43(old, &mut new, patch)?;
    Ok(new)
}

#[cfg(not(feature = "c_backend"))]
const MAGIC_NUMBER_BSDIFF_40: &str = "BSDIFF40";

#[cfg(not(feature = "c_backend"))]
struct JBsDiffStreams<S> {
    pub ctrl_stream: S,
    pub diff_stream: S,
    pub extra_stream: S,
}

#[cfg(not(feature = "c_backend"))]
pub fn jbsdiff40<W: Write>(old: &[u8], new: &[u8], mut patch: W) -> BsDiffResult<()> {
    let mut ctrl_data = Vec::new();
    let mut diff_data = Vec::new();
    let mut extra_data = Vec::new();

    {
        let streams = JBsDiffStreams {
            ctrl_stream: BzEncoder::new(&mut ctrl_data, Compression::Best),
            diff_stream: BzEncoder::new(&mut diff_data, Compression::Best),
            extra_stream: BzEncoder::new(&mut extra_data, Compression::Best),
        };

        let req = BsDiffRequest {
            data: streams,
            ctrl_stream: |data, buffer| data.ctrl_stream.write_all(buffer),
            diff_stream: |data, buffer| data.diff_stream.write_all(buffer),
            extra_stream: |data, buffer| data.extra_stream.write_all(buffer),
        };

        bsdiff_internal(old, new, req)?;
    }

    patch.write_all(MAGIC_NUMBER_BSDIFF_40.as_bytes())?;
    patch.write_u64::<LittleEndian>(ctrl_data.len() as u64)?;
    patch.write_u64::<LittleEndian>(diff_data.len() as u64)?;
    patch.write_u64::<LittleEndian>(new.len() as u64)?;

    patch.write_all(&ctrl_data)?;
    patch.write_all(&diff_data)?;
    patch.write_all(&extra_data)?;

    Ok(())
}

#[cfg(not(feature = "c_backend"))]
pub fn jbsdiff40_vec(old: &[u8], new: &[u8]) -> BsDiffResult<Vec<u8>> {
    let mut patch = Vec::new();
    jbsdiff40(old, new, &mut patch)?;
    Ok(patch)
}

#[cfg(not(feature = "c_backend"))]
pub fn jbspatch40<W: Write, R: Read>(old: &[u8], new: W, mut patch: R) -> BsDiffResult<()> {
    let mut header = [0u8; 32];
    patch.read_exact(&mut header).unwrap();
    assert_eq!(&header[..8], MAGIC_NUMBER_BSDIFF_40.as_bytes());
    let mut header_iter = &header[8..];

    let ctrl_len = header_iter.read_u64::<LittleEndian>().unwrap() as usize;
    let mut ctrl_data = vec![0u8; ctrl_len].into_boxed_slice();
    patch.read_exact(&mut ctrl_data).unwrap();
    let ctrl_stream = BzDecoder::new(&*ctrl_data);

    let diff_len = header_iter.read_u64::<LittleEndian>().unwrap() as usize;
    let mut diff_data = vec![0u8; diff_len].into_boxed_slice();
    patch.read_exact(&mut diff_data).unwrap();
    let diff_stream = BzDecoder::new(&*diff_data);

    let mut extra_data = Vec::new();
    patch.read_to_end(&mut extra_data).unwrap();
    let extra_stream = BzDecoder::new(&*extra_data);
    let out_len = header_iter.read_u64::<LittleEndian>().unwrap() as usize;

    let streams = JBsDiffStreams {
        ctrl_stream,
        diff_stream,
        extra_stream,
    };

    let req = BsPatchRequest {
        data: streams,
        ctrl_stream: |data, buffer| data.ctrl_stream.read_exact(buffer),
        diff_stream: |data, buffer| data.diff_stream.read_exact(buffer),
        extra_stream: |data, buffer| data.extra_stream.read_exact(buffer),
    };

    bspatch_internal(old, new, out_len, req)?;
    Ok(())
}

#[cfg(not(feature = "c_backend"))]
pub fn jbspatch40_vec<R: Read>(old: &[u8], patch: R) -> BsDiffResult<Vec<u8>> {
    let mut new = Vec::new();
    jbspatch40(old, &mut new, patch)?;
    Ok(new)
}

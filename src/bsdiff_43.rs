use crate::{bsdiff_raw, bspatch_raw};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use std::io::{Read, Write};

const MAGIC_NUMBER: &str = "ENDSLEY/BSDIFF43";

pub fn bsdiff<W: Write>(old: &[u8], new: &[u8], patch: &mut W) -> Result<(), i32> {
    patch.write_all(MAGIC_NUMBER.as_bytes()).unwrap();
    patch.write_u64::<LittleEndian>(new.len() as u64).unwrap();
    let mut compress = BzEncoder::new(patch, Compression::Best);
    let exit_code = bsdiff_raw(old, new, &mut compress);
    compress.finish().unwrap();
    exit_code
}

pub fn bspatch<W: Write, R: Read>(old: &[u8], new: &mut W, patch: &mut R) -> Result<(), i32> {
    let mut header = [0u8; 16];
    patch.read_exact(&mut header).unwrap();
    assert_eq!(&header, MAGIC_NUMBER.as_bytes());
    let new_size = patch.read_u64::<LittleEndian>().unwrap();
    let mut new_buffer = vec![0u8; new_size as usize];
    let mut decompress = BzDecoder::new(patch);
    let exit_code = bspatch_raw(old, &mut new_buffer[..], &mut decompress);
    if let Ok(()) = exit_code {
        new.write_all(&mut new_buffer[..]).unwrap();
    };
    exit_code
}

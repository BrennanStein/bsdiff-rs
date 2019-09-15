use std::io::{Read, Write};
use byteorder::{ReadBytesExt, LittleEndian};
use bzip2::read::BzDecoder;

fn bspatch_raw(old: &[u8], new: &mut [u8], stream: &mut dyn Read) -> Result<(), i32> {
    let mut buf = [0u8; 8];
    let mut oldpos: usize = 0;
    let mut newpos: usize = 0;
    let mut ctrl = [0i64; 3];

    while newpos < new.len() {
       for i in 0..3 {
           ctrl[i] = stream.read_i64::<LittleEndian>().unwrap();
       }

        if newpos + ctrl[0] as usize > new.len() {
            return Err(-1);
        }

        stream.read_exact(&mut new[newpos..(newpos + ctrl[0] as usize)]).unwrap();

        for i in 0..(ctrl[0] as usize) {
            if oldpos + i >= 0 && oldpos + i < old.len() {
                new[newpos + i] += old[oldpos + i];
            }
        }

        newpos += ctrl[0] as usize;
        oldpos += ctrl[0] as usize;

        if newpos + ctrl[1] as usize > new.len() {
            return Err(-1);
        }

        stream.read_exact(&mut new[newpos..(newpos + ctrl[1] as usize)]).unwrap();

        newpos += ctrl[1] as usize;
        oldpos += ctrl[2] as usize;
    }

    Ok(())
}

const MAGIC_NUMBER: &str = "ENDSLEY/BSDIFF43";

pub fn bspatch(old: &[u8], new: &mut Write, patch: &mut Read) -> Result<(), i32> {
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

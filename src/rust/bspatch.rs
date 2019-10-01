use crate::BsDiffResult;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

pub fn bspatch_raw(old: &[u8], new: &mut [u8], stream: &mut dyn Read) -> BsDiffResult {
    let mut oldpos: usize = 0;
    let mut newpos: usize = 0;
    let mut ctrl = [0i64; 3];

    while newpos < new.len() {
        for i in 0..3 {
            ctrl[i] = stream.read_i64::<LittleEndian>()?;
        }

        if newpos + ctrl[0] as usize > new.len() {
            return Err(invalid_data!());
        }

        stream.read_exact(&mut new[newpos..(newpos + ctrl[0] as usize)])?;

        for i in 0..(ctrl[0] as usize) {
            if oldpos + i < old.len() {
                new[newpos + i] = new[newpos + i].overflowing_add(old[oldpos + i]).0;
            }
        }

        newpos = (newpos as i64 + ctrl[0]) as usize;
        oldpos = (oldpos as i64 + ctrl[0]) as usize;

        if newpos + ctrl[1] as usize > new.len() {
            return Err(invalid_data!());
        }

        stream.read_exact(&mut new[newpos..(newpos + ctrl[1] as usize)])?;

        newpos = (newpos as i64 + ctrl[1]) as usize;
        oldpos = (oldpos as i64 + ctrl[2]) as usize;
    }

    Ok(())
}

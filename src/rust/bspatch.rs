use std::io::Read;
use byteorder::{ReadBytesExt, LittleEndian};

pub fn bspatch_raw(old: &[u8], new: &mut [u8], stream: &mut dyn Read) -> Result<(), i32> {
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
            if oldpos + i < old.len() {
                new[newpos + i] = new[newpos + i].overflowing_add(old[oldpos + i]).0;
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
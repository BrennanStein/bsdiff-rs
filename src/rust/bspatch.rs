use crate::BsDiffResult;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

pub struct BsPatchRequest<D> {
    pub data: D,
    pub ctrl_stream: fn(&mut D, &mut [u8]) -> BsDiffResult<()>,
    pub diff_stream: fn(&mut D, &mut [u8]) -> BsDiffResult<()>,
    pub extra_stream: fn(&mut D, &mut [u8]) -> BsDiffResult<()>,
}

pub fn bspatch_internal<D>(old: &[u8], new: &mut [u8], req: BsPatchRequest<D>) -> BsDiffResult<D> {
    let BsPatchRequest {
        mut data,
        ctrl_stream,
        diff_stream,
        extra_stream,
    } = req;
    let mut oldpos: usize = 0;
    let mut newpos: usize = 0;
    let mut ctrl_buff = [0u8; 3 * 8];
    let mut ctrl = [0i64; 3];

    while newpos < new.len() {
        ctrl_stream(&mut data, &mut ctrl_buff)?;
        let mut ctrl_buff_stream: &[u8] = &ctrl_buff;
        for i in 0..3 {
            ctrl[i] = ctrl_buff_stream.read_i64::<LittleEndian>()?;
        }

        if newpos + ctrl[0] as usize > new.len() {
            return Err(invalid_data!());
        }

        diff_stream(&mut data, &mut new[newpos..(newpos + ctrl[0] as usize)])?;

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

        extra_stream(&mut data, &mut new[newpos..(newpos + ctrl[1] as usize)])?;

        newpos = (newpos as i64 + ctrl[1]) as usize;
        oldpos = (oldpos as i64 + ctrl[2]) as usize;
    }

    Ok(data)
}

pub fn bspatch_raw<R: Read>(old: &[u8], new: &mut [u8], patch: R) -> BsDiffResult<()> {
    let stream_fn: fn(&mut R, &mut [u8]) -> BsDiffResult<()> =
        |patch, buffer| patch.read_exact(buffer);
    let req = BsPatchRequest {
        data: patch,
        ctrl_stream: stream_fn,
        diff_stream: stream_fn,
        extra_stream: stream_fn,
    };

    bspatch_internal(old, new, req)?;
    Ok(())
}

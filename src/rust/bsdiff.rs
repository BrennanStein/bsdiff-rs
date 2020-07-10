#![allow(non_snake_case)]

use crate::BsDiffResult;
use byteorder::{LittleEndian, WriteBytesExt};
use std::cmp::{min, Ordering};
use std::io::Write;

fn split(I: &mut [isize], V: &mut [isize], start: isize, len: isize, h: isize) {
    if len < 16 {
        let mut k = start;
        while k < start + len {
            let mut j = 1;
            let mut x = V[(I[k as usize] + h) as usize];
            let mut i = 1;
            while k + i < start + len {
                let value = &V[(I[(k + i) as usize] + h) as usize];
                if *value < x {
                    x = *value;
                    j = 0;
                }
                if *value == x {
                    I.swap((k + j) as usize, (k + i) as usize);
                    j += 1;
                }
                i += 1;
            }

            for i in 0..j {
                V[(I[(k + i) as usize]) as usize] = k + j - 1;
                if j == 1 {
                    I[k as usize] = -1;
                }
            }
            k += j;
        }
        return;
    }

    let x = V[(I[(start + len / 2) as usize] + h) as usize];
    let mut jj = 0;
    let mut kk = 0;
    for i in start..(start + len) {
        match V[(I[i as usize] + h) as usize].cmp(&x) {
            Ordering::Less => jj += 1,
            Ordering::Equal => kk += 1,
            Ordering::Greater => {}
        }
    }
    jj += start;
    kk += jj;

    let mut i = start;
    let mut j = 0;
    let mut k = 0;
    while i < jj {
        match V[(I[i as usize] + h) as usize].cmp(&x) {
            Ordering::Less => i += 1,
            Ordering::Equal => {
                I.swap(i as usize, (j + jj) as usize);
                j += 1;
            }
            Ordering::Greater => {
                I.swap(i as usize, (k + kk) as usize);
                k += 1;
            }
        }
    }

    while jj + j < kk {
        if V[(I[(jj + j) as usize] + h) as usize] == x {
            j += 1;
        } else {
            I.swap((jj + j) as usize, (kk + k) as usize);
            k += 1;
        }
    }

    if jj > start {
        split(I, V, start, jj - start, h)
    }
    for i in 0..(kk - jj) {
        V[I[(jj + i) as usize] as usize] = kk - 1
    }

    if jj == kk - 1 {
        I[jj as usize] = -1
    }
    if start + len > kk {
        split(I, V, kk, start + len - kk, h)
    }
}

fn qsufsort(I: &mut [isize], V: &mut [isize], old: &[u8]) {
    let buckets: &mut [isize] = &mut [0; 256];

    // each index n is the frequency that the u8 value n occurs in old
    for i in 0..old.len() {
        buckets[old[i] as usize] += 1
    }

    // adds the previous index
    // index n is the cumulative frequency of the u8 value n and all smaller values
    // assert_eq!(buckets[255], old.len() as i64);
    for i in 1..256 {
        buckets[i] += buckets[i - 1]
    }

    // right shift 1 element, buckets[0] becomes 0
    for i in (1..256).rev() {
        buckets[i] = buckets[i - 1]
    }
    buckets[0] = 0;

    // first step:
    // buckets[n] is now cumulative frequency again because right shift + adding frequency of n
    // means buckets[n] = cumulative of n - 1 + frequency of n => cumulative function again
    //
    // second step:
    // I[n] is the index of the old array which contains the byte with the cumulative frequency of n
    // in other words, `for x in 1..=old.len() {old[I[x]]}` returns old's elements in sorted order
    // when n the sorted index, I[n + 1] is the index of old
    for i in 0..old.len() {
        buckets[old[i] as usize] += 1;
        I[buckets[old[i] as usize] as usize] = i as isize;
    }
    I[0] = old.len() as isize;

    // V[i] is the inverse of I[i], when i is old's index, V[i] the sorted index
    for i in 0..old.len() {
        V[i] = buckets[old[i] as usize]
    }
    V[old.len()] = 0;

    // if I[n] points to a unique byte value of old, now it points to -1
    for i in 1..256 {
        if buckets[i] == buckets[i - 1] + 1 {
            I[buckets[i] as usize] = -1
        }
    }
    I[0] = -1;

    let mut h = 1;
    while I[0] != -(old.len() as isize + 1) {
        let mut len: usize = 0;
        let mut i: usize = 0;
        while i <= old.len() {
            if I[i] < 0 {
                len += (-I[i]) as usize;
                i += (-I[i]) as usize;
            } else {
                if len != 0 {
                    I[i - len] = -(len as isize);
                }
                len = V[I[i] as usize] as usize + 1 - i;
                split(I, V, i as isize, len as isize, h);
                i += len;
                len = 0;
            }
        }
        if len != 0 {
            I[i - len] = -(len as isize);
        }

        h += h;
    }

    for i in 0..=old.len() {
        I[V[i] as usize] = i as isize
    }
}

fn matchlen(old: &[u8], new: &[u8]) -> i64 {
    let mut i = 0;
    let min_length = min(old.len(), new.len());
    while i < min_length {
        if old[i] != new[i] {
            break;
        }
        i += 1;
    }
    return i as i64;
}

fn search(I: &[isize], old: &[u8], new: &[u8], start: usize, end: usize, pos: &mut isize) -> i64 {
    if end - start < 2 {
        let x = matchlen(&old[(I[start] as usize)..], new);
        let y = matchlen(&old[(I[end] as usize)..], new);

        if x > y {
            *pos = I[start];
            x
        } else {
            *pos = I[end];
            y
        }
    } else {
        let middle = start + (end - start) / 2;
        let slice_len = min(old.len() - I[middle] as usize, new.len());
        let lhs = &old[(I[middle] as usize)..(I[middle] as usize + slice_len)];
        let rhs = &new[..slice_len];
        if lhs < rhs {
            search(I, old, new, middle, end, pos)
        } else {
            search(I, old, new, start, middle, pos)
        }
    }
}

pub struct BsDiffRequest<D> {
    pub data: D,
    pub ctrl_stream: fn(&mut D, &[u8]) -> BsDiffResult<()>,
    pub diff_stream: fn(&mut D, &[u8]) -> BsDiffResult<()>,
    pub extra_stream: fn(&mut D, &[u8]) -> BsDiffResult<()>,
}

pub fn bsdiff_internal<D>(old: &[u8], new: &[u8], req: BsDiffRequest<D>) -> BsDiffResult<D> {
    let BsDiffRequest {
        mut data,
        ctrl_stream,
        diff_stream,
        extra_stream,
    } = req;
    let V: &mut [isize] = &mut *vec![0isize; old.len() + 1];
    let I: &mut [isize] = &mut *vec![0isize; old.len() + 1];

    qsufsort(I, V, old);

    let buffer: &mut [u8] = &mut *vec![0u8; new.len()];

    // Compute the differences, writing ctrl as we go
    let mut scan = 0;
    let mut len = 0;
    let mut pos = 0;
    let mut lastscan = 0;
    let mut lastpos = 0;
    let mut lastoffset: isize = 0;
    while scan < new.len() {
        let mut oldscore = 0;
        scan += len;
        let mut scsc = scan;
        while scan < new.len() {
            len = search(I, old, &new[scan..], 0, old.len(), &mut pos) as usize;

            while scsc < scan + len {
                if scsc as isize + lastoffset < old.len() as isize
                    && old[(scsc as isize + lastoffset) as usize] == new[scsc]
                {
                    oldscore += 1
                }
                scsc += 1;
            }

            if len == oldscore && len != 0 || len > oldscore + 8 {
                break;
            }
            if scan as isize + lastoffset < old.len() as isize
                && old[(scan as isize + lastoffset) as usize] == new[scan]
            {
                oldscore -= 1
            }

            scan += 1;
        }

        if len != oldscore || scan == new.len() {
            let mut s = 0;
            let mut Sf = 0;
            let mut lenf = 0;
            let mut i = 0;
            while lastscan + i < scan && lastpos + i < old.len() {
                if old[lastpos + i] == new[lastscan + i] {
                    s += 1
                }
                i += 1;
                if s as i64 * 2 - i as i64 > Sf as i64 * 2 - lenf as i64 {
                    Sf = s;
                    lenf = i;
                }
            }

            let mut lenb = 0;
            if scan < new.len() {
                let mut s = 0;
                let mut Sb = 0;
                let mut i = 1;
                while scan >= lastscan + i && pos as usize >= i {
                    if old[pos as usize - i] == new[scan - i] {
                        s += 1
                    }
                    if (s as i64) * 2 - (i as i64) > (Sb as i64) * 2 - (lenb as i64) {
                        Sb = s;
                        lenb = i;
                    }

                    i += 1;
                }
            }

            if lastscan + lenf > scan - lenb {
                let overlap = (lastscan + lenf) - (scan - lenb);
                let mut s = 0;
                let mut Ss = 0;
                let mut lens = 0;

                for i in 0..overlap {
                    if new[lastscan + lenf - overlap + i] == old[lastpos + lenf - overlap + i] {
                        s += 1
                    }
                    if new[scan - lenb + i] == old[pos as usize - lenb + i] {
                        s -= 1
                    }
                    if s > Ss {
                        Ss = s;
                        lens = i + 1;
                    }
                }

                lenf += lens - overlap;
                lenb -= lens;
            }

            // Write Control Data
            let mut ctrl_buffer = [0u8; 3 * 8];
            let mut ctrl_write: &mut [u8] = &mut ctrl_buffer;
            ctrl_write.write_i64::<LittleEndian>(lenf as i64)?;
            ctrl_write
                .write_i64::<LittleEndian>((scan - lenb) as i64 - (lastscan + lenf) as i64)?;
            ctrl_write.write_i64::<LittleEndian>(
                (pos as i64 - lenb as i64) - (lastpos as i64 + lenf as i64),
            )?;
            ctrl_stream(&mut data, &ctrl_buffer)?;

            // Write Diff Data
            for i in 0..lenf {
                buffer[i] = new[lastscan + i].wrapping_sub(old[lastpos + i]);
            }
            diff_stream(&mut data, &buffer[..lenf])?;

            // Write Extra Data
            for i in 0..((scan - lenb) - (lastscan + lenf)) {
                buffer[i] = new[lastscan + lenf + i];
            }
            extra_stream(&mut data, &buffer[..((scan - lenb) - (lastscan + lenf))])?;

            if scan < new.len() {
                lastscan = scan - lenb;
                lastpos = pos as usize - lenb;
                lastoffset = pos as isize - scan as isize;
            }
        }
    }

    Ok(data)
}

pub fn bsdiff_raw<W: Write>(old: &[u8], new: &[u8], patch: W) -> BsDiffResult<()> {
    let stream_fn: fn(&mut W, &[u8]) -> BsDiffResult<()> =
        |patch: &mut W, buffer| patch.write_all(buffer);
    let req = BsDiffRequest {
        data: patch,
        ctrl_stream: stream_fn,
        diff_stream: stream_fn,
        extra_stream: stream_fn,
    };

    bsdiff_internal(old, new, req)?;
    Ok(())
}

#![doc = include_str!("../README.md")]

mod stdout_raw;

use std::{
    env,
    fs::File,
    io::{self, BufWriter, Read, Write},
};

const WORD_COUNT: usize = 1_000_000;
const _WORD_LEN_MIN: usize = 5;
const WORD_LEN_MAX: usize = 20;
const CHAR_RADIX: u128 = 32; // PERF: set radix to 32 for faster hashing

/// Shuffles the given array with the steps described in [`README.md`](index.html).
///
/// # Panics
///
/// Input array should have an even length (`N % 2 == 0`).
fn shuffle<T, const N: usize>(mut array: [T; N]) -> [T; N] {
    assert!(N % 2 == 0, "input array should have an even length");

    // 1. swap each `2K`th element with `2K+1`th element
    for chunk in array.chunks_mut(2) {
        chunk.swap(0, 1);
    }

    // 2. swap the first half with the last half
    let (first, last) = array.split_at_mut(N / 2);
    first.swap_with_slice(last);

    array
}

/// Indices to values, values to indices. `a[i] = v` to `b[v] = i`.
///
/// Note: Indices not specified in the input array are not intended to be accessed
/// and will be filled with `0xFF` to make it easier to spot out mistakes.
///
/// # Panics
///
/// Input array should not contain a value bigger than output array's length.
fn transpose<const N: usize, const K: usize>(array: &[u8; N]) -> [u8; K] {
    let mut transposed = [u8::MAX; K];
    for (index, value) in array.iter().enumerate() {
        transposed[*value as usize] = index as u8;
    }
    transposed
}


#[derive(Debug)]
struct Word<'a> {
    hash: u128,
    source: &'a [u8],
}

fn main() -> Result<(), io::Error> {
    let alphabets = b"abcdefghijklmnopqrstuvwxyz";
    let sort_order = shuffle(*alphabets);
    let lookup_table: [u8; 'a' as usize + 1] = transpose(&sort_order);

    let sort_key = |b: &u8| {
        debug_assert!(
            b.is_ascii_lowercase(),
            "input byte should be an ASCII lowercase character"
        );

        unsafe { lookup_table.get_unchecked(*b as usize) }
    };

    let input = {
        // argv[1] or "input.txt" by default
        let mut file = match env::args_os().nth(1) {
            Some(path) => File::open(path)?,
            None => File::open("input.txt")?,
        };
        let fsize = file.metadata()?.len() as usize;

        // reserve 1 extra byte for newline
        let mut buf = Vec::with_capacity(fsize + 1);
        file.read_to_end(&mut buf)?;

        // make sure the input ends with a newline
        // for technical reasons I'm too lazy to elaborate
        if buf.last() != Some(&b'\n') {
            buf.push(b'\n');
        }

        buf
    };

    // let mut words = Vec::with_capacity(WORD_COUNT);
    // words.extend(WordIter(&input));
    // words.sort_unstable_by_key(|w| w.hash);

    // // let stdout = io::stdout().lock();
    // let stdout = stdout_raw::stdout_raw();
    // let mut writer = BufWriter::new(stdout);
    // for word in words {
    //     writer.write(word.source)?;
    // }
    // writer.flush()?;

    Ok(())
}

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
const fn shuffle<const N: usize>(mut arr: [u8; N]) -> [u8; N] {
    assert!(N % 2 == 0, "input array should have an even length");

    // 1. swap each `2K`th element with `2K+1`th element
    let mut i = 0;
    while i < N {
        (arr[i], arr[i + 1]) = (arr[i + 1], arr[i]);
        i += 2;
    }

    // 2. swap the first half with the last half
    let mut i = 0;
    while i < N / 2 {
        (arr[i], arr[N / 2 + i]) = (arr[N / 2 + i], arr[i]);
        i += 1;
    }

    arr
}

/// Indices to values, values to indices. `a[i] = v` to `b[v] = i`.
///
/// Note: Indices not specified in the input array are not intended to be accessed
/// and will be filled with `0xFF` to make it easier to spot out mistakes.
///
/// # Panics
///
/// Input array should not contain a value bigger than output array's length.
const fn transpose<const A: usize, const B: usize>(arr: &[u8; A]) -> [u8; B] {
    let mut transposed = [u8::MAX; B];
    let mut i = 0;
    while i < A {
        let (index, value) = (i, arr[i]);
        transposed[value as usize] = index as u8;
        i += 1;
    }
    transposed
}

/// Shuffled one-to-one mapping of ASCII lowercase characters.
///
/// # Panics
///
/// Input byte should be an ASCII lowercase character.
#[inline]
fn sort_key(c: &u8) -> u8 {
    debug_assert!(
        c.is_ascii_lowercase(),
        "input byte should be an ASCII lowercase character"
    );

    const SORT_ORDER: [u8; 26] = shuffle(*b"abcdefghijklmnopqrstuvwxyz");
    const SORT_KEY: [u8; 'a' as usize + 26] = transpose(&SORT_ORDER);

    // SAFETY: it's not safe lol
    unsafe {
        // lookup table is padded with 'a' bytes, no need to extract the offset.
        // the character can be directly used as an index
        *SORT_KEY.get_unchecked(*c as usize)
    }
}

#[derive(Debug)]
struct Word<'a> {
    hash: u128,
    source: &'a [u8],
}

#[derive(Debug)]
struct WordIter<'a>(&'a [u8]);

impl<'a> Iterator for WordIter<'a> {
    type Item = Word<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        let mut iter = self.0.iter();

        // FIX: get both the len and hash from the iterator
        // | `let (len, hash) = iter.magic();`
        let mut hash: u128 = 0;
        let len = iter
            .by_ref()
            .take_while(|b| **b != b'\n')
            .inspect(|b| hash = hash * CHAR_RADIX + sort_key(b) as u128)
            .count();
        hash *= CHAR_RADIX.pow((WORD_LEN_MAX - len) as u32);

        let word = Word {
            hash,
            source: &self.0[..(len + 1)],
        };

        // NOTE: `take_while()` consumes the delimiter (b'\n')
        // | and `iter` now points to the first byte of the next word
        self.0 = iter.as_slice();

        Some(word)
    }
}

fn main() -> Result<(), io::Error> {
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

    let mut words = Vec::with_capacity(WORD_COUNT);
    words.extend(WordIter(&input));
    words.sort_unstable_by_key(|w| w.hash);

    // let stdout = io::stdout().lock();
    let stdout = stdout_raw::stdout_raw();
    let mut writer = BufWriter::new(stdout);
    for word in words {
        writer.write(word.source)?;
    }
    writer.flush()?;

    Ok(())
}

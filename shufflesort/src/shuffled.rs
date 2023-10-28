//! Dealt with the shuffled sorting order using a lookup table.
//!
//! Past tense because everything happened in compile time, through excessive usage of `const`.
//!
//! # Was It Worth It
//!
//! Now, I *could* have...
//!
//! 1. Hardcode the lookup table like a normal person
//! 2. Just generate the table in runtime like a sane person
//!     - Costs nearly nothing compared to the 1M sorting that happens afterwards
//!     - Costs literally nothing if rustc/LLVM unrolls the loop and gets hardcoded anyways
//!
//! But no. I had to do what's called pro-grammer move, avoiding filthy manual labour of typing out
//! whopping 26 characters manually and automating the entire process with extra 100 lines of code.

/// Creates an array of `u8` where each element is set to its index + offset (`a[i] == i + offset`).
///
/// e.g. `enumerate::<4>(2)` == `[2, 3, 4, 5]`
///
/// Replacement for `std::array::from_fn(|i| i + offset)` which cannot be used in `const`.
pub const fn enumerate<const N: usize>(offset: u8) -> [u8; N] {
    let mut arr = [0; N];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = offset + i as u8;
        i += 1;
    }
    arr
}

/// Shuffles the given array with the steps described in [`README.md`](../index.html).
///
/// # Panics
///
/// Input array should have an even length (`N % 2 == 0`).
pub const fn shuffle<const N: usize>(mut arr: [u8; N]) -> [u8; N] {
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
/// Indices of the output array that are not present in the input array's values are considered
/// not intended to be accessed and will be filled with `0xFF` to make it easier to spot mistakes.
///
/// # Panics
///
/// Input array should not contain a value bigger than output array's length.
pub const fn transpose<const A: usize, const B: usize>(arr: &[u8; A]) -> [u8; B] {
    let mut transposed = [u8::MAX; B];
    let mut i = 0;
    while i < A {
        let (index, value) = (i as u8, arr[i] as usize);
        transposed[value] = index;
        i += 1;
    }
    transposed
}

/// Queries the lookup table for the shuffled sorting order.
///
/// # Safety
///
/// Input byte should be within `[a-z]` range.
#[inline]
pub unsafe fn sort_key(c: &u8) -> u8 {
    debug_assert!(
        ('a'..='z').contains(&(*c as char)),
        "input byte should be within [a-z] range"
    );

    // "abcdefghijklmnopqrstuvwxyz"
    const ALPHABETS: [u8; 26] = enumerate('a' as u8);

    // "mporqtsvuxwzybadcfehgjilkn"
    const SHUFFLED: [u8; 26] = shuffle(ALPHABETS);

    // padded with 'a' bytes, allowing us to use the character directly as an index
    // e.g. `LOOKUP['m' as usize] == 0`
    const LOOKUP: [u8; 'a' as usize + 26] = transpose(&SHUFFLED);

    *LOOKUP.get_unchecked(*c as usize)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shuffle() {
        let mut alphabets = *b"abcdefghijklmnopqrstuvwxyz";
        alphabets.sort_unstable_by_key(|c| unsafe { sort_key(c) });
        assert_eq!(alphabets, *b"mporqtsvuxwzybadcfehgjilkn");
    }
}

#![doc = include_str!("../README.md")]

/// Creates an array of `u8` where each element is set to its index (`a[i] == i`).
///
/// Replacement for `std::array::from_fn(|i| i + offset)` which cannot be used in `const`.
const fn enumerate<const N: usize>(offset: u8) -> [u8; N] {
    let mut arr = [0; N];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = offset + i as u8;
        i += 1;
    }
    arr
}

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
/// Indices of the output array that are not present in the input array will be filled with `0xFF`.
///
/// # Panics
///
/// Input array should not contain a value bigger than output array's length.
const fn transpose<const A: usize, const B: usize>(arr: &[u8; A]) -> [u8; B] {
    let mut transposed = [0xFF; B];
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
unsafe fn sort_key(c: &u8) -> u8 {
    debug_assert!(
        ('a'..='z').contains(&(*c as char)),
        "input byte should be within [a-z] range"
    );

    // Now, I *could* have hardcoded b"abcd..." like a normal person,
    // but no, I'm gonna do what's called pro-grammer move and write a whole new function
    // that automates the process and avoid filthy manual labour of typing out 26 characters
    const ALPHABETS: [u8; 26] = enumerate('a' as u8);

    // "mporqtsvuxwzybadcfehgjilkn"
    const SHUFFLED: [u8; 26] = shuffle(ALPHABETS);

    // LOOKUP['m' as usize] == 0
    const LOOKUP: [u8; 'a' as usize + 26] = transpose(&SHUFFLED);

    *LOOKUP.get_unchecked(*c as usize)
}

fn main() {
    let mut alphabets = String::from("abcdefghijklmnopqrstuvwxyz");
    unsafe {
        alphabets
            .as_bytes_mut()
            .sort_unstable_by_key(|c| sort_key(c));
    }
    println!("{}", alphabets);
}

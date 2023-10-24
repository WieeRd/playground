#![doc = include_str!("../README.md")]

/// Creates an array of `u8` where each element is set to its index (`a[i] == i`).
///
/// Replacement for `std::array::from_fn` which cannot be used in `const`.
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
/// # Panics
///
/// Input array should not contain a value bigger than output array's length.
const fn transpose<const A: usize, const B: usize>(arr: &[u8; A]) -> [u8; B] {
    let mut transposed = [0; B];
    let mut i = 0;
    while i < A {
        let (index, value) = (i as u8, arr[i] as usize);
        transposed[value] = index;
        i += 1;
    }
    transposed
}

#[inline]
fn sort_key(c: &u8) -> u8 {
    debug_assert!(
        ('a'..='z').contains(&(*c as char)),
        "input byte should be within [a-z] range"
    );

    // "abcdefghijklmnopqrstuvwxyz"
    const ALPHABETS: [u8; 26] = enumerate('a' as u8);

    // "mporqtsvuxwzybadcfehgjilkn"
    const SHUFFLED: [u8; 26] = shuffle(ALPHABETS);

    // SORT_KEY['m' as usize] == 0
    const SORT_KEY: [u8; 26 + 'a' as usize] = transpose(&SHUFFLED);

    SORT_KEY[*c as usize]
}

fn main() {
    let mut alphabets = String::from("abcdefghijklmnopqrstuvwxyz");
    unsafe {
        alphabets.as_bytes_mut().sort_unstable_by_key(sort_key);
    }
    println!("{}", alphabets);
}

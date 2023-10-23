#![doc = include_str!("../README.md")]

const ALPHABETS: usize = 26;

/// Creates an array of `u8` where each element is set to its index (`a[i] == i`).
///
/// e.g. `enumerate::<N>() == [1u8, 2u8, ..., N-1]`
const fn enumerate<const N: usize>() -> [u8; N] {
    //! # FAQ (the voices inside my head)
    //!
    //! 1. Isn't this supported by the language?
    //!
    //! Well yes, but actually no. We have:
    //!
    //! - `core::array::from_fn(|i| i)` ...except it's not `const`.
    //! - `(0..N).collect()` ...except it only works for `Vec`.
    //!
    //! Until const trait and fixed sized iterator stuffs are stabilized, this is all we've got.
    //!
    //! 2. `T` instead of `u8`?
    //!
    //! We need to convert `usize` index to `T` value in that case.
    //! Which requires `From`, which is a trait, which, cannot be `const`.
    let mut arr = [0; N];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = i as u8;
        i += 1;
    }
    arr
}

/// Shuffles the given array with the steps described in [`README.md`](index.html).
///
/// # Panics
///
/// Given array must have even length (`N % 2 == 0`).
const fn shuffle<const N: usize>(mut arr: [u8; N]) -> [u8; N] {
    assert!(N % 2 == 0, "given array must have even length");

    let mut i = 0;
    while i < N {
        (arr[i], arr[i + 1]) = (arr[i + 1], arr[i]);
        i += 2;
    }

    let mut i = 0;
    while i < N / 2 {
        (arr[i], arr[N / 2 + i]) = (arr[N / 2 + i], arr[i]);
        i += 1;
    }

    arr
}

fn main() {
    const A: [u8; ALPHABETS] = shuffle(enumerate());

    println!("{:?}", std::str::from_utf8(&A.map(|i| i + 'a' as u8)).unwrap());
}

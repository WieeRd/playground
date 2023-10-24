#![doc = include_str!("../README.md")]

const ALPHABETS: usize = 26;

/// Creates an array of `u8` where each element is set to its index (`a[i] == i`).
///
/// Replacement for `std::array::from_fn(|i| i)` which cannot be used in `const`.
const fn enumerate<const N: usize>() -> [u8; N] {
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
/// Given array must have an even length (`N % 2 == 0`).
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
/// Given array should not contain a value bigger than `N`.
const fn transpose<const N: usize>(arr: &[u8; N]) -> [u8; N] {
    let mut transposed = [0; N];
    let mut i = 0;
    while i < N {
        let (index, value) = (i as u8, arr[i] as usize);
        transposed[value] = index;
        i += 1;
    }
    transposed
}

/// Creates a bigger array by padding zeros to the beginning of the given array.
///
/// Replacement for `b[(B - A)..].copy_from_slice(&a)` which cannot be used in `const`.
///
/// # Panics
///
/// Output array should have a bigger length (`B >= A`).
const fn pad_zeros<const A: usize, const B: usize>(arr: &[u8; A]) -> [u8; B] {
    assert!(B >= A, "output array should have a bigger length");
    let mut padded = [0; B];
    let mut i = 0;
    while i < A {
        padded[B - A + i] = arr[i];
        i += 1;
    }
    padded
}

#[inline]
fn main() {
    const A: [u8; ALPHABETS] = shuffle(enumerate());
    const B: [u8; ALPHABETS + 'a' as usize] = pad_zeros(&A);

    println!("{:?}", std::str::from_utf8(&B.map(|i| i + 'a' as u8)));
}

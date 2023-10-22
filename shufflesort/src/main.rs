/// Creates an array of `u8` where the value of each element is its index number.
///
/// e.g. `enumerate::<N>() == [1u8, 2u8, ..., N-1]`
const fn enumerate<const N: usize>() -> [u8; N] {
    // # FAQ (the voices inside my head)
    //
    // 1. Isn't this supported by the language?
    //
    // Well yes, but actually no. We have:
    //
    // - `core::array::from_fn(|i| i)` ...except it's not `const`
    // - `(0..N).collect()` ...except it only works on `Vec`
    //
    // Until const trait and fixed sized iterator stuffs are stabilized, this is all we've got.
    //
    // 2. `T` instead of `u8`?
    //
    // We need to convert `usize` index to `T` value in that case.
    // Which requires `From`, which is a trait, which, cannot be `const`.
    let mut arr = [0u8; N];
    let mut i = 0;
    while i < arr.len() {
        arr[i] = i as u8;
        i += 1;
    }
    arr
}

fn main() {
    const A: [u8; 3] = enumerate();
    println!("{A:?}");
}

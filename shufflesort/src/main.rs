#![doc = include_str!("../README.md")]

mod shuffled;

fn main() {
    let mut alphabets = String::from("abcdefghijklmnopqrstuvwxyz");
    unsafe {
        alphabets
            .as_bytes_mut()
            .sort_unstable_by_key(|c| shuffled::sort_key(c));
    }
    println!("{}", alphabets);
}

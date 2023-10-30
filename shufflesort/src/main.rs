#![doc = include_str!("../README.md")]

use std::{env, fs, io};

#[allow(dead_code)]
mod shuffled;

// const WORD_COUNT: usize = 1_000_000;
// const WORD_LEN_MIN: usize = 5;
// const WORD_LEN_MAX: usize = 20;

fn main() -> Result<(), io::Error> {
    let input = match env::args_os().nth(1) {
        Some(path) => fs::read(path)?,
        None => fs::read("input.txt")?,
    };

    dbg!(input.len());
    Ok(())
}

#![doc = include_str!("../README.md")]

use std::{
    env,
    fs::File,
    io::{self, Read},
};

#[allow(dead_code)]
mod shuffled;

#[allow(dead_code)]
mod word;

const _WORD_COUNT: usize = 1_000_000;

const _WORD_LEN_MIN: usize = 5;
const _WORD_LEN_MAX: usize = 20;

fn main() -> Result<(), io::Error> {
    let _input = {
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

    // let words: Vec<word::Word>;
    // words.sort_unstable();

    Ok(())
}

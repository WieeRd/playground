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
        // read from argv[1] or "input.txt"
        let mut file = match env::args_os().nth(1) {
            Some(path) => File::open(path)?,
            None => File::open("input.txt")?,
        };
        let size = file.metadata()?.len() as usize;

        let mut bytes = Vec::with_capacity(size + 1);
        file.read_to_end(&mut bytes)?;

        // make sure the input ends with a newline
        if bytes.last() != Some(&b'\n') {
            bytes.push(b'\n');
        }

        bytes
    };

    // use std::fs;
    // let _input = match env::args_os().nth(1) {
    //     Some(path) => fs::read(path)?,
    //     None => fs::read("input.txt")?,
    // };

    Ok(())
}

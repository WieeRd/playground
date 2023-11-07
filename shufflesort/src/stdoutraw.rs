use std::{fs::File, io};

#[cfg(unix)]
pub fn stdoutraw() -> File {
    use std::os::fd::{AsRawFd, FromRawFd};

    let stdout = io::stdout();
    let raw_fd = stdout.as_raw_fd();
    unsafe { File::from_raw_fd(raw_fd) }
}

#[cfg(windows)]
pub fn stdoutraw() -> File {
    use std::os::windows::io::{AsRawHandle, FromRawHandle};

    let stdout = io::stdout();
    let raw_handle = stdout.as_raw_handle();
    unsafe { File::from_raw_handle(raw_handle) }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::{self, Write};

    #[test]
    fn fucking_rawwww() -> Result<(), io::Error> {
        let mut stdout = stdoutraw();
        stdout.write_all(b"FUCKING RAWWWW")?;

        Ok(())
    }
}

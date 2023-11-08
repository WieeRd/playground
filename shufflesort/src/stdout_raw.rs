use std::{fs::File, io};

#[cfg(unix)]
pub fn stdout_raw() -> File {
    use std::os::fd::{AsRawFd, FromRawFd};

    let stdout = io::stdout();
    let raw_fd = stdout.as_raw_fd(); // or just use `1`
    unsafe { File::from_raw_fd(raw_fd) }
}

#[cfg(windows)]
pub fn stdout_raw() -> File {
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
    fn fucking_rawwww() -> io::Result<()> {
        let mut stdout = stdout_raw();
        stdout.write_all(b"This stdout... is RAWWWWWW!!!")?;

        Ok(())
    }
}

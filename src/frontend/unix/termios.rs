use std::{io, mem::MaybeUninit};

pub struct RawMode {
    original: libc::termios,
}

impl RawMode {
    pub fn enable() -> io::Result<Self> {
        let fd = libc::STDIN_FILENO;

        let mut original = MaybeUninit::<libc::termios>::uninit();

        if unsafe { libc::tcgetattr(fd, original.as_mut_ptr()) } == -1 {
            return Err(io::Error::last_os_error());
        }

        let original = unsafe { original.assume_init() };
        let mut raw = original;

        raw.c_lflag &= !(libc::ICANON | libc::ECHO | libc::ISIG);
        raw.c_cc[libc::VMIN] = 1;
        raw.c_cc[libc::VTIME] = 0;

        if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &raw) } == -1 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { original })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.original);
        }
    }
}

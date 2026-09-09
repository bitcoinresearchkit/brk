#[cfg(any(
    target_os = "macos",
    target_os = "linux",
    target_os = "freebsd",
    not(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))
))]
use crate::Result;

use std::fs::File;

use crate::Error;

#[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
use libc::off_t;
#[cfg(target_os = "macos")]
use libc::{F_PUNCHHOLE, fcntl};
#[cfg(target_os = "linux")]
use libc::{FALLOC_FL_KEEP_SIZE, FALLOC_FL_PUNCH_HOLE, fallocate};
#[cfg(target_os = "freebsd")]
use libc::{SPACECTL_DEALLOC, fspacectl, spacectl_range};
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "freebsd"))]
use std::io::Error as IoError;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

/// Deallocates file blocks without changing file size (platform-specific).
pub struct HolePunch;

impl HolePunch {
    #[cfg(target_os = "macos")]
    pub fn punch(file: &File, start: usize, length: usize) -> Result<()> {
        let fpunchhole = FPunchhole {
            fp_flags: 0,
            reserved: 0,
            fp_offset: start as off_t,
            fp_length: length as off_t,
        };

        let result = unsafe {
            fcntl(
                file.as_raw_fd(),
                F_PUNCHHOLE,
                &fpunchhole as *const FPunchhole,
            )
        };

        if result == -1 {
            let err = IoError::last_os_error();
            return Err(Error::HolePunchFailed {
                start,
                len: length,
                source: err,
            });
        }

        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn punch(file: &File, start: usize, length: usize) -> Result<()> {
        let result = unsafe {
            fallocate(
                file.as_raw_fd(),
                FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE,
                start as off_t,
                length as off_t,
            )
        };

        if result == -1 {
            let err = IoError::last_os_error();
            return Err(Error::HolePunchFailed {
                start,
                len: length,
                source: err,
            });
        }

        Ok(())
    }

    #[cfg(target_os = "freebsd")]
    pub fn punch(file: &File, start: usize, length: usize) -> Result<()> {
        let fd = file.as_raw_fd();

        let mut spacectl = spacectl_range {
            r_offset: start as off_t,
            r_len: length as off_t,
        };

        let result = unsafe {
            fspacectl(
                fd,
                SPACECTL_DEALLOC,
                &spacectl as *const spacectl_range,
                0,
                &mut spacectl as *mut spacectl_range,
            )
        };

        if result == -1 {
            let err = IoError::last_os_error();
            return Err(Error::HolePunchFailed {
                start,
                len: length,
                source: err,
            });
        }

        Ok(())
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "freebsd")))]
    pub fn punch(_file: &File, _start: usize, _length: usize) -> Result<()> {
        Err(Error::HolePunchUnsupported)
    }
}

#[cfg(target_os = "macos")]
#[repr(C)]
struct FPunchhole {
    fp_flags: u32,
    reserved: u32,
    fp_offset: off_t,
    fp_length: off_t,
}

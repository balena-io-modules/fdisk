
use std::ffi::CString;
use std::io::{Error, ErrorKind, Result};
use std::os::raw::c_int;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use ffi;

pub struct Context {
    pub(crate) ptr: *mut ffi::fdisk_context
}

impl Context {
    pub fn new() -> Context {
        Context {
            ptr: unsafe { ffi::fdisk_new_context() }
        }
    }

    pub fn assign_device<P: AsRef<Path>>(&mut self, path: P, readonly: bool) -> Result<()> {
        let c_path = CString::new(path.as_ref().as_os_str().as_bytes())
            .map_err(|_| ErrorKind::InvalidInput)?;
        let result = unsafe { ffi::fdisk_assign_device(self.ptr, c_path.as_ptr(), readonly as c_int)};
        if result != 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::fdisk_unref_context(self.ptr) }
    }
}
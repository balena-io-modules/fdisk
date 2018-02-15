
use std::io::{Error, Result};

use context::Context;
use ffi;

impl Context {
    pub fn verify_disklabel(&mut self) -> Result<()> {
        match unsafe { ffi::fdisk_write_disklabel(self.ptr) } {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }

    pub fn write_disklabel(&mut self) -> Result<()> {
        match unsafe { ffi::fdisk_write_disklabel(self.ptr) } {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}
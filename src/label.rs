
use error::*;

use context::Context;
use ffi;

impl Context {
    pub fn verify_disklabel(&mut self) -> Result<()> {
        match unsafe { ffi::fdisk_write_disklabel(self.ptr) } {
            0 => Ok(()),
            x => Err(ErrorKind::from(x).into()),
        }
    }

    pub fn write_disklabel(&mut self) -> Result<()> {
        match unsafe { ffi::fdisk_write_disklabel(self.ptr) } {
            0 => Ok(()),
            x => Err(ErrorKind::from(x).into()),
        }
    }
}
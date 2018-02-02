
use ffi;

use context::Context;

use std::mem;
use std::io::{Error, ErrorKind, Result};

pub struct Partition {
    pub(crate) ptr: *mut ffi::fdisk_partition,
}

impl Partition {
    pub fn new() -> Partition {
        Partition {
            ptr: unsafe { ffi::fdisk_new_partition() }
        }
    }
}

impl Drop for Partition {
    fn drop(&mut self) {
        unsafe { ffi::fdisk_unref_partition(self.ptr) }
    }
}
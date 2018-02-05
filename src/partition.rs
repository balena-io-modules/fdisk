
use ffi;

use error::*;
use context::Context;

use std::io::Error as IOError;
use std::ffi::CStr;
use std::mem;

pub struct Partition {
    pub(crate) ptr: *mut ffi::fdisk_partition,
}

impl Partition {
    pub fn new() -> Partition {
        Partition {
            ptr: unsafe { ffi::fdisk_new_partition() }
        }
    }

    pub fn name<'a>(&'a mut self) -> Result<&'a str> {
        let ptr = unsafe { ffi::fdisk_partition_get_name(self.ptr) };
        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .chain_err(|| "invalid name")
    }

    pub fn start(&mut self) -> Option<u64> {
        match unsafe { ffi::fdisk_partition_has_start(self.ptr) } {
            0 => None,
            _ => Some(unsafe { ffi::fdisk_partition_get_size(self.ptr) }),
        }
    }

    pub fn set_start(&mut self, start: Option<u64>) -> Result<()> {
        let code = match start {
            None => unsafe { ffi::fdisk_partition_unset_start(self.ptr) }, 
            Some(x) => unsafe { ffi::fdisk_partition_set_start(self.ptr, x) },
        };

        match code {
            0 => Ok(()),
            _ => Err(Error::with_chain(IOError::last_os_error(), "unable to set start"))
        }
    } 

    pub fn size(&mut self) -> Option<u64> {
        match unsafe { ffi::fdisk_partition_has_size(self.ptr) } {
            0 => None,
            _ => Some(unsafe { ffi::fdisk_partition_get_size(self.ptr) })
        }
    }

    pub fn set_size(&mut self, size: Option<u64>) -> Result<()> {
        let code = match size {
            None => unsafe { ffi::fdisk_partition_unset_size(self.ptr) },
            Some(x) => unsafe { ffi::fdisk_partition_set_start(self.ptr, x) },
        };

        match code {
            0 => Ok(()),
            _ => Err(Error::with_chain(IOError::last_os_error(), "unable to set size")),
        }
    }
}

impl Drop for Partition {
    fn drop(&mut self) {
        unsafe { ffi::fdisk_unref_partition(self.ptr) }
    }
}
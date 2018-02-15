
use ffi;

use error::*;

use std::io::Error as IOError;
use std::ffi::CStr;

pub struct Partition {
    pub(crate) ptr: *mut ffi::fdisk_partition,
}

impl Partition {
    pub fn new() -> Partition {
        Partition {
            ptr: unsafe { ffi::fdisk_new_partition() }
        }
    }

    pub fn end(&mut self) -> Option<u64> {
        match unsafe { ffi::fdisk_partition_has_end(self.ptr) } {
            0 => None,
            _ => Some(unsafe { ffi::fdisk_partition_get_end(self.ptr) as u64 })
        }
    }

    pub fn name<'a>(&'a mut self) -> Result<Option<&'a str>> {
        let ptr = unsafe { ffi::fdisk_partition_get_name(self.ptr) };
        
        if ptr.is_null() {
            return Ok(None);
        }

        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .chain_err(|| "invalid name")
            .map(|s| Some(s))
    }

    pub fn partno(&mut self) -> Option<u64> {
        match unsafe { ffi::fdisk_partition_has_partno(self.ptr) } {
            0 => None,
            _ => Some(unsafe { ffi::fdisk_partition_get_partno(self.ptr) as u64 }),
        }
    }

    pub fn start(&mut self) -> Option<u64> {
        match unsafe { ffi::fdisk_partition_has_start(self.ptr) } {
            0 => None,
            _ => Some(unsafe { ffi::fdisk_partition_get_start(self.ptr) }),
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
            Some(x) => unsafe { ffi::fdisk_partition_set_size(self.ptr, x) },
        };

        match code {
            0 => Ok(()),
            _ => Err(Error::with_chain(IOError::last_os_error(), "unable to set size")),
        }
    }

    pub fn bootable(&mut self) -> bool {
        match unsafe { ffi::fdisk_partition_is_bootable(self.ptr) } {
            0 => false,
            _ => true,
        }
    }
}

impl Drop for Partition {
    fn drop(&mut self) {
        unsafe { ffi::fdisk_unref_partition(self.ptr) }
    }
}
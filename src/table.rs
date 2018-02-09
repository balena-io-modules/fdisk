
use ffi;

use context::Context;
use partition::Partition;

use std::mem;
use std::io::{Error, ErrorKind, Result};

pub struct Table {
    ptr: *mut ffi::fdisk_table,
}

impl Table {
    pub fn iter<'a>(&'a mut self) -> Iter<'a> {
        Iter {
            table: self,
            ptr: unsafe { ffi::fdisk_new_iter(ffi::FDISK_ITER_FORWARD as i32) }
        }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        unsafe { ffi::fdisk_unref_table(self.ptr) }
    }
}

impl Context {
    pub fn get_partitions(&mut self) -> Result<Table> {
        let mut table: *mut ffi::fdisk_table = unsafe { mem::zeroed() };
        if unsafe { ffi::fdisk_get_partitions(self.ptr, &mut table) } != 0 {
            return Err(Error::last_os_error());
        }

        unsafe { ffi::fdisk_ref_table(table); }

        Ok(Table { ptr: table })
    }
}

pub struct Iter<'a> {
    table: &'a mut Table,
    ptr: *mut ffi::fdisk_iter,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Partition;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ptr: *mut ffi::fdisk_partition = unsafe { mem::zeroed() };
        match unsafe { ffi::fdisk_table_next_partition(self.table.ptr, self.ptr, &mut ptr) } {
            0 => Some(Partition { ptr: ptr }),
            _ => None,
        }
    }
}

impl<'a> Drop for Iter<'a> {
    fn drop(&mut self) {
        unsafe { ffi::fdisk_free_iter(self.ptr) }
    }
}
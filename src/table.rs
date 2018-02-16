
use ffi;

use error::*;
use context::Context;
use partition::Partition;

use std::mem;

pub struct Table {
    ptr: *mut ffi::fdisk_table,
}

impl Table {
    pub fn new() -> Table {
        Table {
            ptr: unsafe { ffi::fdisk_new_table() }
        }
    }

    pub fn iter<'a>(&'a mut self) -> Iter<'a> {
        Iter {
            table: self,
            ptr: unsafe { ffi::fdisk_new_iter(ffi::FDISK_ITER_FORWARD as i32) }
        }
    }

    pub fn reset(&mut self) -> Result<()> {
        match unsafe {ffi::fdisk_reset_table(self.ptr)} {
            0 => Ok(()),
            x => Err(ErrorKind::from(x).into()),
        }
    }

    pub fn add_partition(&mut self, partition: &mut Partition) -> Result<()> {
        match unsafe {ffi::fdisk_table_add_partition(self.ptr, partition.ptr)} {
            0 => Ok(()),
            x => Err(ErrorKind::from(x).into()),
        }
    }

    pub fn remove_partition(&mut self, partition: &mut Partition) -> Result<()> {
        match unsafe { ffi::fdisk_table_remove_partition(self.ptr, partition.ptr) } {
            0 => Ok(()),
            x => Err(ErrorKind::from(x).into()),
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
        let mut table = Table::new();
        match unsafe { ffi::fdisk_get_partitions(self.ptr, &mut table.ptr) } {
            0 => Ok(table),
            x => Err(ErrorKind::from(x).into()),
        }
    }

    pub fn apply_table(&mut self, table: &mut Table) -> Result<()> {
        match unsafe { ffi::fdisk_apply_table(self.ptr, table.ptr) } {
            0 => Ok(()),
            x => Err(ErrorKind::from(x).into()),
        }
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
            0 => {
                unsafe { ffi::fdisk_ref_partition(ptr) };
                Some(Partition { ptr: ptr })
            },
            _ => None,
        }
    }
}

impl<'a> Drop for Iter<'a> {
    fn drop(&mut self) {
        unsafe { ffi::fdisk_free_iter(self.ptr) }
    }
}
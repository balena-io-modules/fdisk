
use ffi;

impl From<i32> for ::error::ErrorKind {
    fn from(err: i32) -> Self {
        ::error::ErrorKind::NativeError((-err) as u32)
    }
}

impl From<u64> for ::error::ErrorKind {
    fn from(err: u64) -> Self {
        ::error::ErrorKind::NativeError(err as u32)
    }
}

pub fn debug(enable: bool) {
    let mask = if enable { 0xffff } else { 0 };
    unsafe { ffi::fdisk_init_debug(mask) };
}
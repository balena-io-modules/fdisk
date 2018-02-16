
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
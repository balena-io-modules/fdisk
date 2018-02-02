
extern crate fdisk_sys as ffi;

pub mod context;
pub mod table;
pub mod partition;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

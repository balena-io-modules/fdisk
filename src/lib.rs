
extern crate fdisk_sys as ffi;
#[macro_use]
extern crate error_chain;

pub mod context;
pub mod table;
pub mod partition;

pub mod error {
    error_chain!{}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

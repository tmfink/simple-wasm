//! Safe wrapper around libdouble

use double_sys as ffi;

use std::os::raw::c_uint;

/// Safe wrapper around `double_it()`
pub fn double_it(x: u32) -> u32 {
    unsafe { ffi::double_it(x as c_uint) as u32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(double_it(2), 4);
    }
}
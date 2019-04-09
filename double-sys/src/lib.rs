use std::os::raw::c_uint;

extern "C" {
    pub fn double_it(x: c_uint) -> c_uint;
}

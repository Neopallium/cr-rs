use std::os::raw::{c_int, c_uint, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cr_plugin {
    p: *mut c_void,
    userdata: *mut c_void,
    version: c_uint,
    failure: c_int,
}


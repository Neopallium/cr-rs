use std::os::raw::{c_int, c_uint, c_void};

#[repr(C)]
#[derive(Debug)]
pub struct cr_plugin {
    p: *mut c_void,
    pub userdata: *mut c_void,
    pub version: c_uint,
    pub failure: c_int,
}

impl cr_plugin {
    pub fn new() -> cr_plugin {
        cr_plugin {
            p: std::ptr::null_mut(),
            userdata: std::ptr::null_mut(),
            version: 0,
            failure: 0,
        }
    }
}


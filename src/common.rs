use std::os::raw::{c_int, c_uint, c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cr_plugin {
    p: *mut c_void,
    userdata: *mut c_void,
    version: c_uint,
    failure: c_int,
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


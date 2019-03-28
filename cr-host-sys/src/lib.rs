use std::os::raw::{c_int, c_uint, c_uchar, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
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

extern "C" {
    pub fn cr_plugin_load(ctx: &mut cr_plugin, fullpath: *const c_uchar) -> bool;
    pub fn cr_plugin_update(ctx: &mut cr_plugin, reload_check: bool) -> c_int;
    pub fn cr_plugin_close(ctx: &mut cr_plugin);

    pub fn wrap_cr_set_temporary_path(ctx: &mut cr_plugin, path: *const c_uchar) -> bool;
}

pub unsafe fn cr_set_temporary_path(ctx: &mut cr_plugin, path: *const c_uchar) {
    wrap_cr_set_temporary_path(ctx, path);
}


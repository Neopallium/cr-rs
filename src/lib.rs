extern crate cr_sys;

use cr_sys::*;

pub use cr_sys::cr_op;
pub use cr_sys::cr_failure;

#[derive(Debug)]
pub struct Plugin {
    ctx: cr_plugin,
}

impl Plugin {
    #[cfg(not(feature = "guest"))]
    pub fn new(fullpath: &str) -> Box<Plugin> {
        let plugin = Box::new(Plugin {
            ctx: cr_plugin::new(),
        });
        // Get pointer to heap allocated `Plugin`.
        // TODO: There must be a better way!
        let ptr = Box::into_raw(plugin); // Unbox to get raw pointer
        let mut plugin = unsafe { Box::from_raw(ptr) }; // Rebox.

        // Store our Plugin struct as the `userdata`
        plugin.ctx.userdata = ptr as *mut _ as *mut ::std::os::raw::c_void;

        let s_fullpath = std::ffi::CString::new(fullpath).unwrap();
        unsafe { cr_plugin_load(&mut plugin.ctx, s_fullpath.as_ptr())};

        plugin
    }

    pub fn from_ctx(ctx: &mut cr_plugin) -> &mut Plugin {
        unsafe { &mut *(ctx.userdata as *mut Plugin) }
    }

    #[cfg(not(feature = "guest"))]
    pub fn set_temporary_path(&mut self, path: &str) {
        let s_path = std::ffi::CString::new(path).unwrap();
        unsafe { wrap_cr_set_temporary_path(&mut self.ctx, s_path.as_ptr())};
    }

    #[cfg(not(feature = "guest"))]
    pub fn update(&mut self, reload_check: bool) -> i32 {
        unsafe { cr_plugin_update(&mut self.ctx, reload_check)}
    }

    pub fn get_version(&self) -> u32 {
        self.ctx.version
    }

    pub fn get_failure(&self) -> cr_failure {
        self.ctx.failure
    }
}

#[cfg(not(feature = "guest"))]
impl Drop for Plugin {
    fn drop(&mut self) {
        unsafe { cr_plugin_close(&mut self.ctx)}
    }
}

#[macro_export]
macro_rules! cr_main {
    ($rust_cr_main:ident) => (
        use std::os::raw::c_int;
        #[no_mangle]
        pub fn cr_main(ctx: &mut cr_sys::cr_plugin, cr_op: cr::cr_op) -> c_int {
            $rust_cr_main(cr::Plugin::from_ctx(ctx), cr_op)
        }
    )
}


pub mod common;

#[cfg(not(guest))]
pub mod host;

use common::cr_plugin;

#[derive(Debug)]
pub struct Plugin {
    ctx: cr_plugin,
}

impl Plugin {
    #[cfg(not(guest))]
    pub fn new(fullpath: &str) -> Plugin {
        let mut plugin = Plugin {
            ctx: cr_plugin::new(),
        };
        // Store our Plugin struct as the `userdata`
        plugin.ctx.userdata = &mut plugin as *mut _ as *mut ::std::os::raw::c_void;

        let s_fullpath = std::ffi::CString::new(fullpath).unwrap();
        unsafe { host::cr_plugin_load(&mut plugin.ctx, s_fullpath.as_ptr())};

        plugin
    }

    pub fn from_ctx(ctx: &mut cr_plugin) -> &mut Plugin {
        unsafe { &mut *(ctx.userdata as *mut Plugin) }
    }

    #[cfg(not(guest))]
    pub fn update(&mut self, reload_check: bool) -> i32 {
        unsafe { host::cr_plugin_update(&mut self.ctx, reload_check)}
    }
}

#[cfg(not(guest))]
impl Drop for Plugin {
    fn drop(&mut self) {
        unsafe { host::cr_plugin_close(&mut self.ctx)}
    }
}

#[macro_export]
macro_rules! cr_main {
    ($rust_cr_main:ident) => (
        #[no_mangle]
        pub fn cr_main(ctx: &mut cr_sys::common::cr_plugin, cr_op: c_int) -> c_int {
            let plugin = cr_sys::Plugin::from_ctx(ctx);
            $rust_cr_main(plugin, cr_op)
        }
    )
}


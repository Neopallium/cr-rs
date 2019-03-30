extern crate cr_sys;

use cr_sys::*;

pub use cr_sys::cr_failure;
pub use cr_sys::cr_op;

#[derive(Debug)]
pub struct Plugin<State> {
    ctx: cr_plugin,
    state: State,
}

impl<State> Plugin<State> {
    #[cfg(not(feature = "guest"))]
    pub fn new(state: State, fullpath: &str) -> Box<Plugin<State>> {
        let mut plugin = Box::new(Plugin {
            ctx: cr_plugin::new(),
            state: state,
        });
        // Store pointer to `Plugin` struct in `userdata` to be passed to the guest's `cr_main`
        plugin.ctx.userdata = &mut (*plugin) as *mut _ as *mut ::std::os::raw::c_void;

        let s_fullpath = std::ffi::CString::new(fullpath).unwrap();
        unsafe { cr_plugin_load(&mut plugin.ctx, s_fullpath.as_ptr()) };

        plugin
    }

    pub fn from_ctx(ctx: &mut cr_plugin) -> &mut Plugin<State> {
        unsafe { &mut *(ctx.userdata as *mut Plugin<State>) }
    }

    #[cfg(not(feature = "guest"))]
    pub fn set_temporary_path(&mut self, path: &str) {
        let s_path = std::ffi::CString::new(path).unwrap();
        unsafe { wrap_cr_set_temporary_path(&mut self.ctx, s_path.as_ptr()) };
    }

    #[cfg(not(feature = "guest"))]
    pub fn update(&mut self, reload_check: bool) -> i32 {
        unsafe { cr_plugin_update(&mut self.ctx, reload_check) }
    }

    pub fn get_version(&self) -> u32 {
        self.ctx.version
    }

    pub fn get_failure(&self) -> cr_failure {
        self.ctx.failure
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }
}

#[cfg(not(feature = "guest"))]
impl<State> Drop for Plugin<State> {
    fn drop(&mut self) {
        unsafe { cr_plugin_close(&mut self.ctx) }
    }
}

#[macro_export]
macro_rules! cr_main {
    ($rust_cr_main:ident) => {
        use std::os::raw::c_int;
        #[no_mangle]
        pub fn cr_main(ctx: &mut cr_sys::cr_plugin, cr_op: cr::cr_op) -> c_int {
            $rust_cr_main(cr::Plugin::from_ctx(ctx), cr_op)
        }
    };
}

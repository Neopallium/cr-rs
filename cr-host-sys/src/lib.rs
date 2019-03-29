pub mod host;

pub struct Plugin {
    ctx: host::cr_plugin,
}

impl Plugin {
    pub fn new(fullpath: &str) -> Plugin {
        let mut plugin = Plugin {
            ctx: host::cr_plugin::new(),
        };

        let s_fullpath = std::ffi::CString::new(fullpath).unwrap();
        unsafe { host::cr_plugin_load(&mut plugin.ctx, s_fullpath.as_ptr())};

        plugin
    }

    pub fn update(&mut self, reload_check: bool) -> i32 {
        unsafe { host::cr_plugin_update(&mut self.ctx, reload_check)}
    }

    pub fn close(&mut self) {
        unsafe { host::cr_plugin_close(&mut self.ctx)}
    }
}

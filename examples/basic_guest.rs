extern crate cr_sys;

use std::os::raw::{c_int};

use cr_sys::common::*;

#[no_mangle]
pub fn cr_main(_ctx: &mut cr_plugin, _cr_op: c_int) -> c_int {
    // Test "guest" feature.
    #[cfg(not(guest))]
    {
        println!("Guest compiled with host-side code.");
        // This code will only run if the "guest" feature is not used.
        let plugin = cr_sys::Plugin::new("test");
        println!("- plugin = {:?}", plugin);
    }

    println!("Hello from rust plugin. test2");
    return 0;
}


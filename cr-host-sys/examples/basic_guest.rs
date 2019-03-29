extern crate cr_host_sys;

use std::os::raw::{c_int};

use cr_host_sys::host::*;

#[no_mangle]
pub fn cr_main(ctx: &mut cr_plugin, cr_op: c_int) -> c_int {
    println!("Hello from rust plugin. test2");
    return 0;
}


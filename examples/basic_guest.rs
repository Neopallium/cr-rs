extern crate cr_sys;

use std::os::raw::{c_int};

use cr_sys::common::*;

#[no_mangle]
pub fn cr_main(_ctx: &mut cr_plugin, _cr_op: c_int) -> c_int {
    println!("Hello from rust plugin. test2");
    return 0;
}


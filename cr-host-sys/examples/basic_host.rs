extern crate cr_host_sys;

use std::time::Duration;
use std::thread;

use cr_host_sys::*;

fn main() {
    let mut ctx = cr_plugin::new();

    // build the libbasic_guest.so file from the samples of cr.h
    println!("Call cr_plugin_load(ctx, \"libbasic_guest.so\")");
    unsafe { cr_plugin_load(&mut ctx, b"libbasic_guest.so\0".as_ptr()) };
    loop {
        println!("Run Update:");
        let rc = unsafe { cr_plugin_update(&mut ctx, true) };
        println!("cr_plugin_update(ctx, true) = {}", rc);
        if rc != 0 { break }
        thread::sleep(Duration::from_millis(200));
    }
    println!("Call cr_plugin_close(ctx)");
    unsafe { cr_plugin_close(&mut ctx); }
    println!("exit");
}


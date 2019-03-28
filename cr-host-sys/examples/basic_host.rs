extern crate cr_host_sys;

use cr_host_sys::*;

fn main() {
    let mut ctx = cr_plugin::new();

    println!("Call cr_plugin_load(ctx, \"test\")");
    unsafe { cr_plugin_load(&mut ctx, b"test\0".as_ptr()) };
    loop {
        println!("Run Update:");
        let rc = unsafe { cr_plugin_update(&mut ctx, true) };
        println!("cr_plugin_update(ctx, true) = {}", rc);
        if rc != 0 { break }
    }
    println!("Call cr_plugin_close(ctx)");
    unsafe { cr_plugin_close(&mut ctx); }
    println!("exit");
}


extern crate cr_sys;

use std::env;

use std::time::Duration;
use std::thread;

use cr_sys::*;

fn main() {
    let mut plugin_name = env::current_exe().expect("Failed to get current path");
    println!("Path of this executable is: {}", plugin_name.display());
    plugin_name.set_file_name("libbasic_guest.so");
    // build the libbasic_guest.so file from the samples of cr.h
    println!("Call cr_plugin_load(ctx, {:?})", plugin_name);
    let mut plugin = Plugin::new(plugin_name.to_str().unwrap());

    loop {
        println!("Run Update:");
        let rc = plugin.update(true);
        println!("cr_plugin_update(ctx, true) = {}", rc);
        if rc != 0 { break }
        thread::sleep(Duration::from_millis(200));
    }
    println!("Call cr_plugin_close(ctx)");
    drop(plugin);
    println!("exit");
}


extern crate cr;

use std::env;

use std::thread;
use std::time::Duration;

mod basic_state;

use basic_state::*;

fn main() {
    let mut plugin_name = env::current_exe().expect("Failed to get current path");
    println!("Path of this executable is: {}", plugin_name.display());
    plugin_name.set_file_name("libbasic_guest.so");
    // build the libbasic_guest.so file from the samples of cr.h
    println!("Call cr_plugin_load(ctx, {:?})", plugin_name);
    let mut plugin = BasicPlugin::new(BasicState { counter: 0 }, plugin_name.to_str().unwrap());

    let mut err_cnt = 0;
    loop {
        println!("Run Update:");
        let rc = plugin.update(true);
        println!("cr_plugin_update(ctx, true) = {}", rc);
        if rc != 0 {
            println!("Plugin error: {:?}", plugin.get_failure());
            err_cnt += 1;
            if err_cnt > 10 {
                break;
            }
        }
        thread::sleep(Duration::from_millis(200));
    }
    println!("Call cr_plugin_close(ctx)");
    drop(plugin);
    println!("exit");
}

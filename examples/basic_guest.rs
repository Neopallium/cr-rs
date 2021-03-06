#[macro_use]
extern crate cr;

use std::thread;
use std::time::Duration;

mod basic_state;

use basic_state::*;

use cr::cr_op::*;

cr_main!(plugin_main);

pub fn plugin_main(ctx: &mut BasicPlugin, cr_op: cr::cr_op) -> i32 {
    // Test "guest" feature.
    #[cfg(not(feature = "guest"))]
    {
        println!("Guest compiled with host-side code.");
        // This code will only run if the "guest" feature is not used.
        let plugin = BasicPlugin::new(BasicState { counter: 0 }, "test");
        println!("- plugin = {:?}", plugin);
    }

    println!("test recompile. test1");
    //panic!("test");
    // Test crash.
    // Rollback to the previous version seems to work once.
    // Then on the next reload it gets stuck in a setjmp/longjmp loop?
    /*
    unsafe {
        let ptr
            = std::ptr::null_mut() as *mut std::os::raw::c_int;
        // invalid read.
        println!("Read from null pointer: {:?}", *ptr);
        // invalid write.
        *ptr = 0x1234;
    }
    // */
    match cr_op {
        CR_LOAD => {
            println!("Plugin load. version = {}", ctx.get_version());
        }
        CR_STEP => {
            let version = ctx.get_version();
            let state = ctx.state_mut();
            state.counter += 1;
            println!(
                "Plugin step. count = {}. version = {}",
                state.counter, version
            );

            // slow down the printing.
            thread::sleep(Duration::from_millis(200));
        }
        CR_UNLOAD => {
            println!("Plugin unload. version = {}", ctx.get_version());
        }
        CR_CLOSE => {
            println!("Plugin close. version = {}", ctx.get_version());
        }
    }

    return 0;
}

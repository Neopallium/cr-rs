#[macro_use]
extern crate cr_sys;

cr_main!(plugin_main);

pub fn plugin_main(_ctx: &mut cr_sys::Plugin, _cr_op: i32) -> i32 {
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


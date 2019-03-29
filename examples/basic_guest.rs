#[macro_use]
extern crate cr;

use cr::Plugin;

cr_main!(plugin_main);

pub fn plugin_main(_ctx: &mut Plugin, _cr_op: cr::cr_op) -> i32 {
    // Test "guest" feature.
    #[cfg(not(feature = "guest"))]
    {
        println!("Guest compiled with host-side code.");
        // This code will only run if the "guest" feature is not used.
        let plugin = Plugin::new("test");
        println!("- plugin = {:?}", plugin);
    }

    println!("Hello from rust plugin. test2");
    return 0;
}


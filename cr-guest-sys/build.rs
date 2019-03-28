// build.rs

extern crate cc;

fn main() {  
    cc::Build::new()
        .file("src/cr-guest.c")
        .compile("cr-guest");
}

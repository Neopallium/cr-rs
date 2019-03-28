// build.rs

extern crate cc;

fn main() {  
    cc::Build::new()
        .file("src/cr-host.cpp")
        .compile("cr-host");
}

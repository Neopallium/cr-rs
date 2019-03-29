// build.rs

extern crate cc;

fn main() {  
    println!("cargo:rustc-link-lib=stdc++");
    cc::Build::new()
        .file("src/cr-host.cpp")
        .flag("-Wno-unused-parameter")
        .compile("cr-host");
}

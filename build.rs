// build.rs

extern crate cc;

fn main() {  
    if cfg!(feature = "guest") {
        build_guest();
    } else {
        build_host();
    }
}

fn build_guest() {  
    println!("cargo:rustc-cfg=guest");
    cc::Build::new()
        .file("src/guest.c")
        .flag("-Wno-unused-parameter")
        .compile("cr");
}

fn build_host() {  
    println!("cargo:rustc-link-lib=stdc++");
    cc::Build::new()
        .file("src/host.cpp")
        .flag("-Wno-unused-parameter")
        .compile("cr");
}

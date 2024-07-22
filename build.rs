// use std::env;

fn main() {
    //println!("cargo:rerun-if-changed=build.rs");
    // let target = env::var("TARGET").unwrap();
    // let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-env=RUSTFLAGS=-C opt-level=3");

    // if target.contains("windows") && profile == "dev" {
    //     println!("cargo:rustc-env=RUSTFLAGS=-C opt-level=0");
    // }
}

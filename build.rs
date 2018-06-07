extern crate cc;
extern crate rustc_version;
use rustc_version::{version, Version};

fn main() {
    let newer_than_1_27 = version().unwrap() >= Version::parse("1.27.0").unwrap();
    if newer_than_1_27 {
        println!("cargo:rustc-cfg=feature=\"use_arch\"");
    } else {
        cc::Build::new().file("src/cpuid.c").compile("libcpuid.a");
    }
}

extern crate rustc_version;
extern crate cc;
use rustc_version::{version, version_meta, Channel, Version};

fn main() {
    let nightly = version_meta().unwrap().channel == Channel::Nightly;
    let newer_than_1_27 = version().unwrap() >= Version::parse("1.27.0").unwrap();

    let use_arch = nightly || newer_than_1_27;

    if use_arch {
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    } else {
        cc::Build::new().file("src/cpuid.c").compile("libcpuid.a");
    }
}

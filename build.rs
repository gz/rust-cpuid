extern crate cc;

#[cfg(not(feature = "nightly"))]
fn main() {
    cc::Build::new()
        .file("src/cpuid.c")
        .compile("libcpuid.a");

}

#[cfg(feature = "nightly")]
fn main() {
    
}

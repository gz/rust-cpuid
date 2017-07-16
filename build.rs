extern crate gcc;

#[cfg(not(feature = "nightly"))]
fn main() {
    gcc::compile_library("libcpuid.a", &["src/cpuid.c"]);
}

#[cfg(feature = "nightly")]
fn main() {
    
}

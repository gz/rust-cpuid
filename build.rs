extern crate gcc;

#[cfg(not(feature = "nightly"))]
fn main() {
    gcc::Build::new()
        .file("src/cpuid.c")
        .compile("libcpuid.a");

}

#[cfg(feature = "nightly")]
fn main() {
    
}

extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("src/cpuid.c")
        .compile("libcpuid.a");

}

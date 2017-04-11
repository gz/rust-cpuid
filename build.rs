extern crate gcc;

fn main() {
    gcc::compile_library("libcpuid.a", &["src/cpuid.c"]);
}

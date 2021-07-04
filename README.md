# cpuid [![Crates.io](https://img.shields.io/crates/v/raw_cpuid.svg)](https://crates.io/crates/raw-cpuid) [![Standard checks](https://github.com/gz/rust-cpuid/actions/workflows/standard.yml/badge.svg)](https://github.com/gz/rust-cpuid/actions/workflows/standard.yml)

A library to parse the x86 CPUID instruction, written in rust with no external dependencies. The implementation closely resembles the Intel CPUID manual description. The library does only depend on libcore.

The code should be in sync with the latest March 2018 revision of the Intel Architectures Software Developer’s Manual.

## Library usage

```rust
use raw_cpuid::CpuId;
let cpuid = CpuId::new();

if let Some(vf) = cpuid.get_vendor_info() {
    assert!(vf.as_string() == "GenuineIntel" || vf.as_string() == "AuthenticAMD");
}

let has_sse = cpuid.get_feature_info().map_or(false, |finfo| finfo.has_sse());
if has_sse {
    println!("CPU supports SSE!");
}

if let Some(cparams) = cpuid.get_cache_parameters() {
    for cache in cparams {
        let size = cache.associativity() * cache.physical_line_partitions() * cache.coherency_line_size() * cache.sets();
        println!("L{}-Cache size is {}", cache.level(), size);
    }
} else {
    println!("No cache parameter information available")
}
```

## Documentation

* [API Documentation](https://docs.rs/raw-cpuid/)
* [Examples](https://github.com/gz/rust-cpuid/tree/master/examples)

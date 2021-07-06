# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Use more idiomatic rust code in readme/doc.rs example.
- Use `str::from_utf8` instead of `str::from_utf8_unchecked` to avoid potential
  panics with the Deserialize trait ([#43](https://github.com/gz/rust-cpuid/issues/43)).
- More extensive Debug trait implementation ([#49](https://github.com/gz/rust-cpuid/pull/49))

## [9.1.0] - 2021-07-03

### Added

- A `CpuId::with_cpuid_fn` that allows to override the default cpuid function.

### Changed

- Fixed `RdtAllocationInfo.has_memory_bandwidth_allocation`: was using the wrong bit
- Fixed `capacity_mask_length` in `L3CatInfo` and `L2CatInfo`: add +1 to returned value
- Fixed `MemBwAllocationInfo.max_hba_throttling`: add +1 to returned value
- Refactored tests into a module.
- Started to add tests for Ryzen/AMD.

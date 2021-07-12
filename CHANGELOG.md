# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased v10]

 - Removed `get_extended_function_info` with new AMD support: Use `get_processor_brand_string`, `get_extended_processor_and_feature_identifiers`, `xxx` instead. (**breaking change**)
 - Removed `deterministic_address_translation_info`. Use `get_deterministic_address_translation_info` instead. (**breaking change**)
 - Renamed `model_id` and `family_id` to `base_model_id` and `base_family_id` in `FeatureInfo`. Added new `family_id` and `model_id` functions
   that compute the actual model and family according to the spec by joining base and extended family/model. (**breaking change**)
 - Extend Hypervisor enum with more variants ([#50](https://github.com/gz/rust-cpuid/pull/50)) (**breaking change**)
 - Remove `has_rdseet` function (deprecated since 3.2), clients should use the correctly named `has_rdseed` function instead (**breaking change**).
 - Updated Debug trait for SGX iterators.
 - Make CpuId derive Clone and Copy ([#53](https://github.com/gz/rust-cpuid/pull/53))
 - Improved documentation in some places by adding leaf numbers.
 
## [9.1.1] - 2021-07-06

### Changed

- Use more idiomatic rust code in readme/doc.rs example.
- Use `str::from_utf8` instead of `str::from_utf8_unchecked` to avoid potential
  panics with the Deserialize trait ([#43](https://github.com/gz/rust-cpuid/issues/43)).
- More extensive Debug trait implementation ([#49](https://github.com/gz/rust-cpuid/pull/49))
- Fix 2 clippy warnings

## [9.1.0] - 2021-07-03

### Added

- A `CpuId::with_cpuid_fn` that allows to override the default cpuid function.

### Changed

- Fixed `RdtAllocationInfo.has_memory_bandwidth_allocation`: was using the wrong bit
- Fixed `capacity_mask_length` in `L3CatInfo` and `L2CatInfo`: add +1 to returned value
- Fixed `MemBwAllocationInfo.max_hba_throttling`: add +1 to returned value
- Refactored tests into a module.
- Started to add tests for Ryzen/AMD.

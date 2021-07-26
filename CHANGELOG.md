# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
 - AMD SVM feature leaf (0x8000_000A)
 - Added methods to display upper 64-96 bits of processor serial number (`serial_all`, `serial_upper`)
 - Implement `Display` for `CacheType`

## [10.0.0] - 2021-07-14

### Breaking changes for v10

- Removed `get_extended_function_info` / `ExtendedFunctionInfo` due to added AMD support: Use
  `get_processor_brand_string`,
  `get_extended_processor_and_feature_identifiers`, `get_l1_cache_and_tlb_info`,
  `get_l2_l3_cache_and_tlb_info`, `get_advanced_power_mgmt_info`,
  `get_processor_capacity_feature_info` instead:

  Migration guide for replacing `get_extended_function_info` / `ExtendedFunctionInfo`:

  | <= v9                      | >= v10                                                                         |
  | -------------------------- | ------------------------------------------------------------------------------ |
  | `processor_brand_string()` | `CpuId.get_processor_brand_string`                                             |
  | `cache_line_size()`        | `Cpuid.get_l2_l3_cache_and_tlb_info().l2cache_line_size()`                     |
  | `l2_associativity()`       | `Cpuid.get_l2_l3_cache_and_tlb_info().l2cache_associativity()`                 |
  | `cache_size()`             | `Cpuid.get_l2_l3_cache_and_tlb_info().l2cache_size()`                          |
  | `physical_address_bits()`  | `CpuId.get_processor_capacity_feature_info().physical_address_bits()`          |
  | `linear_address_bits()`    | `CpuId.get_processor_capacity_feature_info().linear_address_bits()`            |
  | `has_invariant_tsc()`      | `CpuId.get_advanced_power_mgmt_info.has_invariant_tsc()`                       |
  | `has_lahf_sahf()`          | `CpuId.get_extended_processor_and_feature_identifiers().has_lahf_sahf()`       |
  | `has_lzcnt()`              | `CpuId.get_extended_processor_and_feature_identifiers().has_lzcnt()`           |
  | `has_prefetchw()`          | `CpuId.get_extended_processor_and_feature_identifiers().has_prefetchw()`       |
  | `has_syscall_sysret()`     | `CpuId.get_extended_processor_and_feature_identifiers().has_syscall_sysret()`  |
  | `has_execute_disable()`    | `CpuId.get_extended_processor_and_feature_identifiers().has_execute_disable()` |
  | `has_1gib_pages()`         | `CpuId.get_extended_processor_and_feature_identifiers().has_1gib_pages()`      |
  | `has_rdtscp()`             | `CpuId.get_extended_processor_and_feature_identifiers().has_rdtscp()`          |
  | `has_64bit_mode()`         | `CpuId.get_extended_processor_and_feature_identifiers().has_64bit_mode()`      |

- Removed `CpuId.deterministic_address_translation_info`. Use
  `CpuId.get_deterministic_address_translation_info` instead.
- Renamed `model_id` and `family_id` to `base_model_id` and `base_family_id` in
  `FeatureInfo`. Added new `family_id` and `model_id` functions that compute the actual
  model and family according to the spec by joining base and extended family/model.
- Extend Hypervisor enum with more variants
  ([#50](https://github.com/gz/rust-cpuid/pull/50))
- Remove `has_rdseet` function (deprecated since 3.2), clients should use the correctly
  named `has_rdseed` function instead.

  Migration guide for `cpuid.get_feature_info()`:

  | <= v9           | >= v10          |
  | -----------     | -----------     |
  | `has_rdseet()`  | `has_rdseed()`  |

- Removed `Default` traits for most structs. `default()` should not be used anymore.

### Changes

- Updated Debug trait for SGX iterators.
- Make CpuId derive Clone and Copy ([#53](https://github.com/gz/rust-cpuid/pull/53))
- Improved documentation in some places by adding leaf numbers.
- Updated AMD leaf 0x8000_001f (Encrypted Memory) to latest manual.
- `ProcessorBrandString.as_str()` now trims the returned string.
- Fix `RdtAllocationInfo.memory_bandwidth_allocation()` which was using l2cat
  availability to determine if it exists.

### Added

- Added AMD support for leaf 0x8000_0001
- Added AMD support for leaf 0x8000_0005
- Added AMD support for leaf 0x8000_0006
- Added AMD support for leaf 0x8000_0007
- Added AMD support for leaf 0x8000_0008

### Deprecated

- `VendorInfo.as_string()` is deprecated in favor of `VendorInfo.as_str()`
- `SoCVendorBrand.as_string()` is deprecated in favor of `SoCVendorBrand.as_str()`

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

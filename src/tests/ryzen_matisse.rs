use crate::{CpuId, CpuIdResult, TopologyType};
use phf::phf_map;

/// Raw dump of ryzen mantisse cpuid values.
///
// Key format is (eax << 32 | ecx) e.g., two 32 bit values packed in one 64 bit value
static CPUID_VALUE_MAP: phf::Map<u64, CpuIdResult> = phf_map! {
    0x00000000_00000000u64 => CpuIdResult { eax: 0x00000010, ebx: 0x68747541, ecx: 0x444d4163, edx: 0x69746e65 },
    0x00000001_00000000u64 => CpuIdResult { eax: 0x00870f10, ebx: 0x000c0800, ecx: 0x7ed8320b, edx: 0x178bfbff },
    0x00000002_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x00000003_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x00000005_00000000u64 => CpuIdResult { eax: 0x00000040, ebx: 0x00000040, ecx: 0x00000003, edx: 0x00000011 },
    0x00000006_00000000u64 => CpuIdResult { eax: 0x00000004, ebx: 0x00000000, ecx: 0x00000001, edx: 0x00000000 },
    0x00000007_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x219c91a9, ecx: 0x00400004, edx: 0x00000000 },
    0x00000008_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x00000009_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000a_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000b_00000000u64 => CpuIdResult { eax: 0x00000001, ebx: 0x00000002, ecx: 0x00000100, edx: 0x00000000 },
    0x0000000b_00000001u64 => CpuIdResult { eax: 0x00000007, ebx: 0x0000000c, ecx: 0x00000201, edx: 0x00000000 },
    0x0000000c_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000d_00000000u64 => CpuIdResult { eax: 0x00000207, ebx: 0x00000340, ecx: 0x00000380, edx: 0x00000000 },
    0x0000000d_00000001u64 => CpuIdResult { eax: 0x0000000f, ebx: 0x00000340, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000d_00000002u64 => CpuIdResult { eax: 0x00000100, ebx: 0x00000240, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000d_00000009u64 => CpuIdResult { eax: 0x00000040, ebx: 0x00000340, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000e_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000f_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x000000ff, ecx: 0x00000000, edx: 0x00000002 },
    0x0000000f_00000001u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000040, ecx: 0x000000ff, edx: 0x00000007 },
    0x00000010_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000002, ecx: 0x00000000, edx: 0x00000000 },
    0x00000010_00000001u64 => CpuIdResult { eax: 0x0000000f, ebx: 0x00000000, ecx: 0x00000004, edx: 0x0000000f },
    0x20000000_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000000_00000000u64 => CpuIdResult { eax: 0x80000020, ebx: 0x68747541, ecx: 0x444d4163, edx: 0x69746e65 },
    0x80000001_00000000u64 => CpuIdResult { eax: 0x00870f10, ebx: 0x20000000, ecx: 0x75c237ff, edx: 0x2fd3fbff },
    0x80000002_00000000u64 => CpuIdResult { eax: 0x20444d41, ebx: 0x657a7952, ecx: 0x2035206e, edx: 0x30303633 },
    0x80000003_00000000u64 => CpuIdResult { eax: 0x2d362058, ebx: 0x65726f43, ecx: 0x6f725020, edx: 0x73736563 },
    0x80000004_00000000u64 => CpuIdResult { eax: 0x2020726f, ebx: 0x20202020, ecx: 0x20202020, edx: 0x00202020 },
    0x80000005_00000000u64 => CpuIdResult { eax: 0xff40ff40, ebx: 0xff40ff40, ecx: 0x20080140, edx: 0x20080140 },
    0x80000006_00000000u64 => CpuIdResult { eax: 0x48006400, ebx: 0x68006400, ecx: 0x02006140, edx: 0x01009140 },
    0x80000007_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x0000001b, ecx: 0x00000000, edx: 0x00006799 },
    0x80000008_00000000u64 => CpuIdResult { eax: 0x00003030, ebx: 0x010eb757, ecx: 0x0000700b, edx: 0x00010000 },
    0x80000009_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000a_00000000u64 => CpuIdResult { eax: 0x00000001, ebx: 0x00008000, ecx: 0x00000000, edx: 0x0013bcff },
    0x8000000b_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000c_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000d_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000e_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000f_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000010_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000011_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000012_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000013_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000014_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000015_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000016_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000017_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000018_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000019_00000000u64 => CpuIdResult { eax: 0xf040f040, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001a_00000000u64 => CpuIdResult { eax: 0x00000006, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001b_00000000u64 => CpuIdResult { eax: 0x000003ff, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001c_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001d_00000000u64 => CpuIdResult { eax: 0x00004121, ebx: 0x01c0003f, ecx: 0x0000003f, edx: 0x00000000 },
    0x8000001d_00000001u64 => CpuIdResult { eax: 0x00004122, ebx: 0x01c0003f, ecx: 0x0000003f, edx: 0x00000000 },
    0x8000001d_00000002u64 => CpuIdResult { eax: 0x00004143, ebx: 0x01c0003f, ecx: 0x000003ff, edx: 0x00000002 },
    0x8000001d_00000003u64 => CpuIdResult { eax: 0x00014163, ebx: 0x03c0003f, ecx: 0x00003fff, edx: 0x00000001 },
    0x8000001e_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000100, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001f_00000000u64 => CpuIdResult { eax: 0x0001000f, ebx: 0x0000016f, ecx: 0x000001fd, edx: 0x00000001 },
    0x80000020_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000002, ecx: 0x00000000, edx: 0x00000000 },
    0x80000020_00000001u64 => CpuIdResult { eax: 0x0000000b, ebx: 0x00000000, ecx: 0x00000000, edx: 0x0000000f },
    0x80860000_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0xc0000000_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
};

fn cpuid_reader(eax: u32, ecx: u32) -> CpuIdResult {
    let key = (eax as u64) << u32::BITS | ecx as u64;
    CPUID_VALUE_MAP[&key]
}

/// Check that vendor is AuthenticAMD.
#[test]
fn vendor_check() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let v = cpuid.get_vendor_info().expect("Need to find vendor info");
    assert_eq!(v.as_string(), "AuthenticAMD");
}

/// Check feature info gives correct values for CPU
#[test]
fn version_info() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let f = cpuid.get_feature_info().expect("Need to find feature info");

    assert_eq!(f.family_id(), 0xf);
    assert_eq!(f.model_id(), 0x1);
    assert_eq!(f.stepping_id(), 0x0);
    assert_eq!(f.extended_family_id(), 0x8);
    assert_eq!(f.extended_model_id(), 0x7);
    assert_eq!(f.brand_index(), 0x0);
    assert_eq!(f.cflush_cache_line_size(), 0x8);
    assert_eq!(f.max_logical_processor_ids(), 0xc);

    assert!(f.has_fpu());
    assert!(f.has_vme());
    assert!(f.has_de());
    assert!(f.has_pse());
    assert!(f.has_tsc());
    assert!(f.has_msr());
    assert!(f.has_pae());
    assert!(f.has_mce());
    assert!(f.has_cmpxchg8b());
    assert!(f.has_apic());
    assert!(f.has_sysenter_sysexit());
    assert!(f.has_mtrr());
    assert!(f.has_pge());
    assert!(f.has_mca());
    assert!(f.has_cmov());
    assert!(f.has_pat());
    assert!(f.has_pse36());
    assert!(!f.has_psn());
    assert!(f.has_clflush());
    assert!(!f.has_ds());
    assert!(!f.has_acpi());
    assert!(f.has_mmx());
    assert!(f.has_fxsave_fxstor());
    assert!(f.has_sse());
    assert!(f.has_sse2());
    assert!(!f.has_ss());
    assert!(f.has_htt());
    assert!(!f.has_tm());
    assert!(!f.has_pbe());

    assert!(f.has_sse3());
    assert!(f.has_pclmulqdq());
    assert!(!f.has_ds_area());
    assert!(f.has_monitor_mwait());
    assert!(!f.has_cpl());
    assert!(!f.has_vmx());
    assert!(!f.has_smx());
    assert!(!f.has_eist());
    assert!(!f.has_tm2());
    assert!(f.has_ssse3());
    assert!(!f.has_cnxtid());
    // has_SDBG
    assert!(f.has_fma());
    assert!(f.has_cmpxchg16b());
    // xTPR
    assert!(!f.has_pdcm());
    assert!(!f.has_pcid());
    assert!(!f.has_dca());
    assert!(f.has_sse41());
    assert!(f.has_sse42());
    assert!(!f.has_x2apic());
    assert!(f.has_movbe());
    assert!(f.has_popcnt());
    assert!(!f.has_tsc_deadline());
    assert!(f.has_aesni());
    assert!(f.has_xsave());
    assert!(f.has_oxsave());
    assert!(f.has_avx());
    assert!(f.has_f16c());
    assert!(f.has_rdrand());
    assert!(!f.has_hypervisor());
}

#[test]
fn cache_info() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    assert!(cpuid.get_cache_info().is_none(), "Not supported by AMD");
}

#[test]
fn processor_serial() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    assert!(
        cpuid.get_processor_serial().is_none(),
        "Not supported by AMD"
    );
}

#[test]
fn monitor_mwait() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let mw = cpuid.get_monitor_mwait_info().expect("Leaf is supported");
    assert_eq!(mw.largest_monitor_line(), 64);
    assert_eq!(mw.smallest_monitor_line(), 64);
    assert!(mw.interrupts_as_break_event());
    assert!(mw.extensions_supported());
    // supported_cX_states functions are not supported according to the manual
}

#[test]
fn thermal_power() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let mw = cpuid.get_thermal_power_info().expect("Leaf is supported");

    assert_eq!(mw.dts_irq_threshold(), 0x0);
    assert!(!mw.has_dts());
    assert!(mw.has_arat());
    assert!(!mw.has_turbo_boost());
    assert!(!mw.has_pln());
    assert!(!mw.has_ecmd());
    assert!(!mw.has_ptm());
    assert!(!mw.has_hwp());
    assert!(!mw.has_hwp_notification());
    assert!(!mw.has_hwp_activity_window());
    assert!(!mw.has_hwp_energy_performance_preference());
    assert!(!mw.has_hwp_package_level_request());
    assert!(!mw.has_hdc());
    assert!(!mw.has_turbo_boost3());
    assert!(!mw.has_hwp_capabilities());
    assert!(!mw.has_hwp_peci_override());
    assert!(!mw.has_flexible_hwp());
    assert!(!mw.has_hwp_fast_access_mode());
    assert!(!mw.has_ignore_idle_processor_hwp_request());
    assert!(mw.has_hw_coord_feedback());
    assert!(!mw.has_energy_bias_pref());
}

#[test]
fn extended_features() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let e = cpuid
        .get_extended_feature_info()
        .expect("Leaf is supported");

    assert!(e.has_fsgsbase());
    assert!(!e.has_tsc_adjust_msr());
    assert!(e.has_bmi1());
    assert!(!e.has_hle());
    assert!(e.has_avx2());
    assert!(!e.has_fdp());
    assert!(e.has_smep());
    assert!(e.has_bmi2());
    assert!(!e.has_rep_movsb_stosb());
    assert!(!e.has_invpcid());
    assert!(!e.has_rtm());
    assert!(e.has_rdtm());
    assert!(!e.has_fpu_cs_ds_deprecated());
    assert!(!e.has_mpx());
    assert!(e.has_rdta());
    assert!(e.has_rdseed());
    assert!(e.has_adx());
    assert!(e.has_smap());
    assert!(e.has_clflushopt());
    assert!(!e.has_processor_trace());
    assert!(e.has_sha());
    assert!(!e.has_sgx());
    assert!(!e.has_avx512f());
    assert!(!e.has_avx512dq());
    assert!(!e.has_avx512_ifma());
    assert!(!e.has_avx512pf());
    assert!(!e.has_avx512er());
    assert!(!e.has_avx512cd());
    assert!(!e.has_avx512bw());
    assert!(!e.has_avx512vl());
    assert!(e.has_clwb());
    assert!(!e.has_prefetchwt1());
    assert!(e.has_umip());
    assert!(!e.has_pku());
    assert!(!e.has_ospke());
    assert!(e.has_rdpid());
    assert!(!e.has_sgx_lc());
    assert_eq!(e.mawau_value(), 0x0);
}

#[test]
fn direct_cache_access() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    assert!(
        cpuid.get_direct_cache_access_info().is_none(),
        "Not supported by AMD"
    );
}

#[test]
fn perfmon_info() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    assert!(
        cpuid.get_performance_monitoring_info().is_none(),
        "Not supported by AMD"
    );
}

#[test]
fn extended_topology_info() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let mut e = cpuid
        .get_extended_topology_info()
        .expect("Leaf is supported");

    let t = e.next().expect("Have level 0");
    assert_eq!(t.processors(), 2);
    assert_eq!(t.level_number(), 0);
    assert_eq!(t.level_type(), TopologyType::SMT);
    assert_eq!(t.x2apic_id(), 0x0);
    assert_eq!(t.shift_right_for_next_apic_id(), 0x1);

    let t = e.next().expect("Have level 1");
    assert_eq!(t.processors(), 12);
    assert_eq!(t.level_number(), 1);
    assert_eq!(t.level_type(), TopologyType::Core);
    assert_eq!(t.x2apic_id(), 0x0);
    assert_eq!(t.shift_right_for_next_apic_id(), 0x7);
}

#[test]
fn extended_state_info() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let e = cpuid.get_extended_state_info().expect("Leaf is supported");

    assert!(e.xcr0_supports_legacy_x87());
    assert!(e.xcr0_supports_sse_128());
    assert!(e.xcr0_supports_avx_256());
    assert!(!e.xcr0_supports_mpx_bndregs());
    assert!(!e.xcr0_supports_mpx_bndcsr());
    assert!(!e.xcr0_supports_avx512_opmask());
    assert!(!e.xcr0_supports_avx512_zmm_hi256());
    assert!(!e.xcr0_supports_avx512_zmm_hi16());
    assert!(e.xcr0_supports_pkru());
    assert!(!e.ia32_xss_supports_pt());
    assert!(!e.ia32_xss_supports_hdc());
    assert_eq!(e.xsave_area_size_enabled_features(), 0x00000340);
    assert_eq!(e.xsave_area_size_supported_features(), 0x00000380);
    assert!(e.has_xsaveopt());
    assert!(e.has_xsavec());
    assert!(e.has_xgetbv());
    assert!(e.has_xsaves_xrstors());
    assert_eq!(e.xsave_size(), 0x00000340);

    let mut e = e.iter();
    let ee = e.next().expect("Has level 2");
    assert_eq!(ee.size(), 256);
    assert_eq!(ee.offset(), 576);
    assert!(ee.is_in_xcr0());
    assert!(!ee.is_compacted_format());

    let ee = e.next().expect("Has level 9");
    assert_eq!(ee.size(), 64);
    assert_eq!(ee.offset(), 832);
    assert!(ee.is_in_xcr0());
    assert!(!ee.is_compacted_format());
}

#[test]
fn rdt_monitoring_info() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let e = cpuid.get_rdt_monitoring_info().expect("Leaf is supported");

    assert!(e.has_l3_monitoring());
    assert_eq!(e.rmid_range(), 255);

    let l3m = e.l3_monitoring().expect("Leaf is available");
    assert_eq!(l3m.conversion_factor(), 64);
    assert_eq!(l3m.maximum_rmid_range(), 255);
    assert!(l3m.has_occupancy_monitoring());
    assert!(l3m.has_total_bandwidth_monitoring());
    assert!(l3m.has_local_bandwidth_monitoring());
}

#[test]
fn rdt_allocation_info() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let e = cpuid.get_rdt_allocation_info().expect("Leaf is supported");

    assert!(e.has_l3_cat());
    assert!(!e.has_l2_cat());
    assert!(!e.has_memory_bandwidth_allocation());
    assert!(e.l2_cat().is_none());
    assert!(e.memory_bandwidth_allocation().is_none());

    let l3c = e.l3_cat().expect("Leaf is available");
    assert_eq!(l3c.capacity_mask_length(), 0x10);
    assert_eq!(l3c.isolation_bitmap(), 0x0);
    assert_eq!(l3c.highest_cos(), 15);
    assert!(l3c.has_code_data_prioritization());
}

#[test]
fn remaining_unsupported_leafs() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);

    assert!(cpuid.get_sgx_info().is_none());
    assert!(cpuid.get_processor_trace_info().is_none());
    assert!(cpuid.get_tsc_info().is_none());
    assert!(cpuid.get_processor_frequency_info().is_none());
    assert!(cpuid.deterministic_address_translation_info().is_none());
    assert!(cpuid.get_soc_vendor_info().is_none());
}

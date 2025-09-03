//! An example of loading and printing features from a CPUID dump.
extern crate raw_cpuid;

use raw_cpuid::{
    ApmInfo, CpuIdDump, CpuIdResult, ExtendedFeatureIdentification2, ExtendedFeatures,
    ExtendedProcessorFeatureIdentifiers, ExtendedState, ExtendedStateInfo, ExtendedTopologyLevel,
    FeatureInfo, L1CacheTlbInfo, L2And3CacheTlbInfo, PerformanceOptimizationInfo,
    ProcessorCapacityAndFeatureInfo, ProcessorTopologyInfo, ThermalPowerInfo, Tlb1gbPageInfo,
    Vendor, VendorInfo,
};

// Construct a CPUID profile that looks something like a virtual machine might see on an AMD Milan
// (e.g. 7003-series) processor.
fn synthetic_milan() -> CpuIdDump {
    let mut cpuid = raw_cpuid::CpuId::with_cpuid_reader(CpuIdDump::new());
    let leaf = VendorInfo::amd();
    cpuid.set_vendor_info(Some(leaf)).unwrap();
    cpuid.set_extended_function_info(Some(leaf)).unwrap();

    let mut leaf = FeatureInfo::new(Vendor::Amd);

    // Set up EAX
    leaf.set_extended_family_id(0x00);
    leaf.set_extended_family_id(0xA);
    leaf.set_base_family_id(0x0F);
    leaf.set_base_model_id(0x01);
    leaf.set_stepping_id(0x01);

    // Set up EBX
    leaf.set_brand_index(0);
    leaf.set_cflush_cache_line_size(8);
    leaf.set_initial_local_apic_id(0); // Populated dynamically in a real system.
    leaf.set_max_logical_processor_ids(0); // Populated dynamically in a real system.

    // Set up ECX
    leaf.set_sse3(true);
    leaf.set_pclmulqdq(true);
    leaf.set_ds_area(false);
    leaf.set_monitor_mwait(false);

    leaf.set_cpl(false);
    leaf.set_vmx(false);
    leaf.set_smx(false);
    leaf.set_eist(false);

    leaf.set_tm2(false);
    leaf.set_ssse3(true);
    leaf.set_cnxtid(false);
    // bit 11 is reserved

    leaf.set_fma(true);
    leaf.set_cmpxchg16b(true);
    // bit 14 is reserved
    leaf.set_pdcm(false);

    //bit 16 is reserved
    leaf.set_pcid(false);
    leaf.set_dca(false);
    leaf.set_sse41(true);

    leaf.set_sse42(true);
    leaf.set_x2apic(true);
    leaf.set_movbe(true);
    leaf.set_popcnt(true);

    leaf.set_tsc_deadline(false);
    leaf.set_aesni(true);
    leaf.set_xsave(true);
    leaf.set_oxsave(false); // Populated dynamically in a real system.

    leaf.set_avx(true);
    leaf.set_f16c(true);
    leaf.set_rdrand(true);
    leaf.set_hypervisor(true); // This CPUID leaf will be presented to hypervisor guests

    // Set up EDX
    leaf.set_fpu(true);
    leaf.set_vme(true);
    leaf.set_de(true);
    leaf.set_pse(true);

    leaf.set_tsc(true);
    leaf.set_msr(true);
    leaf.set_pae(true);
    leaf.set_mce(true);

    leaf.set_cmpxchg8b(true);
    leaf.set_apic(true);
    // bit 10 is reserved
    leaf.set_sysenter_sysexit(true);

    leaf.set_mtrr(true);
    leaf.set_pge(true);
    leaf.set_mca(true);
    leaf.set_cmov(true);

    leaf.set_pat(true);
    leaf.set_pse36(true);
    // bit 18 is reserved
    leaf.set_clflush(true);

    // bit 20 is reserved
    // bit 21 is reserved
    // bit 22 is reserved
    leaf.set_mmx(true);

    leaf.set_fxsave_fxstor(true);
    leaf.set_sse(true);
    leaf.set_sse2(true);
    // bit 27 is reserved

    leaf.set_htt(true);
    // bits 29-31 are not used here.

    cpuid.set_feature_info(Some(leaf)).unwrap();

    // Leaf 2, 3, 4: all skipped on AMD

    // Leaf 5: Monitor and MWait. These are hidden from the guest, so zero this leaf.
    cpuid.set_monitor_mwait_info(None).unwrap();

    // Leaf 6: Power management and some more feature bits.
    let mut leaf = ThermalPowerInfo::empty();
    leaf.set_arat(true);
    leaf.set_hw_coord_feedback(false);

    cpuid.set_thermal_power_info(Some(leaf)).unwrap();

    // Leaf 7: Extended features
    let mut leaf = ExtendedFeatures::new();
    leaf.set_fsgsbase(true);
    leaf.set_tsc_adjust_msr(false);
    leaf.set_sgx(false);
    leaf.set_bmi1(true);

    leaf.set_hle(false);
    leaf.set_avx2(true);
    leaf.set_fdp(false);
    leaf.set_smep(true);

    leaf.set_bmi2(true);
    leaf.set_rep_movsb_stosb(true); // aka ERMS
    leaf.set_invpcid(false);
    // Bit 11 is reserved on AMD

    // PQM (bit 12) is clear here
    // Bit 13 is reserved on AMD
    // Bit 14 is reserved on AMD
    // Bit 15 is reserved on AMD

    leaf.set_avx512f(false);
    leaf.set_avx512dq(false);
    leaf.set_rdseed(true);
    leaf.set_adx(true);

    leaf.set_smap(true);
    leaf.set_avx512_ifma(false);
    // Bit 22 is reserved on AMD
    leaf.set_clflushopt(true);

    leaf.set_clwb(true);
    // Bit 25 is reserved on AMD
    // Bit 26 is reserved on AMD
    // Bit 27 is reserved on AMD

    leaf.set_avx512cd(false);
    leaf.set_sha(true);
    leaf.set_avx512bw(false);
    leaf.set_avx512vl(false);

    // Set up leaf 7 ECX

    // Bit 0 is reserved on AMD
    leaf.set_avx512vbmi(false);
    leaf.set_umip(false);
    leaf.set_pku(false);

    leaf.set_ospke(false);
    // Bit 5 is reserved on AMD
    leaf.set_avx512vbmi2(false);
    leaf.set_cet_ss(false);

    leaf.set_gfni(false);
    leaf.set_vaes(true);
    leaf.set_vpclmulqdq(true);
    leaf.set_avx512vnni(false);

    leaf.set_avx512bitalg(false);
    // Bit 13 is reserved on AMD
    leaf.set_avx512vpopcntdq(false);
    // Bit 15 is reserved on AMD

    // Bits 16 through 31 are either reserved or zero on Milan.

    // Set up leaf 7 EDX
    leaf.set_fsrm(true);
    cpuid.set_extended_feature_info(Some(leaf)).unwrap();

    // Set up extended topology info (leaf Bh)
    let mut levels = Vec::new();

    let mut topo_level1 = ExtendedTopologyLevel::empty();
    topo_level1.set_shift_right_for_next_apic_id(1);
    topo_level1.set_processors(2);
    topo_level1.set_level_number(0);
    topo_level1.set_level_type(1);

    levels.push(topo_level1);

    let mut topo_level2 = ExtendedTopologyLevel::empty();
    topo_level2.set_shift_right_for_next_apic_id(7);
    topo_level2.set_processors(32);
    topo_level2.set_level_number(1);
    topo_level2.set_level_type(2);

    levels.push(topo_level2);

    let mut topo_level3 = ExtendedTopologyLevel::empty();
    topo_level3.set_level_number(2);
    topo_level3.set_level_type(0); // This level is invalid.

    levels.push(topo_level3);
    cpuid
        .set_extended_topology_info(Some(levels.as_slice()))
        .unwrap();

    // TODO: it is not great to pass another `CpuIdDump` here just to create the type..
    let mut state = ExtendedStateInfo::empty(CpuIdDump::new());
    state.set_xcr0_supports_legacy_x87(true);
    state.set_xcr0_supports_sse_128(true);
    state.set_xcr0_supports_avx_256(true);
    state.set_xsave_area_size_enabled_features(0x340); // Populated dynamically in a real system.
    state.set_xsave_area_size_supported_features(0x340);

    state.set_xsaveopt(true);
    state.set_xsavec(true);
    state.set_xgetbv(true);
    state.set_xsave_size(0x340);

    let mut leaves = state.into_leaves().to_vec();
    let mut ymm_state = ExtendedState::empty();
    ymm_state.set_size(0x100);
    ymm_state.set_offset(0x240);
    leaves.push(Some(ymm_state.into_leaf()));

    cpuid.set_extended_state_info(Some(&leaves[..])).unwrap();

    let mut leaf = ExtendedProcessorFeatureIdentifiers::empty(Vendor::Amd);
    // This is the same as the leaf 1 EAX configured earlier.
    leaf.set_extended_signature(0x00A00F11);

    // Set up EBX
    leaf.set_pkg_type(0x4);

    // Set up ECX
    leaf.set_lahf_sahf(true);
    leaf.set_cmp_legacy(false);
    leaf.set_svm(false);
    leaf.set_ext_apic_space(false);

    leaf.set_alt_mov_cr8(true);
    leaf.set_lzcnt(true);
    leaf.set_sse4a(true);
    leaf.set_misaligned_sse_mode(true);

    leaf.set_prefetchw(true);
    // Probably set in hardware, but hide this and the MSR from guests.
    leaf.set_osvw(false);
    leaf.set_ibs(false);
    leaf.set_xop(false);

    leaf.set_skinit(false);
    leaf.set_wdt(false);
    // Bit 15 is reserved here.
    leaf.set_lwp(false);

    leaf.set_fma4(false); // Not on Milan

    // Bits 17-19 are reserved

    // Bit 20 is reserved
    // Bit 21 is reserved, formerly TBM
    leaf.set_topology_extensions(true);
    leaf.set_perf_cntr_extensions(false);

    leaf.set_nb_perf_cntr_extensions(false);
    // Bit 25 is reserved
    leaf.set_data_access_bkpt_extension(true);
    leaf.set_perf_tsc(false);

    leaf.set_perf_cntr_llc_extensions(false);
    leaf.set_monitorx_mwaitx(false);
    leaf.set_addr_mask_extension(true);
    // Bit 31 is reserved

    // Set up EDX
    leaf.set_syscall_sysret(true);
    leaf.set_execute_disable(true);
    leaf.set_mmx_extensions(true);
    leaf.set_fast_fxsave_fxstor(true);
    leaf.set_1gib_pages(true);
    leaf.set_rdtscp(false);
    leaf.set_64bit_mode(true);

    cpuid
        .set_extended_processor_and_feature_identifiers(Some(leaf))
        .unwrap();

    // Leaves 8000_0002 through 8000_0005
    cpuid
        .set_processor_brand_string(Some(b"AMD EPYC Processor"))
        .unwrap();

    // Set up L1 cache+TLB info (leaf 8000_0005h)
    let mut leaf = L1CacheTlbInfo::empty();

    leaf.set_itlb_2m_4m_size(0x40);
    leaf.set_itlb_2m_4m_associativity(0xff);
    leaf.set_dtlb_2m_4m_size(0x40);
    leaf.set_dtlb_2m_4m_associativity(0xff);

    leaf.set_itlb_4k_size(0x40);
    leaf.set_itlb_4k_associativity(0xff);
    leaf.set_dtlb_4k_size(0x40);
    leaf.set_dtlb_4k_associativity(0xff);

    leaf.set_dcache_line_size(0x40);
    leaf.set_dcache_lines_per_tag(0x01);
    leaf.set_dcache_associativity(0x08);
    leaf.set_dcache_size(0x20);

    leaf.set_icache_line_size(0x40);
    leaf.set_icache_lines_per_tag(0x01);
    leaf.set_icache_associativity(0x08);
    leaf.set_icache_size(0x20);

    cpuid.set_l1_cache_and_tlb_info(Some(leaf)).unwrap();

    // Set up L2 and L3 cache+TLB info (leaf 8000_0006h)
    let mut leaf = L2And3CacheTlbInfo::empty();

    // Set up leaf 8000_0006h EAX
    leaf.set_itlb_2m_4m_size(0x200);
    leaf.set_itlb_2m_4m_associativity(0x2);
    leaf.set_dtlb_2m_4m_size(0x800);
    leaf.set_dtlb_2m_4m_associativity(0x4);

    // Set up leaf 8000_0006h EBX
    leaf.set_itlb_4k_size(0x200);
    leaf.set_itlb_4k_associativity(0x4);
    leaf.set_dtlb_4k_size(0x800);
    leaf.set_dtlb_4k_associativity(0x6);

    // Set up leaf 8000_0006h ECX
    leaf.set_l2cache_line_size(0x40);
    leaf.set_l2cache_lines_per_tag(0x1);
    leaf.set_l2cache_associativity(0x6);
    leaf.set_l2cache_size(0x0200);

    // Set up leaf 8000_0006h EDX
    leaf.set_l3cache_line_size(0x40);
    leaf.set_l3cache_lines_per_tag(0x1);
    leaf.set_l3cache_associativity(0x9);
    leaf.set_l3cache_size(0x0200);

    cpuid.set_l2_l3_cache_and_tlb_info(Some(leaf)).unwrap();

    // Set up advanced power management info (leaf 8000_0007h)
    let mut leaf = ApmInfo::empty();
    leaf.set_invariant_tsc(true);
    cpuid.set_advanced_power_mgmt_info(Some(leaf)).unwrap();

    // Set up processor capacity info (leaf 8000_0008h)
    let mut leaf = ProcessorCapacityAndFeatureInfo::empty();

    // Set up leaf 8000_0008 EAX
    leaf.set_physical_address_bits(0x30);
    leaf.set_linear_address_bits(0x30);
    leaf.set_guest_physical_address_bits(0);

    // St up leaf 8000_0008 EBX
    leaf.set_cl_zero(true);
    leaf.set_restore_fp_error_ptrs(true);
    leaf.set_wbnoinvd(true);

    leaf.set_num_phys_threads(1); // Populated dynamically in a real system.
    leaf.set_apic_id_size(0);
    leaf.set_perf_tsc_size(0);

    leaf.set_invlpgb_max_pages(0);
    leaf.set_max_rdpru_id(0);

    cpuid
        .set_processor_capacity_feature_info(Some(leaf))
        .unwrap();

    // Leaf 8000_000Ah is zeroed out for guests.
    cpuid.set_svm_info(None).unwrap();

    // Set up TLB information for 1GiB pages (leaf 8000_0019h)
    let mut leaf = Tlb1gbPageInfo::empty();
    leaf.set_dtlb_l1_1gb_associativity(0xF);
    leaf.set_dtlb_l1_1gb_size(0x40);
    leaf.set_itlb_l1_1gb_associativity(0xF);
    leaf.set_itlb_l1_1gb_size(0x40);
    leaf.set_dtlb_l2_1gb_associativity(0xF);
    leaf.set_dtlb_l2_1gb_size(0x40);
    leaf.set_itlb_l2_1gb_associativity(0);
    leaf.set_itlb_l2_1gb_size(0);
    cpuid.set_tlb_1gb_page_info(Some(leaf)).unwrap();

    // Set up processor optimization info (leaf 8000_001Ah)
    let mut leaf = PerformanceOptimizationInfo::empty();
    leaf.set_movu(true);
    leaf.set_fp256(true);
    cpuid.set_performance_optimization_info(Some(leaf)).unwrap();

    // Leaf 8000_001B
    // TODO: no support for leaf 8000_001B, but zero is what we wanted.
    // Leaf 8000_001C
    // TODO: no support for leaf 8000_001C, but zero is what we wanted.

    // Leaf 8000_001D
    let levels = vec![
        CpuIdResult {
            eax: 0x00000121,
            ebx: 0x01C0003F,
            ecx: 0x0000003F,
            edx: 0x00000000,
        },
        CpuIdResult {
            eax: 0x00000122,
            ebx: 0x01C0003F,
            ecx: 0x0000003F,
            edx: 0x00000000,
        },
        CpuIdResult {
            eax: 0x00000143,
            ebx: 0x01C0003F,
            ecx: 0x000003FF,
            edx: 0x00000002,
        },
        CpuIdResult {
            eax: 0x00000163,
            ebx: 0x03C0003F,
            ecx: 0x00007FFF,
            edx: 0x00000001,
        },
    ];
    cpuid
        .set_extended_cache_parameters(Some(levels.as_slice()))
        .unwrap();

    let mut leaf = ProcessorTopologyInfo::empty();
    leaf.set_threads_per_core(2);
    cpuid.set_processor_topology_info(Some(leaf)).unwrap();

    cpuid.set_memory_encryption_info(None).unwrap();

    let mut leaf = ExtendedFeatureIdentification2::empty();
    leaf.set_no_nested_data_bp(true);
    leaf.set_lfence_always_serializing(true);
    leaf.set_null_select_clears_base(true);
    cpuid
        .set_extended_feature_identification_2(Some(leaf))
        .unwrap();

    cpuid.into_source()
}

fn main() {
    let synthetic_milan = synthetic_milan();

    for (leaf, subleaf, regs) in synthetic_milan.into_iter() {
        let place = match subleaf {
            Some(subleaf) => format!("leaf {:08x}h.{}h", leaf, subleaf),
            None => format!("leaf {:08x}h", leaf),
        };

        let CpuIdResult { eax, ebx, ecx, edx } = regs;

        println!("{place}: eax=0x{eax:08x} ebx=0x{ebx:08x} ecx=0x{ecx:08x} edx=0x{edx:08x}");
    }
}

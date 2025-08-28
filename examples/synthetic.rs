//! An example of loading and printing features from a CPUID dump.
extern crate raw_cpuid;

use raw_cpuid::{CpuIdReader, CpuIdResult, CpuIdWriter, FeatureInfo, Vendor, VendorInfo, ThermalPowerInfo, ExtendedFeatures, ExtendedTopologyLevel, ExtendedState, ExtendedStateInfo, L1CacheTlbInfo, L2And3CacheTlbInfo, ApmInfo, ProcessorCapacityAndFeatureInfo, PerformanceOptimizationInfo, Tlb1gbPageInfo};
use std::collections::HashMap;

#[derive(Debug)]
struct CpuidEntry {
    leaf: u32,
    subleaf: Option<u32>,
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl CpuidEntry {
    pub fn just_regs(&self) -> CpuIdResult {
        CpuIdResult {
            eax: self.eax,
            ebx: self.ebx,
            ecx: self.ecx,
            edx: self.edx,
        }
    }
}


#[derive(Clone)]
enum LeafOrSubleaves {
    Leaf(CpuIdResult),
    Subleaf(HashMap<u32, CpuIdResult>),
}

// TODO: Clone is necessary because CpuIdReader wants it (for leaves with more complex subleaf
// structures, like the extended topology info leaf)
//
// This implies that there's a full clone of the dump held on for those leaf-specific views, which
// is unfortunate! It's also not yet really clear how to assemble those more complex leaves for
// writer purposes.
#[derive(Clone)]
struct CpuIdDump {
    leaves: HashMap<u32, LeafOrSubleaves>,
}

impl CpuIdDump {
    // TODO: probably should just take vendor in the initial constructor here
    // (that also lets this pick the right leaf/subleaf fallback behavior from the get-go)
    fn new() -> Self {
        Self {
            leaves: HashMap::new()
        }
    }
}

const DEFAULT_LEAF: CpuIdResult = CpuIdResult {
    eax: 0,
    ebx: 0,
    ecx: 0,
    edx: 0,
};

impl CpuIdWriter for CpuIdDump {
    fn set_leaf(&mut self, leaf: u32, bits: Option<CpuIdResult>) {
        if let Some(bits) = bits {
            self.leaves.insert(leaf, LeafOrSubleaves::Leaf(bits));
        } else {
            self.leaves.remove(&leaf);
        }

        self.update_max_leaves();
    }
    fn set_subleaf(&mut self, leaf: u32, subleaf: u32, bits: Option<CpuIdResult>) {
        if let Some(bits) = bits {
            match self.leaves.entry(leaf).or_insert(LeafOrSubleaves::Subleaf(HashMap::new())) {
                LeafOrSubleaves::Leaf(_) => {
                    panic!("adding a subleaf where there's a leaf. no");
                }
                LeafOrSubleaves::Subleaf(leaves) => {
                    leaves.insert(subleaf, bits);
                }
            }
        } else {
            self.leaves.get_mut(&leaf).map(|ent| if let LeafOrSubleaves::Subleaf(leaves) = ent {
                leaves.remove(&subleaf);
            } else {
                panic!("removing a subleaf when there's a leaf. no");
            });
        }

        self.update_max_leaves();
    }
}

impl CpuIdDump {
    fn update_max_leaves(&mut self) {
        let mut max_standard = None;
        let mut max_hv = None;
        let mut max_extended = None;

        for (idx, k) in self.leaves.keys().enumerate() {
            let k = *k;
            if k < 0x40000000 {
                max_standard = Some(match max_standard {
                    None => k,
                    Some(prev) => std::cmp::max(k, prev),
                });
            } else if k < 0x80000000 {
                max_hv = Some(match max_hv {
                    None => k,
                    Some(prev) => std::cmp::max(k, prev),
                });
            } else if k < 0xc0000000 {
                max_extended = Some(match max_extended {
                    None => k,
                    Some(prev) => std::cmp::max(k, prev),
                });
            }
        }

        if let Some(eax) = max_standard {
            match self.leaves.entry(0).or_insert(LeafOrSubleaves::Leaf(CpuIdResult::empty())) {
                LeafOrSubleaves::Leaf(leaf) => {
                    leaf.eax = eax;
                }
                _ => {
                    panic!("cannot update leaf 1.EAX: leaf has subleaves?");
                }
            }
        }

        if let Some(eax) = max_hv {
            match self.leaves.entry(0x40000000).or_insert(LeafOrSubleaves::Leaf(CpuIdResult::empty())) {
                LeafOrSubleaves::Leaf(leaf) => {
                    leaf.eax = eax;
                }
                _ => {
                    panic!("cannot update leaf 0x40000000.EAX: leaf has subleaves?");
                }
            }
        }

        if let Some(eax) = max_extended {
            match self.leaves.entry(0x80000000).or_insert(LeafOrSubleaves::Leaf(CpuIdResult::empty())) {
                LeafOrSubleaves::Leaf(leaf) => {
                    leaf.eax = eax;
                }
                _ => {
                    panic!("cannot update leaf 0x80000000.EAX: leaf has subleaves?");
                }
            }
        }
    }
}

impl CpuIdReader for CpuIdDump {
    fn cpuid1(&self, leaf: u32) -> CpuIdResult {
        match self.leaves.get(&leaf) {
            Some(LeafOrSubleaves::Leaf(res)) => *res,
            Some(LeafOrSubleaves::Subleaf(subleaves)) => {
                *subleaves.get(&0).unwrap_or_else(|| {
                    // TODO: vendor-specific fallback behavior
                    &DEFAULT_LEAF
                })
            }
            None => {
                // TODO: more vendor-specific fallback behavior
                DEFAULT_LEAF
            }
        }
    }

    fn cpuid2(&self, leaf: u32, subleaf: u32) -> CpuIdResult {
        match self.leaves.get(&leaf) {
            Some(LeafOrSubleaves::Leaf(res)) => {
                // TODO: vendor-specific fallback behavior
                DEFAULT_LEAF
            }
            Some(LeafOrSubleaves::Subleaf(subleaves)) => {
                *subleaves.get(&subleaf).unwrap_or_else(|| {
                    // TODO: vendor-specific fallback behavior
                    &DEFAULT_LEAF
                })
            }
            None => {
                // TODO: more vendor-specific fallback behavior
                DEFAULT_LEAF
            }
        }
    }
}

macro_rules! cpuid_leaf {
    ($leaf:literal, $eax:literal, $ebx:literal, $ecx:literal, $edx:literal) => {
        CpuidEntry {
            leaf: $leaf,
            subleaf: None,
            eax: $eax,
            ebx: $ebx,
            ecx: $ecx,
            edx: $edx,
        }
    };
}

macro_rules! cpuid_subleaf {
    ($leaf:literal, $sl:literal, $eax:literal, $ebx:literal, $ecx:literal, $edx:literal) => {
        CpuidEntry {
            leaf: $leaf,
            subleaf: Some($sl),
            eax: $eax,
            ebx: $ebx,
            ecx: $ecx,
            edx: $edx,
        }
    };
}

const MILAN_CPUID: [CpuidEntry; 32] = [
    cpuid_leaf!(0x0, 0x0000000D, 0x68747541, 0x444D4163, 0x69746E65),
    cpuid_leaf!(0x1, 0x00A00F11, 0x00000800, 0xF6D83203, 0x078BFBFF),
    cpuid_leaf!(0x5, 0x00000000, 0x00000000, 0x00000000, 0x00000000),
    cpuid_leaf!(0x6, 0x00000004, 0x00000000, 0x00000000, 0x00000000),
    cpuid_subleaf!(
        0x7, 0x0, 0x00000000, 0x219803A9, 0x00000600, 0x00000010
    ),
    cpuid_subleaf!(
        0xB, 0x0, 0x00000001, 0x00000002, 0x00000100, 0x00000000
    ),
    cpuid_subleaf!(
        0xB, 0x1, 0x00000000, 0x00000000, 0x00000201, 0x00000000
    ),
    cpuid_subleaf!(
        0xB, 0x2, 0x00000000, 0x00000000, 0x00000002, 0x00000000
    ),
    cpuid_subleaf!(
        0xD, 0x0, 0x00000007, 0x00000340, 0x00000340, 0x00000000
    ),
    cpuid_subleaf!(
        0xD, 0x1, 0x00000007, 0x00000340, 0x00000000, 0x00000000
    ),
    cpuid_subleaf!(
        0xD, 0x2, 0x00000100, 0x00000240, 0x00000000, 0x00000000
    ),
    cpuid_leaf!(0x80000000, 0x80000021, 0x68747541, 0x444D4163, 0x69746E65),
    // ecx bit 23 should be flipped true at some point, but is currently
    // hidden and will continue to be for the moment.
    // ecx bit 3 should be masked, but is is not and advertises support for
    // unsupported extensions to LAPIC space.
    //
    // RFD 314 talks about these bits more, but we currently allow them to
    // be wrong as they have been wrong before and we'll get to them
    // individually later.
    cpuid_leaf!(0x80000001, 0x00A00F11, 0x40000000, 0x444001F1, 0x27D3FBFF),
    cpuid_leaf!(0x80000002, 0x20444D41, 0x43595045, 0x31373720, 0x36205033),
    cpuid_leaf!(0x80000003, 0x6F432D34, 0x50206572, 0x65636F72, 0x726F7373),
    cpuid_leaf!(0x80000004, 0x20202020, 0x20202020, 0x20202020, 0x00202020),
    cpuid_leaf!(0x80000005, 0xFF40FF40, 0xFF40FF40, 0x20080140, 0x20080140),
    cpuid_leaf!(0x80000006, 0x48002200, 0x68004200, 0x02006140, 0x08009140),
    cpuid_leaf!(0x80000007, 0x00000000, 0x00000000, 0x00000000, 0x00000100),
    cpuid_leaf!(0x80000008, 0x00003030, 0x00000205, 0x00000000, 0x00000000),
    cpuid_leaf!(0x8000000A, 0x00000000, 0x00000000, 0x00000000, 0x00000000),
    cpuid_leaf!(0x80000019, 0xF040F040, 0xF0400000, 0x00000000, 0x00000000),
    cpuid_leaf!(0x8000001A, 0x00000006, 0x00000000, 0x00000000, 0x00000000),
    cpuid_leaf!(0x8000001B, 0x00000000, 0x00000000, 0x00000000, 0x00000000),
    cpuid_leaf!(0x8000001C, 0x00000000, 0x00000000, 0x00000000, 0x00000000),
    cpuid_subleaf!(
        0x8000001D, 0x0, 0x00000121, 0x01C0003F, 0x0000003F, 0x00000000
    ),
    cpuid_subleaf!(
        0x8000001D, 0x1, 0x00000122, 0x01C0003F, 0x0000003F, 0x00000000
    ),
    cpuid_subleaf!(
        0x8000001D, 0x2, 0x00000143, 0x01C0003F, 0x000003FF, 0x00000002
    ),
    cpuid_subleaf!(
        0x8000001D, 0x3, 0x00000163, 0x03C0003F, 0x00007FFF, 0x00000001
    ),
    cpuid_leaf!(0x8000001E, 0x00000000, 0x00000100, 0x00000000, 0x00000000),
    cpuid_leaf!(0x8000001F, 0x00000000, 0x00000000, 0x00000000, 0x00000000),
    cpuid_leaf!(0x80000021, 0x00000045, 0x00000000, 0x00000000, 0x00000000),
];

fn synthetic_milan() -> CpuIdDump {
    let mut bits = CpuIdDump::new();
    let mut cpuid = raw_cpuid::CpuId::with_cpuid_reader(bits);
    let mut leaf = VendorInfo::amd();
    cpuid.set_vendor_info(Some(leaf));

    let mut leaf = FeatureInfo::new(Vendor::Amd);

    // Set up EAX
    leaf.set_extended_family_id(0x00);
    leaf.set_extended_family_id(0xA);
    leaf.set_base_family_id(0x0F);
    leaf.set_base_model_id(0x01);
    leaf.set_stepping_id(0x01);

    // Set up EBX
    leaf.set_brand_index(0); //
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
    leaf.set_x2apic(false); // GOT THIS WRONG IN OMICRON UGHGHGHGGHH
    leaf.set_movbe(true);
    leaf.set_popcnt(true);

    leaf.set_tsc_deadline(false);
    leaf.set_aesni(true);
    leaf.set_xsave(true);
    leaf.set_oxsave(false); // managed dynamically in practice

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

    leaf.set_htt(false); // managed dynamically in practice
    // bits 29-31 are not used here.

    // Milan Leaf 1 EAX: 0x00A00F11
    // Milan Leaf 1 EBX: 0xXXYY0800
    // Milan Leaf 1 ECX: 0xF6F83203
    // Milan Leaf 1 EDX: 0x078BFBFF
    cpuid.set_feature_info(Some(leaf));

    // Leaf 2, 3, 4: all skipped on AMD

    // Leaf 5: Monitor and MWait. All zero here.
    cpuid.set_monitor_mwait_info(None);

    // Leaf 6: Power management and .. some feature bits.
    let mut leaf = ThermalPowerInfo::empty();
    leaf.set_arat(true);
    leaf.set_hw_coord_feedback(false);

    // Milan Leaf 6 EAX 0x00000004
    // Milan Leaf 6 EBX 0x00000004
    // Milan Leaf 6 ECX 0x00000004
    // Milan Leaf 6 EDX 0x00000004
    cpuid.set_thermal_power_info(Some(leaf));

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
    leaf.set_rep_movsb_stosb(true); // ERMS
    leaf.set_invpcid(false);
    // Bit 11 is reserved on AMD

    // PQM (bit 12) is clear here
    // Bit 13 is reserved on AMD
    // Bit 14 is reserved on AMD
    // Bit 15 is reserved on AMD

    leaf.set_avx512f(false); // Not on Milan
    leaf.set_avx512dq(false); // Not on Milan
    leaf.set_rdseed(false); // False here
    leaf.set_adx(true);

    leaf.set_smap(true);
    leaf.set_avx512_ifma(false); // Not on Milan
    // Bit 22 is reserved on AMD
    leaf.set_clflushopt(true);

    leaf.set_clwb(true);
    // Bit 25 is reserved on AMD
    // Bit 26 is reserved on AMD
    // Bit 27 is reserved on AMD

    leaf.set_avx512cd(false); // Not on Milan
    leaf.set_sha(true);
    leaf.set_avx512bw(false); // Not on Milan
    leaf.set_avx512vl(false); // Not on Milan

    // Set up leaf 7 ECX

    // Bit 0 is reserved on AMD
    leaf.set_avx512vbmi(false);
    leaf.set_umip(false);
    leaf.set_pku(false);

    leaf.set_ospke(false);
    // Bit 5 is reserved on AMD
    leaf.set_avx512vbmi2(false);
    leaf.set_cet_ss(false);

    leaf.set_gfni(false); // TODO: milan??
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
    cpuid.set_extended_feature_info(Some(leaf));

    // Set up extended topology info (leaf Bh)
    let mut levels = Vec::new();

    let mut topo_level1 = ExtendedTopologyLevel::empty();
    // EAX
    // These perhaps should be dynamic based on SMT or no?
    topo_level1.set_shift_right_for_next_apic_id(1);
    // EBX
    topo_level1.set_processors(2);
    // ECX
    topo_level1.set_level_number(0);
    topo_level1.set_level_type(1); // If there's no SMT, there should be no SMT right..?

    levels.push(topo_level1);

    let mut topo_level2 = ExtendedTopologyLevel::empty();
    // ECX
    topo_level2.set_level_number(1);
    topo_level2.set_level_type(2);

    levels.push(topo_level2);

    let mut topo_level3 = ExtendedTopologyLevel::empty();
    // ECX
    topo_level3.set_level_number(2);
    topo_level3.set_level_type(0); // This level is invalid.

    levels.push(topo_level3);
    cpuid.set_extended_topology_info(Some(levels.as_slice()));

    // TODO: ok, kind of messed up to have to pass a `CpuIdDump` here..
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

    cpuid.set_extended_state_info(Some(&leaves[..]));

    // TODO: figure out leaves 8000_0000/8000_0001

    // Leaves 8000_0002 through 8000_0005
    cpuid.set_processor_brand_string(Some(b"AMD EPYC 7713P 64-Core Processor"));

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

    cpuid.set_l1_cache_and_tlb_info(Some(leaf));

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
    leaf.set_l3cache_size(0x0800);

    cpuid.set_l2_l3_cache_and_tlb_info(Some(leaf));

    // Set up advanced power management info (leaf 8000_0007h)
    let mut leaf = ApmInfo::empty();
    leaf.set_invariant_tsc(true);
    cpuid.set_advanced_power_mgmt_info(Some(leaf));

    // Set up processor capacity info (leaf 8000_0008h)
    let mut leaf = ProcessorCapacityAndFeatureInfo::empty();

    // Set up leaf 8000_0008 EAX
    leaf.set_physical_address_bits(0x30); // TODO: BREAKING
    leaf.set_linear_address_bits(0x30); // TODO: BREAKING
    leaf.set_guest_physical_address_bits(0); // TODO: BREAKING

    // St up leaf 8000_0008 EBX
    leaf.set_cl_zero(true);
    leaf.set_restore_fp_error_ptrs(true);
    leaf.set_wbnoinvd(true);

    leaf.set_num_phys_threads(1); // Populated dynamically in a real system.
    leaf.set_apic_id_size(0);
    leaf.set_perf_tsc_size(0);

    leaf.set_invlpgb_max_pages(0); // TODO: BREAKING
    leaf.set_max_rdpru_id(0); // TODO: BREAKING

    cpuid.set_processor_capacity_feature_info(Some(leaf));

    // Set up TODO: (leaf 8000_000Ah)

    // Set up TLB information for 1GiB pages (leaf 8000_0019h)
    let mut leaf = Tlb1gbPageInfo::empty();
    leaf.set_dtlb_l1_1gb_associativity(0xF);
    leaf.set_dtlb_l1_1gb_size(0x40);
    leaf.set_itlb_l1_1gb_associativity(0xF);
    leaf.set_itlb_l1_1gb_size(0x40);
    leaf.set_dtlb_l2_1gb_associativity(0xF);
    leaf.set_dtlb_l2_1gb_size(0x40);
    leaf.set_itlb_l1_1gb_associativity(0);
    leaf.set_itlb_l1_1gb_size(0);
    cpuid.set_tlb_1gb_page_info(Some(leaf)); // TODO: Would zero this

    // Set up processor optimization info (leaf 8000_001Ah)
    let mut leaf = PerformanceOptimizationInfo::empty();
    leaf.set_movu(true);
    leaf.set_fp256(true);
    cpuid.set_performance_optimization_info(Some(leaf));

    // Leaf 8000_001B
    // TODO: no support for leaf 1B?
    // Leaf 8000_001C
    // TODO: no support for leaf 1C?
    // Leaf 8000_001D

    // Leaf 8000_001Eh EBX bit 8 needs attention.
    cpuid.set_processor_topology_info(None);
    cpuid.set_memory_encryption_info(None);

    cpuid.into_source()
}

fn main() {
    let synthetic_milan = synthetic_milan();

    for leaf in MILAN_CPUID.iter() {
        let synth = if let Some(subleaf) = leaf.subleaf {
            synthetic_milan.cpuid2(leaf.leaf, subleaf)
        } else {
            synthetic_milan.cpuid1(leaf.leaf)
        };

        let regs = leaf.just_regs();

        let place = match leaf.subleaf {
            Some(subleaf) => format!("leaf {:08x}h.{}h", leaf.leaf, subleaf),
            None => format!("leaf {:08x}h", leaf.leaf)
        };

        if regs.eax != synth.eax {
            panic!("{}: eax different: expected {:08x}, was {:08x}", place, regs.eax, synth.eax);
        }

        if regs.ebx != synth.ebx {
            panic!("{}: ebx different: expected {:08x}, was {:08x}", place, regs.ebx, synth.ebx);
        }

        if regs.ecx != synth.ecx {
            panic!("{}: ecx different: expected {:08x}, was {:08x}", place, regs.ecx, synth.ecx);
        }

        if regs.edx != synth.edx {
            panic!("{}: edx different: expected {:08x}, was {:08x}", place, regs.edx, synth.edx);
        }
    }
}

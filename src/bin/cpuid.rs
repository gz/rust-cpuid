use std::fmt::Display;

use raw_cpuid::{
    Associativity, CacheType, CpuId, CpuIdResult, DatType, ExtendedRegisterStateLocation,
    SgxSectionInfo, SoCVendorBrand, TopologyType,
};
use termimad::{minimad::TextTemplate, minimad::TextTemplateExpander, MadSkin};

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn simple_table(attrs: &[(&'static str, String)]) {
    let table_template = TextTemplate::from(
        r#"
|-:|-:|
${feature-rows
|**${attr-name}**|${attr-avail}|
}
|-|-|
    "#,
    );

    fn make_table_display<'a, 'b, D: Display>(
        text_template: &'a TextTemplate<'b>,
        attrs: &[(&'b str, D)],
    ) -> TextTemplateExpander<'a, 'b> {
        let mut expander = text_template.expander();

        for (attr, desc) in attrs {
            let sdesc = string_to_static_str(format!("{}", desc));
            expander
                .sub("feature-rows")
                .set("attr-name", attr)
                .set("attr-avail", sdesc);
        }

        expander
    }

    let table = make_table_display(&table_template, &attrs);
    let skin = MadSkin::default();
    skin.print_expander(table);
}

fn table3(attrs: &[(&'static str, &'static str, String)]) {
    let table_template3 = TextTemplate::from(
        r#"
|:-|-:|-:|
${feature-rows
|**${category-name}**|**${attr-name}**|${attr-avail}|
}
|-|-|
    "#,
    );

    fn make_table_display3<'a, 'b, D: Display>(
        text_template: &'a TextTemplate<'b>,
        attrs: &[(&'b str, &'b str, D)],
    ) -> TextTemplateExpander<'a, 'b> {
        let mut expander = text_template.expander();

        for (cat, attr, desc) in attrs {
            let sdesc = string_to_static_str(format!("{}", desc));
            expander
                .sub("feature-rows")
                .set("category-name", cat)
                .set("attr-name", attr)
                .set("attr-avail", sdesc);
        }

        expander
    }

    let table = make_table_display3(&table_template3, &attrs);
    let skin = MadSkin::default();
    skin.print_expander(table);
}

fn print_title_line(title: &str, attr: Option<&str>) {
    let skin = MadSkin::default();
    if let Some(opt) = attr {
        skin.print_text(format!("## {} = \"{}\"\n", title, opt).as_str());
    } else {
        skin.print_text(format!("## {}\n", title).as_str());
    }
}

fn print_title_attr(title: &str, attr: &str) {
    print_title_line(title, Some(attr));
}

fn print_title(title: &str) {
    print_title_line(title, None)
}

fn print_subtitle(title: &str) {
    let skin = MadSkin::default();
    skin.print_text(format!("### {}\n", title).as_str());
}

fn print_attr<T: Display, A: Display>(name: T, attr: A) {
    let skin = MadSkin::default();
    skin.print_text(format!("{} = {}", name, attr).as_str());
}

fn print_cpuid_result<T: Display>(name: T, attr: CpuIdResult) {
    let skin = MadSkin::default();
    skin.print_text(
        format!(
            "{}: eax = {:#x} ebx = {:#x} ecx = {:#x} edx = {:#x}",
            name, attr.eax, attr.ebx, attr.ecx, attr.edx,
        )
        .as_str(),
    );
}

fn bool_repr(x: bool) -> String {
    if x {
        "✅".to_string()
    } else {
        "❌".to_string()
    }
}

trait RowGen {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String);
}

impl RowGen for bool {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = bool_repr(attr);
        (t, s)
    }
}

impl RowGen for u64 {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for usize {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for u32 {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for u16 {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for u8 {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for String {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        (t, attr)
    }
}

impl RowGen for Associativity {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for CacheType {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for TopologyType {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for ExtendedRegisterStateLocation {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for DatType {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

impl RowGen for Option<SoCVendorBrand> {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!(
            "{}",
            attr.map(|v| v.as_str().to_string())
                .unwrap_or(String::from(""))
        );

        (t, s)
    }
}

fn main() {
    let cpuid = CpuId::new();
    let skin = MadSkin::default();

    skin.print_text("# CpuId\n");

    if let Some(info) = cpuid.get_vendor_info() {
        print_title_attr("vendor_id (0x00)", info.as_str());
    }

    if let Some(info) = cpuid.get_feature_info() {
        print_title("version information (1/eax):");
        simple_table(&[
            RowGen::make_row("base family", info.base_family_id()),
            RowGen::make_row("base model", info.base_model_id()),
            RowGen::make_row("stepping", info.stepping_id()),
            RowGen::make_row("extended family", info.extended_family_id()),
            RowGen::make_row("extended model", info.extended_model_id()),
            RowGen::make_row("family", info.family_id()),
            RowGen::make_row("model", info.model_id()),
        ]);

        print_title("miscellaneous (1/ebx):");
        simple_table(&[
            RowGen::make_row("processor APIC physical id", info.initial_local_apic_id()),
            RowGen::make_row("max. cpus", info.max_logical_processor_ids()),
            RowGen::make_row("CLFLUSH line size", info.cflush_cache_line_size()),
            RowGen::make_row("brand index", info.brand_index()),
        ]);

        print_title("feature information (1/edx):");
        simple_table(&[
            RowGen::make_row("fpu", info.has_fpu()),
            RowGen::make_row("vme", info.has_vme()),
            RowGen::make_row("de", info.has_de()),
            RowGen::make_row("pse", info.has_pse()),
            RowGen::make_row("tsc", info.has_tsc()),
            RowGen::make_row("msr", info.has_msr()),
            RowGen::make_row("pae", info.has_pae()),
            RowGen::make_row("mce", info.has_mce()),
            RowGen::make_row("cmpxchg8b", info.has_cmpxchg8b()),
            RowGen::make_row("apic", info.has_apic()),
            RowGen::make_row("sysenter_sysexit", info.has_sysenter_sysexit()),
            RowGen::make_row("mtrr", info.has_mtrr()),
            RowGen::make_row("pge", info.has_pge()),
            RowGen::make_row("mca", info.has_mca()),
            RowGen::make_row("cmov", info.has_cmov()),
            RowGen::make_row("pat", info.has_pat()),
            RowGen::make_row("pse36", info.has_pse36()),
            RowGen::make_row("psn", info.has_psn()),
            RowGen::make_row("clflush", info.has_clflush()),
            RowGen::make_row("ds", info.has_ds()),
            RowGen::make_row("acpi", info.has_acpi()),
            RowGen::make_row("mmx", info.has_mmx()),
            RowGen::make_row("fxsave_fxstor", info.has_fxsave_fxstor()),
            RowGen::make_row("sse", info.has_sse()),
            RowGen::make_row("sse2", info.has_sse2()),
            RowGen::make_row("ss", info.has_ss()),
            RowGen::make_row("htt", info.has_htt()),
            RowGen::make_row("tm", info.has_tm()),
            RowGen::make_row("pbe", info.has_pbe()),
        ]);

        print_title("feature information (1/ecx):");
        simple_table(&[
            RowGen::make_row("sse3", info.has_sse3()),
            RowGen::make_row("pclmulqdq", info.has_pclmulqdq()),
            RowGen::make_row("ds_area", info.has_ds_area()),
            RowGen::make_row("monitor_mwait", info.has_monitor_mwait()),
            RowGen::make_row("cpl", info.has_cpl()),
            RowGen::make_row("vmx", info.has_vmx()),
            RowGen::make_row("smx", info.has_smx()),
            RowGen::make_row("eist", info.has_eist()),
            RowGen::make_row("tm2", info.has_tm2()),
            RowGen::make_row("ssse3", info.has_ssse3()),
            RowGen::make_row("cnxtid", info.has_cnxtid()),
            RowGen::make_row("fma", info.has_fma()),
            RowGen::make_row("cmpxchg16b", info.has_cmpxchg16b()),
            RowGen::make_row("pdcm", info.has_pdcm()),
            RowGen::make_row("pcid", info.has_pcid()),
            RowGen::make_row("dca", info.has_dca()),
            RowGen::make_row("sse41", info.has_sse41()),
            RowGen::make_row("sse42", info.has_sse42()),
            RowGen::make_row("x2apic", info.has_x2apic()),
            RowGen::make_row("movbe", info.has_movbe()),
            RowGen::make_row("popcnt", info.has_popcnt()),
            RowGen::make_row("tsc_deadline", info.has_tsc_deadline()),
            RowGen::make_row("aesni", info.has_aesni()),
            RowGen::make_row("xsave", info.has_xsave()),
            RowGen::make_row("oxsave", info.has_oxsave()),
            RowGen::make_row("avx", info.has_avx()),
            RowGen::make_row("f16c", info.has_f16c()),
            RowGen::make_row("rdrand", info.has_rdrand()),
            RowGen::make_row("hypervisor", info.has_hypervisor()),
        ]);
    }

    if let Some(info) = cpuid.get_cache_info() {
        println!("Cache");
        println!("{:?}", info);
    }

    if let Some(info) = cpuid.get_processor_serial() {
        print_title_attr(
            "processor serial number (0x03)",
            format!(
                "{:0>8x}-{:0>8x}-{:0>8x}",
                info.serial_upper(),
                info.serial_middle(),
                info.serial_lower()
            )
            .as_str(),
        );
    }

    if let Some(iter) = cpuid.get_cache_parameters() {
        print_title("deterministic cache parameters (0x04):");
        for cache in iter {
            print_subtitle(format!("L{} Cache:", cache.level()).as_str());

            let size = (cache.associativity()
                * cache.physical_line_partitions()
                * cache.coherency_line_size()
                * cache.sets()) as u64;

            simple_table(&[
                RowGen::make_row("cache type", cache.cache_type()),
                RowGen::make_row("cache level", cache.level()),
                RowGen::make_row(
                    "self-initializing cache level",
                    cache.is_self_initializing(),
                ),
                RowGen::make_row("fully associative cache", cache.is_fully_associative()),
                RowGen::make_row("threads sharing this cache", cache.max_cores_for_cache()),
                RowGen::make_row("processor cores on this die", cache.max_cores_for_package()),
                RowGen::make_row("system coherency line size", cache.coherency_line_size()),
                RowGen::make_row("physical line partitions", cache.physical_line_partitions()),
                RowGen::make_row("ways of associativity", cache.associativity()),
                RowGen::make_row(
                    "WBINVD/INVD acts on lower caches",
                    cache.is_write_back_invalidate(),
                ),
                RowGen::make_row("inclusive to lower caches", cache.is_inclusive()),
                RowGen::make_row("complex cache indexing", cache.has_complex_indexing()),
                RowGen::make_row("number of sets", cache.sets()),
                RowGen::make_row("(size synth.)", size),
            ]);
        }
    }

    if let Some(info) = cpuid.get_monitor_mwait_info() {
        print_title("MONITOR/MWAIT (0x05):");
        simple_table(&[
            RowGen::make_row("smallest monitor-line size", info.smallest_monitor_line()),
            RowGen::make_row("largest monitor-line size", info.largest_monitor_line()),
            RowGen::make_row("MONITOR/MWAIT exts", info.extensions_supported()),
            RowGen::make_row(
                "Interrupts as break-event for MWAIT",
                info.interrupts_as_break_event(),
            ),
        ]);

        skin.print_text("number of CX sub C-states using MWAIT:\n");
        let cstate_table = TextTemplate::from(
            r#"
        | :-: |  :-: | :-: | :-: | :-: | :-: | :-: | :-: |
        |**C0**|**C1**|**C2**|**C3**|**C4**|**C5**|**C6**|**C7**|
        | :-: |  :-: | :-: | :-: | :-: | :-: | :-: | :-: |
        |${c0}|${c1}|${c2}|${c3}|${c4}|${c5}|${c6}|${c7}|
        | :-: |  :-: | :-: | :-: | :-: | :-: | :-: | :-: |
        "#,
        );
        let c0 = format!("{}", info.supported_c0_states());
        let c1 = format!("{}", info.supported_c1_states());
        let c2 = format!("{}", info.supported_c2_states());
        let c3 = format!("{}", info.supported_c3_states());
        let c4 = format!("{}", info.supported_c4_states());
        let c5 = format!("{}", info.supported_c5_states());
        let c6 = format!("{}", info.supported_c6_states());
        let c7 = format!("{}", info.supported_c7_states());

        let mut ctbl = cstate_table.expander();
        ctbl.set("c0", c0.as_str());
        ctbl.set("c1", c1.as_str());
        ctbl.set("c2", c2.as_str());
        ctbl.set("c3", c3.as_str());
        ctbl.set("c4", c4.as_str());
        ctbl.set("c5", c5.as_str());
        ctbl.set("c6", c6.as_str());
        ctbl.set("c7", c7.as_str());
        skin.print_expander(ctbl);
    }

    if let Some(info) = cpuid.get_thermal_power_info() {
        print_title("Thermal and Power Management Features (0x06):");
        simple_table(&[
            RowGen::make_row("digital thermometer", info.has_dts()),
            RowGen::make_row("Intel Turbo Boost Technology", info.has_turbo_boost()),
            RowGen::make_row("ARAT always running APIC timer", info.has_arat()),
            RowGen::make_row("PLN power limit notification", info.has_pln()),
            RowGen::make_row("ECMD extended clock modulation duty", info.has_ecmd()),
            RowGen::make_row("PTM package thermal management", info.has_ptm()),
            RowGen::make_row("HWP base registers", info.has_hwp()),
            RowGen::make_row("HWP notification", info.has_hwp_notification()),
            RowGen::make_row("HWP activity window", info.has_hwp_activity_window()),
            RowGen::make_row(
                "HWP energy performance preference",
                info.has_hwp_energy_performance_preference(),
            ),
            RowGen::make_row(
                "HWP package level request",
                info.has_hwp_package_level_request(),
            ),
            RowGen::make_row("HDC base registers", info.has_hdc()),
            RowGen::make_row(
                "Intel Turbo Boost Max Technology 3.0",
                info.has_turbo_boost3(),
            ),
            RowGen::make_row("HWP capabilities", info.has_hwp_capabilities()),
            RowGen::make_row("HWP PECI override", info.has_hwp_peci_override()),
            RowGen::make_row("flexible HWP", info.has_flexible_hwp()),
            RowGen::make_row(
                "IA32_HWP_REQUEST MSR fast access mode",
                info.has_hwp_fast_access_mode(),
            ),
            RowGen::make_row(
                "ignoring idle logical processor HWP req",
                info.has_ignore_idle_processor_hwp_request(),
            ),
            RowGen::make_row("digital thermometer threshold", info.dts_irq_threshold()),
            RowGen::make_row(
                "hardware coordination feedback",
                info.has_hw_coord_feedback(),
            ),
            RowGen::make_row(
                "performance-energy bias capability",
                info.has_energy_bias_pref(),
            ),
        ]);
    }

    if let Some(info) = cpuid.get_extended_feature_info() {
        print_title("Extended feature flags (0x07):");

        simple_table(&[
            RowGen::make_row("FSGSBASE", info.has_fsgsbase()),
            RowGen::make_row("IA32_TSC_ADJUST MSR", info.has_tsc_adjust_msr()),
            RowGen::make_row("SGX: Software Guard Extensions", info.has_sgx()),
            RowGen::make_row("BMI1", info.has_bmi1()),
            RowGen::make_row("HLE hardware lock elision", info.has_hle()),
            RowGen::make_row("AVX2: advanced vector extensions 2", info.has_avx2()),
            RowGen::make_row("FDP_EXCPTN_ONLY", info.has_fdp()),
            RowGen::make_row("SMEP supervisor mode exec protection", info.has_smep()),
            RowGen::make_row("BMI2 instructions", info.has_bmi2()),
            RowGen::make_row("enhanced REP MOVSB/STOSB", info.has_rep_movsb_stosb()),
            RowGen::make_row("INVPCID instruction", info.has_invpcid()),
            RowGen::make_row("RTM: restricted transactional memory", info.has_rtm()),
            RowGen::make_row("RDT-CMT/PQoS cache monitoring", info.has_rdtm()),
            RowGen::make_row("deprecated FPU CS/DS", info.has_fpu_cs_ds_deprecated()),
            RowGen::make_row("MPX: intel memory protection extensions", info.has_mpx()),
            RowGen::make_row("RDT-CAT/PQE cache allocation", info.has_rdta()),
            RowGen::make_row(
                "AVX512F: AVX-512 foundation instructions",
                info.has_avx512f(),
            ),
            RowGen::make_row(
                "AVX512DQ: double & quadword instructions",
                info.has_avx512dq(),
            ),
            RowGen::make_row("RDSEED instruction", info.has_rdseed()),
            RowGen::make_row("ADX instructions", info.has_adx()),
            RowGen::make_row("SMAP: supervisor mode access prevention", info.has_smap()),
            RowGen::make_row("AVX512IFMA: fused multiply add", info.has_avx512_ifma()),
            RowGen::make_row("CLFLUSHOPT instruction", info.has_clflushopt()),
            RowGen::make_row("CLWB instruction", info.has_clwb()),
            RowGen::make_row("Intel processor trace", info.has_processor_trace()),
            RowGen::make_row("AVX512PF: prefetch instructions", info.has_avx512pf()),
            RowGen::make_row(
                "AVX512ER: exponent & reciprocal instrs",
                info.has_avx512er(),
            ),
            RowGen::make_row("AVX512CD: conflict detection instrs", info.has_avx512cd()),
            RowGen::make_row("SHA instructions", info.has_sha()),
            RowGen::make_row("AVX512BW: byte & word instructions", info.has_avx512bw()),
            RowGen::make_row("AVX512VL: vector length", info.has_avx512vl()),
            RowGen::make_row("PREFETCHWT1", info.has_prefetchwt1()),
            RowGen::make_row("UMIP: user-mode instruction prevention", info.has_umip()),
            RowGen::make_row("PKU protection keys for user-mode", info.has_pku()),
            RowGen::make_row("OSPKE CR4.PKE and RDPKRU/WRPKRU", info.has_ospke()),
            RowGen::make_row(
                "BNDLDX/BNDSTX MAWAU value in 64-bit mode",
                info.mawau_value(),
            ),
            RowGen::make_row("RDPID: read processor ID", info.has_rdpid()),
            RowGen::make_row("SGX_LC: SGX launch config", info.has_sgx_lc()),
        ]);
    }

    if let Some(info) = cpuid.get_direct_cache_access_info() {
        print_title("Direct Cache Access Parameters (0x09):");
        print_attr("PLATFORM_DCA_CAP MSR bits", info.get_dca_cap_value());
    }

    if let Some(info) = cpuid.get_performance_monitoring_info() {
        print_title("Architecture Performance Monitoring Features (0x0a)");

        print_subtitle("Monitoring Hardware Info (0x0a/{eax, edx}):");
        simple_table(&[
            RowGen::make_row("version ID", info.version_id()),
            RowGen::make_row(
                "number of counters per HW thread",
                info.number_of_counters(),
            ),
            RowGen::make_row("bit width of counter", info.counter_bit_width()),
            RowGen::make_row("length of EBX bit vector", info.ebx_length()),
            RowGen::make_row("number of fixed counters", info.fixed_function_counters()),
            RowGen::make_row(
                "bit width of fixed counters",
                info.fixed_function_counters_bit_width(),
            ),
            RowGen::make_row("anythread deprecation", info.has_any_thread_deprecation()),
        ]);

        print_subtitle("Monitoring Hardware Features (0x0a/ebx):");
        simple_table(&[
            RowGen::make_row(
                "core cycle event not available",
                info.is_core_cyc_ev_unavailable(),
            ),
            RowGen::make_row(
                "instruction retired event not available",
                info.is_inst_ret_ev_unavailable(),
            ),
            RowGen::make_row(
                "reference cycles event not available",
                info.is_ref_cycle_ev_unavailable(),
            ),
            RowGen::make_row(
                "last-level cache ref event not available",
                info.is_cache_ref_ev_unavailable(),
            ),
            RowGen::make_row(
                "last-level cache miss event not avail",
                info.is_ll_cache_miss_ev_unavailable(),
            ),
            RowGen::make_row(
                "branch inst retired event not available",
                info.is_branch_inst_ret_ev_unavailable(),
            ),
            RowGen::make_row(
                "branch mispred retired event not available",
                info.is_branch_midpred_ev_unavailable(),
            ),
        ]);
    }

    if let Some(info) = cpuid.get_extended_topology_info() {
        print_title("x2APIC features / processor topology (0x0b):");

        for level in info {
            print_subtitle(format!("level {}:", level.level_number()).as_str());
            simple_table(&[
                RowGen::make_row("level type", level.level_type()),
                RowGen::make_row("bit width of level", level.shift_right_for_next_apic_id()),
                RowGen::make_row("number of logical processors at level", level.processors()),
                RowGen::make_row("x2apic id of current processor", level.x2apic_id()),
            ]);
        }
    }

    if let Some(info) = cpuid.get_extended_state_info() {
        print_title("Extended Register State (0x0d/0):");

        print_subtitle("XCR0/IA32_XSS supported states:");
        table3(&[
            ("XCR0", "x87", bool_repr(info.xcr0_supports_legacy_x87())),
            ("XCR0", "SSE state", bool_repr(info.xcr0_supports_sse_128())),
            ("XCR0", "AVX state", bool_repr(info.xcr0_supports_avx_256())),
            (
                "XCR0",
                "MPX BNDREGS",
                bool_repr(info.xcr0_supports_mpx_bndregs()),
            ),
            (
                "XCR0",
                "MPX BNDCSR",
                bool_repr(info.xcr0_supports_mpx_bndcsr()),
            ),
            (
                "XCR0",
                "AVX-512 opmask",
                bool_repr(info.xcr0_supports_avx512_opmask()),
            ),
            (
                "XCR0",
                "AVX-512 ZMM_Hi256",
                bool_repr(info.xcr0_supports_avx512_zmm_hi256()),
            ),
            (
                "XCR0",
                "AVX-512 Hi16_ZMM",
                bool_repr(info.xcr0_supports_avx512_zmm_hi16()),
            ),
            ("IA32_XSS", "PT", bool_repr(info.ia32_xss_supports_pt())),
            ("XCR0", "PKRU", bool_repr(info.xcr0_supports_pkru())),
            //("XCR0", "CET_U state", xxx),
            //("XCR0", "CET_S state", xxx),
            ("IA32_XSS", "HDC", bool_repr(info.ia32_xss_supports_hdc())),
        ]);

        simple_table(&[
            RowGen::make_row(
                "bytes required by fields in XCR0",
                info.xsave_area_size_enabled_features(),
            ),
            RowGen::make_row(
                "bytes required by XSAVE/XRSTOR area",
                info.xsave_area_size_supported_features(),
            ),
        ]);

        print_subtitle("XSAVE features (0x0d/1):");
        simple_table(&[
            RowGen::make_row("XSAVEOPT instruction", info.has_xsaveopt()),
            RowGen::make_row("XSAVEC instruction", info.has_xsavec()),
            RowGen::make_row("XGETBV instruction", info.has_xgetbv()),
            RowGen::make_row("XSAVES/XRSTORS instructions", info.has_xsaves_xrstors()),
            RowGen::make_row("SAVE area size [Bytes]", info.xsave_size()),
        ]);

        for state in info.iter() {
            print_subtitle(
                format!("{} features (0x0d/{}):", state.register(), state.subleaf).as_str(),
            );
            simple_table(&[
                RowGen::make_row("save state size [Bytes]", state.size()),
                RowGen::make_row("save state byte offset", state.offset()),
                RowGen::make_row("supported in IA32_XSS or XCR0", state.location()),
                RowGen::make_row(
                    "64-byte alignment in compacted XSAVE",
                    state.is_compacted_format(),
                ),
            ]);
        }
    }

    if let Some(info) = cpuid.get_rdt_monitoring_info() {
        print_title("Quality of Service Monitoring Resource Type (0x0f/0):");
        simple_table(&[
            RowGen::make_row("Maximum range of RMID", info.rmid_range()),
            RowGen::make_row("L3 cache QoS monitoring", info.has_l3_monitoring()),
        ]);

        if let Some(rmid) = info.l3_monitoring() {
            print_subtitle("L3 Cache Quality of Service Monitoring (0x0f/1):");

            simple_table(&[
                RowGen::make_row(
                    "Conversion factor from IA32_QM_CTR to bytes",
                    rmid.conversion_factor(),
                ),
                RowGen::make_row("Maximum range of RMID", rmid.maximum_rmid_range()),
                RowGen::make_row("L3 occupancy monitoring", rmid.has_occupancy_monitoring()),
                RowGen::make_row(
                    "L3 total bandwidth monitoring",
                    rmid.has_total_bandwidth_monitoring(),
                ),
                RowGen::make_row(
                    "L3 local bandwidth monitoring",
                    rmid.has_local_bandwidth_monitoring(),
                ),
            ]);
        }
    }

    if let Some(info) = cpuid.get_rdt_allocation_info() {
        print_title("Resource Director Technology Allocation (0x10/0)");
        simple_table(&[
            RowGen::make_row("L3 cache allocation technology", info.has_l3_cat()),
            RowGen::make_row("L2 cache allocation technology", info.has_l2_cat()),
            RowGen::make_row(
                "memory bandwidth allocation",
                info.has_memory_bandwidth_allocation(),
            ),
        ]);

        if let Some(l3_cat) = info.l3_cat() {
            print_subtitle("L3 Cache Allocation Technology (0x10/1):");
            simple_table(&[
                RowGen::make_row("length of capacity bit mask", l3_cat.capacity_mask_length()),
                RowGen::make_row(
                    "Bit-granular map of isolation/contention",
                    l3_cat.isolation_bitmap(),
                ),
                RowGen::make_row(
                    "code and data prioritization",
                    l3_cat.has_code_data_prioritization(),
                ),
                RowGen::make_row("highest COS number", l3_cat.highest_cos()),
            ]);
        }
        if let Some(l2_cat) = info.l2_cat() {
            print_subtitle("L2 Cache Allocation Technology (0x10/2):");
            simple_table(&[
                RowGen::make_row("length of capacity bit mask", l2_cat.capacity_mask_length()),
                RowGen::make_row(
                    "Bit-granular map of isolation/contention",
                    l2_cat.isolation_bitmap(),
                ),
                RowGen::make_row("highest COS number", l2_cat.highest_cos()),
            ]);
        }
        if let Some(mem) = info.memory_bandwidth_allocation() {
            print_subtitle("Memory Bandwidth Allocation (0x10/3):");
            simple_table(&[
                RowGen::make_row("maximum throttling value", mem.max_hba_throttling()),
                RowGen::make_row("delay values are linear", mem.has_linear_response_delay()),
                RowGen::make_row("highest COS number", mem.highest_cos()),
            ]);
        }
    }

    if let Some(info) = cpuid.get_sgx_info() {
        print_title("SGX - Software Guard Extensions (0x12/{0,1}):");

        simple_table(&[
            RowGen::make_row("SGX1", info.has_sgx1()),
            RowGen::make_row("SGX2", info.has_sgx2()),
            RowGen::make_row(
                "SGX ENCLV E*VIRTCHILD, ESETCONTEXT",
                info.has_enclv_leaves_einvirtchild_edecvirtchild_esetcontext(),
            ),
            RowGen::make_row(
                "SGX ENCLS ETRACKC, ERDINFO, ELDBC, ELDUC",
                info.has_encls_leaves_etrackc_erdinfo_eldbc_elduc(),
            ),
            RowGen::make_row("MISCSELECT", info.miscselect()),
            RowGen::make_row(
                "MaxEnclaveSize_Not64 (log2)",
                info.max_enclave_size_non_64bit(),
            ),
            RowGen::make_row("MaxEnclaveSize_64 (log2)", info.max_enclave_size_64bit()),
        ]);

        for (idx, leaf) in info.iter().enumerate() {
            let SgxSectionInfo::Epc(section) = leaf;
            print_subtitle(format!("Enclave Page Cache (0x12/{})", idx + 2).as_str());
            simple_table(&[
                RowGen::make_row("physical base address", section.physical_base()),
                RowGen::make_row("size", section.size()),
            ]);
        }
    }

    if let Some(info) = cpuid.get_processor_trace_info() {
        print_title("Intel Processor Trace (0x14):");
        simple_table(&[
            RowGen::make_row(
                "IA32_RTIT_CR3_MATCH is accessible",
                info.has_rtit_cr3_match(),
            ),
            RowGen::make_row(
                "configurable PSB & cycle-accurate",
                info.has_configurable_psb_and_cycle_accurate_mode(),
            ),
            RowGen::make_row(
                "IP & TraceStop filtering; PT preserve",
                info.has_ip_tracestop_filtering(),
            ),
            RowGen::make_row(
                "MTC timing packet; suppress COFI-based",
                info.has_mtc_timing_packet_coefi_suppression(),
            ),
            RowGen::make_row("PTWRITE", info.has_ptwrite()),
            RowGen::make_row("power event trace", info.has_power_event_trace()),
            RowGen::make_row("ToPA output scheme", info.has_topa()),
            RowGen::make_row(
                "ToPA can hold many output entries",
                info.has_topa_maximum_entries(),
            ),
            RowGen::make_row(
                "single-range output scheme support",
                info.has_single_range_output_scheme(),
            ),
            RowGen::make_row(
                "output to trace transport",
                info.has_trace_transport_subsystem(),
            ),
            RowGen::make_row(
                "IP payloads have LIP values & CS",
                info.has_lip_with_cs_base(),
            ),
            RowGen::make_row(
                "configurable address ranges",
                info.configurable_address_ranges(),
            ),
            RowGen::make_row(
                "supported MTC periods bitmask",
                info.supported_mtc_period_encodings(),
            ),
            RowGen::make_row(
                "supported cycle threshold bitmask",
                info.supported_cycle_threshold_value_encodings(),
            ),
            RowGen::make_row(
                "supported config PSB freq bitmask",
                info.supported_psb_frequency_encodings(),
            ),
        ]);
    }

    if let Some(info) = cpuid.get_tsc_info() {
        print_title("Time Stamp Counter/Core Crystal Clock Information (0x15):");
        simple_table(&[
            RowGen::make_row(
                "TSC/clock ratio",
                format!("{} / {}", info.numerator(), info.denominator()),
            ),
            RowGen::make_row("nominal core crystal clock", info.nominal_frequency()),
        ]);
    }

    if let Some(info) = cpuid.get_processor_frequency_info() {
        print_title("Processor Frequency Information (0x16):");
        simple_table(&[
            RowGen::make_row("Core Base Frequency (MHz)", info.processor_base_frequency()),
            RowGen::make_row(
                "Core Maximum Frequency (MHz)",
                info.processor_max_frequency(),
            ),
            RowGen::make_row("Bus (Reference) Frequency (MHz)", info.bus_frequency()),
        ]);
    }

    if let Some(dat_iter) = cpuid.get_deterministic_address_translation_info() {
        for (idx, info) in dat_iter.enumerate() {
            print_title(
                format!(
                    "Deterministic Address Translation Structure (0x18/{}):",
                    idx
                )
                .as_str(),
            );
            simple_table(&[
                RowGen::make_row("number of sets", info.sets()),
                RowGen::make_row("4 KiB page size entries", info.has_4k_entries()),
                RowGen::make_row("2 MiB page size entries", info.has_2mb_entries()),
                RowGen::make_row("4 MiB page size entries", info.has_4mb_entries()),
                RowGen::make_row("1 GiB page size entries", info.has_1gb_entries()),
                RowGen::make_row("partitioning", info.partitioning()),
                RowGen::make_row("ways of associativity", info.ways()),
                RowGen::make_row("translation cache type", info.cache_type()),
                RowGen::make_row("translation cache level", info.cache_level()),
                RowGen::make_row("fully associative", info.is_fully_associative()),
                RowGen::make_row(
                    "maximum number of addressible IDs",
                    info.max_addressable_ids(),
                ),
                RowGen::make_row(
                    "maximum number of addressible IDs",
                    info.max_addressable_ids(),
                ),
            ]);
        }
    }

    if let Some(info) = cpuid.get_soc_vendor_info() {
        print_title("System-on-Chip (SoC) Vendor Info (0x17):");
        simple_table(&[
            RowGen::make_row("Vendor ID", info.get_soc_vendor_id()),
            RowGen::make_row("Project ID", info.get_project_id()),
            RowGen::make_row("Stepping ID", info.get_stepping_id()),
            RowGen::make_row("Vendor Brand", info.get_vendor_brand()),
        ]);

        if let Some(iter) = info.get_vendor_attributes() {
            for (idx, attr) in iter.enumerate() {
                print_cpuid_result(format!("0x17 {:#x}", idx + 4), attr);
            }
        }
    }

    if let Some(info) = cpuid.get_processor_brand_string() {
        print_attr(
            "Processor Brand String",
            format!("\"**{}**\"", info.as_str()),
        );
    }

    if let Some(info) = cpuid.get_l1_cache_and_tlb_info() {
        print_title("L1 TLB 2/4 MiB entries (0x8000_0005/eax):");
        simple_table(&[
            RowGen::make_row("iTLB #entries", info.dtlb_2m_4m_size()),
            RowGen::make_row("iTLB associativity", info.itlb_2m_4m_associativity()),
            RowGen::make_row("dTLB #entries", info.itlb_2m_4m_size()),
            RowGen::make_row("dTLB associativity", info.dtlb_2m_4m_associativity()),
        ]);

        print_title("L1 TLB 4 KiB entries (0x8000_0005/ebx):");
        simple_table(&[
            RowGen::make_row("iTLB #entries", info.itlb_4k_size()),
            RowGen::make_row("iTLB associativity", info.itlb_4k_associativity()),
            RowGen::make_row("dTLB #entries", info.dtlb_4k_size()),
            RowGen::make_row("dTLB associativity", info.dtlb_4k_associativity()),
        ]);

        print_title("L1 dCache (0x8000_0005/ecx):");
        simple_table(&[
            RowGen::make_row("line size [Bytes]", info.dcache_line_size()),
            RowGen::make_row("lines per tag", info.dcache_lines_per_tag()),
            RowGen::make_row("associativity", info.dcache_associativity()),
            RowGen::make_row("size [KiB]", info.dcache_size()),
        ]);

        print_title("L1 iCache (0x8000_0005/edx):");
        simple_table(&[
            RowGen::make_row("line size [Bytes]", info.icache_line_size()),
            RowGen::make_row("lines per tag", info.icache_lines_per_tag()),
            RowGen::make_row("associativity", info.icache_associativity()),
            RowGen::make_row("size [KiB]", info.icache_size()),
        ]);
    }

    if let Some(info) = cpuid.get_l2_l3_cache_and_tlb_info() {
        print_title("L2 TLB 2/4 MiB entries (0x8000_0006/eax):");
        simple_table(&[
            RowGen::make_row("iTLB #entries", info.dtlb_2m_4m_size()),
            RowGen::make_row("iTLB associativity", info.itlb_2m_4m_associativity()),
            RowGen::make_row("dTLB #entries", info.itlb_2m_4m_size()),
            RowGen::make_row("dTLB associativity", info.dtlb_2m_4m_associativity()),
        ]);

        print_title("L2 TLB 4 KiB entries (0x8000_0006/ebx):");
        simple_table(&[
            RowGen::make_row("iTLB #entries", info.itlb_4k_size()),
            RowGen::make_row("iTLB associativity", info.itlb_4k_associativity()),
            RowGen::make_row("dTLB #entries", info.dtlb_4k_size()),
            RowGen::make_row("dTLB associativity", info.dtlb_4k_associativity()),
        ]);

        print_title("L2 Cache (0x8000_0006/ecx):");
        simple_table(&[
            RowGen::make_row("line size [Bytes]", info.l2cache_line_size()),
            RowGen::make_row("lines per tag", info.l2cache_lines_per_tag()),
            RowGen::make_row("associativity", info.l2cache_associativity()),
            RowGen::make_row("size [KiB]", info.l2cache_size()),
        ]);

        print_title("L3 Cache (0x8000_0006/edx):");
        simple_table(&[
            RowGen::make_row("line size [Bytes]", info.l3cache_line_size()),
            RowGen::make_row("lines per tag", info.l3cache_lines_per_tag()),
            RowGen::make_row("associativity", info.l3cache_associativity()),
            RowGen::make_row("size [KiB]", info.l3cache_size() * 512),
        ]);
    }

    if let Some(info) = cpuid.get_advanced_power_mgmt_info() {
        print_title("RAS Capability (0x8000_0007/ebx):");
        simple_table(&[
            RowGen::make_row("MCA overflow recovery", info.has_mca_overflow_recovery()),
            RowGen::make_row("SUCCOR", info.has_succor()),
            RowGen::make_row("HWA: hardware assert", info.has_hwa()),
        ]);

        print_title("Advanced Power Management (0x8000_0007/ecx):");
        print_attr(
            "Ratio of Compute Unit Power Acc. sample period to TSC",
            info.cpu_pwr_sample_time_ratio(),
        );

        print_title("Advanced Power Management (0x8000_0007/edx):");
        simple_table(&[
            RowGen::make_row("TS: temperature sensing diode", info.has_ts()),
            RowGen::make_row("FID: frequency ID control", info.has_freq_id_ctrl()),
            RowGen::make_row("VID: voltage ID control", info.has_volt_id_ctrl()),
            RowGen::make_row("TTP: thermal trip", info.has_thermtrip()),
            RowGen::make_row("TM: thermal monitor", info.has_tm()),
            RowGen::make_row("100 MHz multiplier control", info.has_100mhz_steps()),
            RowGen::make_row("hardware P-State control", info.has_hw_pstate()),
            RowGen::make_row("Invariant TSC", info.has_invariant_tsc()),
            RowGen::make_row("CPB: core performance boost", info.has_cpb()),
            RowGen::make_row(
                "read-only effective frequency interface",
                info.has_ro_effective_freq_iface(),
            ),
            RowGen::make_row("processor feedback interface", info.has_feedback_iface()),
            RowGen::make_row("APM power reporting", info.has_power_reporting_iface()),
        ]);
    }

    if let Some(info) = cpuid.get_processor_capacity_feature_info() {
        print_title("Physical Address and Linear Address Size (0x8000_0008/eax):");
        simple_table(&[
            RowGen::make_row(
                "maximum physical address [Bits]",
                info.physical_address_bits(),
            ),
            RowGen::make_row(
                "maximum linear (virtual) address [Bits]",
                info.linear_address_bits(),
            ),
            RowGen::make_row(
                "maximum guest physical address [Bits]",
                info.guest_physical_address_bits(),
            ),
        ]);

        print_title("Extended Feature Extensions ID (0x8000_0008/ebx):");
        simple_table(&[
            RowGen::make_row("CLZERO", info.has_cl_zero()),
            RowGen::make_row("instructions retired count", info.has_inst_ret_cntr_msr()),
            RowGen::make_row(
                "always save/restore error pointers",
                info.has_restore_fp_error_ptrs(),
            ),
            RowGen::make_row("RDPRU", info.has_rdpru()),
            RowGen::make_row("INVLPGB", info.has_invlpgb()),
            RowGen::make_row("MCOMMIT", info.has_mcommit()),
            RowGen::make_row("WBNOINVD", info.has_wbnoinvd()),
            RowGen::make_row("WBNOINVD/WBINVD interruptible", info.has_int_wbinvd()),
            RowGen::make_row("EFER.LMSLE unsupported", info.has_unsupported_efer_lmsle()),
            RowGen::make_row("INVLPGB with nested paging", info.has_invlpgb_nested()),
        ]);

        print_title("Size Identifiers (0x8000_0008/ecx):");
        simple_table(&[
            RowGen::make_row("Logical processors", info.num_phys_threads()),
            RowGen::make_row("APIC core ID size", info.apic_id_size()),
            RowGen::make_row("Max. logical processors", info.maximum_logical_processors()),
            RowGen::make_row("Perf. TSC size [Bits]", info.perf_tsc_size()),
        ]);

        print_title("Size Identifiers (0x8000_0008/edx):");
        simple_table(&[
            RowGen::make_row("RDPRU max. input value", info.max_rdpru_id()),
            RowGen::make_row("INVLPGB max. #pages", info.invlpgb_max_pages()),
        ]);
    }

    if let Some(info) = cpuid.get_svm_info() {
        print_title("SVM Secure Virtual Machine (0x8000_000a/eax):");
        print_attr("Revision", info.revision());

        print_title("SVM Secure Virtual Machine (0x8000_000a/edx):");
        simple_table(&[
            RowGen::make_row("nested paging", info.has_nested_paging()),
            RowGen::make_row("LBR virtualization", info.has_lbr_virtualization()),
            RowGen::make_row("SVM lock", info.has_svm_lock()),
            RowGen::make_row("NRIP", info.has_nrip()),
            RowGen::make_row("MSR based TSC rate control", info.has_tsc_rate_msr()),
            RowGen::make_row("VMCB clean bits support", info.has_vmcb_clean_bits()),
            RowGen::make_row("flush by ASID", info.has_flush_by_asid()),
            RowGen::make_row("decode assists", info.has_decode_assists()),
            RowGen::make_row("pause intercept filter", info.has_pause_filter()),
            RowGen::make_row("pause filter threshold", info.has_pause_filter_threshold()),
            RowGen::make_row("AVIC: virtual interrupt controller", info.has_avic()),
            RowGen::make_row(
                "virtualized VMLOAD/VMSAVE",
                info.has_vmsave_virtualization(),
            ),
            RowGen::make_row("GIF: virtual global interrupt flag", info.has_gif()),
            RowGen::make_row("GMET: guest mode execute trap", info.has_gmet()),
            RowGen::make_row("SPEC_CTRL virtualization", info.has_spec_ctrl()),
            RowGen::make_row("Supervisor shadow-stack restrictions", info.has_sss_check()),
            RowGen::make_row("#MC intercept", info.has_host_mce_override()),
            RowGen::make_row("INVLPGB/TLBSYNC virtualization", info.has_tlb_ctrl()),
        ]);
    }

    if let Some(info) = cpuid.get_memory_encryption_info() {
        print_title("Memory Encryption Support (0x8000_001f):");
        simple_table(&[
            RowGen::make_row("SME: Secure Memory Encryption", info.has_sme()),
            RowGen::make_row("SEV: Secure Encrypted Virtualization", info.has_sev()),
            RowGen::make_row("Page Flush MSR", info.has_page_flush_msr()),
            RowGen::make_row("SEV-ES: Encrypted State", info.has_sev_es()),
            RowGen::make_row("SEV Secure Nested Paging", info.has_sev_snp()),
            RowGen::make_row("VM Permission Levels", info.has_vmpl()),
            RowGen::make_row(
                "Hardware cache coherency across encryption domains",
                info.has_hw_enforced_cache_coh(),
            ),
            RowGen::make_row("SEV guests only with 64-bit host", info.has_64bit_mode()),
            RowGen::make_row("Restricted injection", info.has_restricted_injection()),
            RowGen::make_row("Alternate injection", info.has_alternate_injection()),
            RowGen::make_row(
                "Full debug state swap for SEV-ES guests",
                info.has_debug_swap(),
            ),
            RowGen::make_row(
                "Disallowing IBS use by the host supported",
                info.has_prevent_host_ibs(),
            ),
            RowGen::make_row("Virtual Transparent Encryption", info.has_vte()),
            RowGen::make_row("C-bit position in page-table", info.c_bit_position()),
            RowGen::make_row(
                "Physical address bit reduction",
                info.physical_address_reduction(),
            ),
            RowGen::make_row(
                "Max. simultaneouslys encrypted guests",
                info.max_encrypted_guests(),
            ),
            RowGen::make_row(
                "Minimum ASID value for SEV guest",
                info.min_sev_no_es_asid(),
            ),
        ]);
    }
}

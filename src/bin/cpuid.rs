use std::fmt::Display;

use raw_cpuid::{CpuId, TopologyType};
use termimad::{minimad::TextTemplate, minimad::TextTemplateExpander, MadSkin};

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn make_table<'a, 'b>(
    text_template: &'a TextTemplate<'b>,
    attrs: &[(&'b str, u64)],
) -> TextTemplateExpander<'a, 'b> {
    let mut expander = text_template.expander();
    expander.set("app-version", "2");

    for &(attr, desc) in attrs {
        let sdesc = string_to_static_str(format!("{}", desc));
        expander
            .sub("feature-rows")
            .set("attr-name", attr)
            .set("attr-avail", sdesc);
    }

    expander
}

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

fn make_feature_table<'a, 'b>(
    text_template: &'a TextTemplate<'b>,
    attrs: &[(&'b str, bool)],
) -> TextTemplateExpander<'a, 'b> {
    let mut expander = text_template.expander();

    for &(attr, desc) in attrs {
        let desc = if desc { "✅" } else { "❌" };
        expander
            .sub("feature-rows")
            .set("attr-name", attr)
            .set("attr-avail", desc);
    }

    expander
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

impl RowGen for TopologyType {
    fn make_row(t: &'static str, attr: Self) -> (&'static str, String) {
        let s = format!("{}", attr);
        (t, s)
    }
}

fn main() {
    let cpuid = CpuId::new();
    let skin = MadSkin::default();

    skin.print_text("# CpuId\n");

    let table_template = TextTemplate::from(
        r#"
    |-:|-:|
    ${feature-rows
    |**${attr-name}**|${attr-avail}|
    }
    |-|-|
    "#,
    );

    if let Some(info) = cpuid.get_vendor_info() {
        print_title_attr("vendor_id (0x00)", info.as_str());
    }

    if let Some(info) = cpuid.get_feature_info() {
        print_title("version information (1/eax):");
        let table = make_table(
            &table_template,
            &[
                ("base family", info.base_family_id().into()),
                ("base model", info.base_model_id().into()),
                ("stepping", info.stepping_id().into()),
                ("extended family", info.extended_family_id().into()),
                ("extended model", info.extended_model_id().into()),
                ("family", info.family_id().into()),
                ("model", info.model_id().into()),
            ],
        );
        skin.print_expander(table);

        print_title("miscellaneous (1/ebx):");
        let table = make_table(
            &table_template,
            &[
                (
                    "processor APIC physical id",
                    info.initial_local_apic_id().into(),
                ),
                ("max. cpus", info.max_logical_processor_ids().into()),
                ("CLFLUSH line size", info.cflush_cache_line_size().into()),
                ("brand index", info.brand_index().into()),
            ],
        );
        skin.print_expander(table);

        print_title("feature information (1/edx):");
        let table = make_feature_table(
            &table_template,
            &[
                ("fpu", info.has_fpu()),
                ("vme", info.has_vme()),
                ("de", info.has_de()),
                ("pse", info.has_pse()),
                ("tsc", info.has_tsc()),
                ("msr", info.has_msr()),
                ("pae", info.has_pae()),
                ("mce", info.has_mce()),
                ("cmpxchg8b", info.has_cmpxchg8b()),
                ("apic", info.has_apic()),
                ("sysenter_sysexit", info.has_sysenter_sysexit()),
                ("mtrr", info.has_mtrr()),
                ("pge", info.has_pge()),
                ("mca", info.has_mca()),
                ("cmov", info.has_cmov()),
                ("pat", info.has_pat()),
                ("pse36", info.has_pse36()),
                ("psn", info.has_psn()),
                ("clflush", info.has_clflush()),
                ("ds", info.has_ds()),
                ("acpi", info.has_acpi()),
                ("mmx", info.has_mmx()),
                ("fxsave_fxstor", info.has_fxsave_fxstor()),
                ("sse", info.has_sse()),
                ("sse2", info.has_sse2()),
                ("ss", info.has_ss()),
                ("htt", info.has_htt()),
                ("tm", info.has_tm()),
                ("pbe", info.has_pbe()),
            ],
        );
        skin.print_expander(table);

        print_title("feature information (1/ecx):");

        let table = make_feature_table(
            &table_template,
            &[
                ("sse3", info.has_sse3()),
                ("pclmulqdq", info.has_pclmulqdq()),
                ("ds_area", info.has_ds_area()),
                ("monitor_mwait", info.has_monitor_mwait()),
                ("cpl", info.has_cpl()),
                ("vmx", info.has_vmx()),
                ("smx", info.has_smx()),
                ("eist", info.has_eist()),
                ("tm2", info.has_tm2()),
                ("ssse3", info.has_ssse3()),
                ("cnxtid", info.has_cnxtid()),
                ("fma", info.has_fma()),
                ("cmpxchg16b", info.has_cmpxchg16b()),
                ("pdcm", info.has_pdcm()),
                ("pcid", info.has_pcid()),
                ("dca", info.has_dca()),
                ("sse41", info.has_sse41()),
                ("sse42", info.has_sse42()),
                ("x2apic", info.has_x2apic()),
                ("movbe", info.has_movbe()),
                ("popcnt", info.has_popcnt()),
                ("tsc_deadline", info.has_tsc_deadline()),
                ("aesni", info.has_aesni()),
                ("xsave", info.has_xsave()),
                ("oxsave", info.has_oxsave()),
                ("avx", info.has_avx()),
                ("f16c", info.has_f16c()),
                ("rdrand", info.has_rdrand()),
                ("hypervisor", info.has_hypervisor()),
            ],
        );
        skin.print_expander(table);
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

            let attrs = vec![
                ("cache type", format!("{:?}", cache.cache_type())),
                ("cache level", format!("{}", cache.level())),
                (
                    "self-initializing cache level",
                    bool_repr(cache.is_self_initializing()),
                ),
                (
                    "fully associative cache",
                    bool_repr(cache.is_fully_associative()),
                ),
                (
                    "threads sharing this cache",
                    format!("{}", cache.max_cores_for_cache()),
                ),
                (
                    "processor cores on this die",
                    format!("{}", cache.max_cores_for_package()),
                ),
                (
                    "system coherency line size",
                    format!("{}", cache.coherency_line_size()),
                ),
                (
                    "physical line partitions",
                    format!("{}", cache.physical_line_partitions()),
                ),
                (
                    "ways of associativity",
                    format!("{}", cache.associativity()),
                ),
                (
                    "WBINVD/INVD acts on lower caches",
                    bool_repr(cache.is_write_back_invalidate()),
                ),
                ("inclusive to lower caches", bool_repr(cache.is_inclusive())),
                (
                    "complex cache indexing",
                    bool_repr(cache.has_complex_indexing()),
                ),
                ("number of sets", format!("{}", cache.sets())),
                ("(size synth.)", format!("{}", size)),
            ];

            let table = make_table_display(&table_template, &*attrs);
            skin.print_expander(table);
        }
    }

    if let Some(info) = cpuid.get_monitor_mwait_info() {
        print_title("MONITOR/MWAIT (0x05):");

        let attrs = vec![
            (
                "smallest monitor-line size",
                format!("{:?}", info.smallest_monitor_line()),
            ),
            (
                "largest monitor-line size",
                format!("{}", info.largest_monitor_line()),
            ),
            (
                "MONITOR/MWAIT exts supported",
                bool_repr(info.extensions_supported()),
            ),
            (
                "Interrupts as break-event for MWAIT",
                bool_repr(info.interrupts_as_break_event()),
            ),
        ];

        let table = make_table_display(&table_template, &*attrs);
        skin.print_expander(table);

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
        let attrs = vec![
            ("digital thermometer", bool_repr(info.has_dts())),
            (
                "Intel Turbo Boost Technology",
                bool_repr(info.has_turbo_boost()),
            ),
            ("ARAT always running APIC timer", bool_repr(info.has_arat())),
            ("PLN power limit notification", bool_repr(info.has_pln())),
            (
                "ECMD extended clock modulation duty",
                bool_repr(info.has_ecmd()),
            ),
            ("PTM package thermal management", bool_repr(info.has_ptm())),
            ("HWP base registers", bool_repr(info.has_hwp())),
            ("HWP notification", bool_repr(info.has_hwp_notification())),
            (
                "HWP activity window",
                bool_repr(info.has_hwp_activity_window()),
            ),
            (
                "HWP energy performance preference",
                bool_repr(info.has_hwp_energy_performance_preference()),
            ),
            (
                "HWP package level request",
                bool_repr(info.has_hwp_package_level_request()),
            ),
            ("HDC base registers", bool_repr(info.has_hdc())),
            (
                "Intel Turbo Boost Max Technology 3.0",
                bool_repr(info.has_turbo_boost3()),
            ),
            ("HWP capabilities", bool_repr(info.has_hwp_capabilities())),
            ("HWP PECI override", bool_repr(info.has_hwp_peci_override())),
            ("flexible HWP", bool_repr(info.has_flexible_hwp())),
            (
                "IA32_HWP_REQUEST MSR fast access mode",
                bool_repr(info.has_hwp_fast_access_mode()),
            ),
            (
                "ignoring idle logical processor HWP req",
                bool_repr(info.has_ignore_idle_processor_hwp_request()),
            ),
            (
                "digital thermometer threshold",
                format!("{}", info.dts_irq_threshold()),
            ),
            (
                "hardware coordination feedback",
                bool_repr(info.has_hw_coord_feedback()),
            ),
            (
                "performance-energy bias capability",
                bool_repr(info.has_energy_bias_pref()),
            ),
        ];

        let table = make_table_display(&table_template, &*attrs);
        skin.print_expander(table);
    }
    if let Some(info) = cpuid.get_extended_feature_info() {
        print_title("Extended feature flags (0x07):");

        let attrs = [
            ("FSGSBASE instructions", bool_repr(info.has_fsgsbase())),
            (
                "IA32_TSC_ADJUST MSR supported",
                bool_repr(info.has_tsc_adjust_msr()),
            ),
            (
                "SGX: Software Guard Extensions supported",
                bool_repr(info.has_sgx()),
            ),
            ("BMI1 instructions", bool_repr(info.has_bmi1())),
            ("HLE hardware lock elision", bool_repr(info.has_hle())),
            (
                "AVX2: advanced vector extensions 2",
                bool_repr(info.has_avx2()),
            ),
            ("FDP_EXCPTN_ONLY", bool_repr(info.has_fdp())),
            (
                "SMEP supervisor mode exec protection",
                bool_repr(info.has_smep()),
            ),
            ("BMI2 instructions", bool_repr(info.has_bmi2())),
            (
                "enhanced REP MOVSB/STOSB",
                bool_repr(info.has_rep_movsb_stosb()),
            ),
            ("INVPCID instruction", bool_repr(info.has_invpcid())),
            (
                "RTM: restricted transactional memory",
                bool_repr(info.has_rtm()),
            ),
            ("RDT-CMT/PQoS cache monitoring", bool_repr(info.has_rdtm())),
            (
                "deprecated FPU CS/DS",
                bool_repr(info.has_fpu_cs_ds_deprecated()),
            ),
            (
                "MPX: intel memory protection extensions",
                bool_repr(info.has_mpx()),
            ),
            ("RDT-CAT/PQE cache allocation", bool_repr(info.has_rdta())),
            (
                "AVX512F: AVX-512 foundation instructions",
                bool_repr(info.has_avx512f()),
            ),
            (
                "AVX512DQ: double & quadword instructions",
                bool_repr(info.has_avx512dq()),
            ),
            ("RDSEED instruction", bool_repr(info.has_rdseed())),
            ("ADX instructions", bool_repr(info.has_adx())),
            (
                "SMAP: supervisor mode access prevention",
                bool_repr(info.has_smap()),
            ),
            (
                "AVX512IFMA: fused multiply add",
                bool_repr(info.has_avx512_ifma()),
            ),
            ("CLFLUSHOPT instruction", bool_repr(info.has_clflushopt())),
            ("CLWB instruction", bool_repr(info.has_clwb())),
            (
                "Intel processor trace",
                bool_repr(info.has_processor_trace()),
            ),
            (
                "AVX512PF: prefetch instructions",
                bool_repr(info.has_avx512pf()),
            ),
            (
                "AVX512ER: exponent & reciprocal instrs",
                bool_repr(info.has_avx512er()),
            ),
            (
                "AVX512CD: conflict detection instrs",
                bool_repr(info.has_avx512cd()),
            ),
            ("SHA instructions", bool_repr(info.has_sha())),
            (
                "AVX512BW: byte & word instructions",
                bool_repr(info.has_avx512bw()),
            ),
            ("AVX512VL: vector length", bool_repr(info.has_avx512vl())),
            ("PREFETCHWT1", bool_repr(info.has_prefetchwt1())),
            (
                "UMIP: user-mode instruction prevention",
                bool_repr(info.has_umip()),
            ),
            (
                "PKU protection keys for user-mode",
                bool_repr(info.has_pku()),
            ),
            (
                "OSPKE CR4.PKE and RDPKRU/WRPKRU",
                bool_repr(info.has_ospke()),
            ),
            (
                "BNDLDX/BNDSTX MAWAU value in 64-bit mode",
                format!("{}", info.mawau_value()),
            ),
            (
                "RDPID: read processor ID supported",
                bool_repr(info.has_rdpid()),
            ),
            (
                "SGX_LC: SGX launch config supported",
                bool_repr(info.has_sgx_lc()),
            ),
        ];
        let table = make_table_display(&table_template, &attrs);
        skin.print_expander(table);
    }
    if let Some(info) = cpuid.get_direct_cache_access_info() {
        print_title("Direct Cache Access Parameters (0x09):");
        print_attr("PLATFORM_DCA_CAP MSR bits", info.get_dca_cap_value());
    }
    if let Some(info) = cpuid.get_performance_monitoring_info() {
        print_title("Architecture Performance Monitoring Features (0x0a)");
        print_subtitle("Monitoring Hardware Info (0x0a/{eax, edx}):");
        let table = make_table_display(
            &table_template,
            &[
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
            ],
        );
        skin.print_expander(table);

        print_subtitle("Monitoring Hardware Features (0x0a/ebx):");
        let attrs = [
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
                "branch mispred retired event not avail",
                info.is_branch_midpred_ev_unavailable(),
            ),
        ];
        let table = make_table_display(&table_template, &attrs);
        skin.print_expander(table);
    }
    if let Some(info) = cpuid.get_extended_topology_info() {
        print_title("x2APIC features / processor topology (0x0b):");

        for level in info {
            print_subtitle(format!("level {}:", level.level_number()).as_str());

            let attrs = [
                RowGen::make_row("level type", level.level_type()),
                RowGen::make_row("bit width of level", level.shift_right_for_next_apic_id()),
                RowGen::make_row("number of logical processors at level", level.processors()),
                RowGen::make_row("x2apic id of current processor", level.x2apic_id()),
            ];
            let table = make_table_display(&table_template, &attrs);
            skin.print_expander(table);
        }
    }
    if let Some(info) = cpuid.get_extended_state_info() {
        print_title("Extended Register State (0x0d/0):");

        let table_template3 = TextTemplate::from(
            r#"
|-:|-:|-:|
${feature-rows
|**${category-name}**|**${attr-name}**|${attr-avail}|
}
|-|-|
    "#,
        );

        print_subtitle("XCR0/IA32_XSS supported states:");
        let attrs = [
            (
                "XCR0",
                "x87 state",
                bool_repr(info.xcr0_supports_legacy_x87()),
            ),
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
            (
                "IA32_XSS",
                "PT state",
                bool_repr(info.ia32_xss_supports_pt()),
            ),
            ("XCR0", "PKRU state", bool_repr(info.xcr0_supports_pkru())),
            //("XCR0", "CET_U state", xxx),
            //("XCR0", "CET_S state", xxx),
            (
                "IA32_XSS",
                "HDC state",
                bool_repr(info.ia32_xss_supports_hdc()),
            ),
        ];
        let table = make_table_display3(&table_template3, &attrs);
        skin.print_expander(table);
    }

    /*
    if let Some(info) = cpuid.get_rdt_monitoring_info() {
        println!("RDT Monitoring");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_rdt_allocation_info() {
        println!("RDT Allocation");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_sgx_info() {
        println!("Software Guard Extensions");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_processor_trace_info() {
        println!("Processor Trace");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_tsc_info() {
        println!("TSC");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_processor_frequency_info() {
        println!("Processor Frequency");
        println!("{:?}", info);
    }
    if let Some(dats) = cpuid.get_deterministic_address_translation_info() {
        println!("Deterministic Address Translation");
        for dat in dats {
            println!("{:?}", dat);
        }
    }
    if let Some(info) = cpuid.get_soc_vendor_info() {
        println!("SoC Vendor Info");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_processor_brand_string() {
        println!("Processor Brand String");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_memory_encryption_info() {
        println!("Memory Encryption Info");
        println!("{:?}", info);
    }
    */
}

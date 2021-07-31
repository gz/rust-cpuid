use std::fmt::Display;

use raw_cpuid::{
    CpuId, CpuIdResult, DatType, ExtendedRegisterStateLocation, SgxSectionInfo, SoCVendorBrand,
    TopologyType,
};
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

    let table = make_table_display(&table_template, &attrs);
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
                "branch mispred retired event not avail",
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

        let table_template3 = TextTemplate::from(
            r#"
|:-|-:|-:|
${feature-rows
|**${category-name}**|**${attr-name}**|${attr-avail}|
}
|-|-|
    "#,
        );

        print_subtitle("XCR0/IA32_XSS supported states:");
        let attrs = [
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
        ];
        let table = make_table_display3(&table_template3, &attrs);
        skin.print_expander(table);

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
            RowGen::make_row("SAVE area size in bytes", info.xsave_size()),
        ]);

        for state in info.iter() {
            print_subtitle(
                format!("{} features (0x0d/{}):", state.register(), state.subleaf).as_str(),
            );
            simple_table(&[
                RowGen::make_row("save state byte size", state.size()),
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
            RowGen::make_row("supports L3 cache QoS monitoring", info.has_l3_monitoring()),
        ]);

        if let Some(rmid) = info.l3_monitoring() {
            print_subtitle("L3 Cache Quality of Service Monitoring (0x0f/1):");

            simple_table(&[
                RowGen::make_row(
                    "Conversion factor from IA32_QM_CTR to bytes",
                    rmid.conversion_factor(),
                ),
                RowGen::make_row("Maximum range of RMID", rmid.maximum_rmid_range()),
                RowGen::make_row(
                    "supports L3 occupancy monitoring",
                    rmid.has_occupancy_monitoring(),
                ),
                RowGen::make_row(
                    "supports L3 total bandwidth monitoring",
                    rmid.has_total_bandwidth_monitoring(),
                ),
                RowGen::make_row(
                    "supports L3 local bandwidth monitoring",
                    rmid.has_local_bandwidth_monitoring(),
                ),
            ]);
        }
    }

    if let Some(info) = cpuid.get_rdt_allocation_info() {
        print_title("Resource Director Technology Allocation (0x10/0)");
        simple_table(&[
            RowGen::make_row(
                "L3 cache allocation technology supported",
                info.has_l3_cat(),
            ),
            RowGen::make_row(
                "L2 cache allocation technology supported",
                info.has_l2_cat(),
            ),
            RowGen::make_row(
                "memory bandwidth allocation supported",
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
                    "code and data prioritization supported",
                    l3_cat.has_code_data_prioritization(),
                ),
                RowGen::make_row("highest COS number supported", l3_cat.highest_cos()),
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
                RowGen::make_row("highest COS number supported", l2_cat.highest_cos()),
            ]);
        }
        if let Some(mem) = info.memory_bandwidth_allocation() {
            print_subtitle("Memory Bandwidth Allocation (0x10/3):");
            simple_table(&[
                RowGen::make_row("maximum throttling value", mem.max_hba_throttling()),
                RowGen::make_row("delay values are linear", mem.has_linear_response_delay()),
                RowGen::make_row("highest COS number supported", mem.highest_cos()),
            ]);
        }
    }

    if let Some(info) = cpuid.get_sgx_info() {
        print_title("SGX - Software Guard Extensions (0x12/{0,1}):");

        simple_table(&[
            RowGen::make_row("SGX1 supported", info.has_sgx1()),
            RowGen::make_row("SGX2 supported", info.has_sgx2()),
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
            RowGen::make_row("PTWRITE support", info.has_ptwrite()),
            RowGen::make_row("power event trace support", info.has_power_event_trace()),
            RowGen::make_row("ToPA output scheme support", info.has_topa()),
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
                RowGen::make_row("4 KiB page size entries supported", info.has_4k_entries()),
                RowGen::make_row("2 MiB page size entries supported", info.has_2mb_entries()),
                RowGen::make_row("4 MiB page size entries supported", info.has_4mb_entries()),
                RowGen::make_row("1 GiB page size entries supported", info.has_1gb_entries()),
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
        print_attr("Processor Brand String", info.as_str());
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
            RowGen::make_row("Restricted Injection", info.has_restricted_injection()),
            RowGen::make_row("Alternate Injection", info.has_alternate_injection()),
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

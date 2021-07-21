use raw_cpuid::CpuId;
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

fn make_feature_table<'a, 'b>(
    text_template: &'a TextTemplate<'b>,
    attrs: &[(&'b str, bool)],
) -> TextTemplateExpander<'a, 'b> {
    let mut expander = text_template.expander();
    expander.set("app-version", "2");

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
    let mut skin = MadSkin::default();
    skin.print_text(format!("## {}\n", title).as_str());
}

fn print_title_attr(title: &str, attr: &str) {
    print_title_line(title, Some(attr));
}

fn print_title(title: &str) {
    print_title_line(title, None)
}

fn main() {
    let cpuid = CpuId::new();
    let mut skin = MadSkin::default();

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
        println!("Processor Serial");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_cache_parameters() {
        println!("Cache Parameters");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_monitor_mwait_info() {
        println!("Monitor/MWait");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_thermal_power_info() {
        println!("Thermal Power");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_extended_feature_info() {
        println!("Extended Features");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_direct_cache_access_info() {
        println!("Direct Cache Access");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_performance_monitoring_info() {
        println!("Performance Monitoring");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_extended_topology_info() {
        println!("Extended Topology");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_extended_state_info() {
        println!("Extended State");
        println!("{:?}", info);
    }
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
}

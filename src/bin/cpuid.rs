use std::fmt::Display;

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

            let mut attrs = vec![
                ("cache type", format!("{:?}", cache.cache_type())),
                ("cache level", format!("{}", cache.level())),
                (
                    "self-initializing cache level",
                    if cache.is_self_initializing() {
                        "✅".to_string()
                    } else {
                        "❌".to_string()
                    },
                ),
                (
                    "fully associative cache",
                    if cache.is_fully_associative() {
                        "✅".to_string()
                    } else {
                        "❌".to_string()
                    },
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
                    if cache.is_write_back_invalidate() {
                        "✅".to_string()
                    } else {
                        "❌".to_string()
                    },
                ),
                (
                    "inclusive to lower caches",
                    if cache.is_inclusive() {
                        "✅".to_string()
                    } else {
                        "❌".to_string()
                    },
                ),
                (
                    "complex cache indexing",
                    if cache.has_complex_indexing() {
                        "✅".to_string()
                    } else {
                        "❌".to_string()
                    },
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

        let mut attrs = vec![
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
                if info.extensions_supported() {
                    "✅".to_string()
                } else {
                    "❌".to_string()
                },
            ),
            (
                "Interrupts as break-event for MWAIT",
                if info.interrupts_as_break_event() {
                    "✅".to_string()
                } else {
                    "❌".to_string()
                },
            ),
        ];

        let table = make_table_display(&table_template, &*attrs);
        skin.print_expander(table);

        skin.print_text("number of CX sub C-states using MWAIT:\n");
        let cstate_table = TextTemplate::from(
            r#"
        |:-|-:|
        |**C0**|**C1**|**C2**|**C3**|**C4**|**C5**|**C6**|**C7**|
        |${c0}|${c1}|${c2}|${c3}|${c4}|${c5}|${c6}|${c7}|
        |-|-|
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

    /*
    ///   MONITOR/MWAIT (5):
    ///      smallest monitor-line size (bytes)       = 0x40 (64)
    ///      largest monitor-line size (bytes)        = 0x40 (64)
    ///      enum of Monitor-MWAIT exts supported     = true
    ///      supports intrs as break-event for MWAIT  = true
    ///      number of C0 sub C-states using MWAIT    = 0x1 (1)
    ///      number of C1 sub C-states using MWAIT    = 0x1 (1)
    ///      number of C2 sub C-states using MWAIT    = 0x0 (0)
    ///      number of C3 sub C-states using MWAIT    = 0x0 (0)
    ///      number of C4 sub C-states using MWAIT    = 0x0 (0)
    ///      number of C5 sub C-states using MWAIT    = 0x0 (0)
    ///      number of C6 sub C-states using MWAIT    = 0x0 (0)
    ///      number of C7 sub C-states using MWAIT    = 0x0 (0)

        */

    /*
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
    */
}

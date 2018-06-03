use *;

extern crate serde_json;

#[test]
fn genuine_intel() {
    let vf = VendorInfo {
        ebx: 1970169159,
        edx: 1231384169,
        ecx: 1818588270,
    };
    assert!(vf.as_string() == "GenuineIntel");
}

#[test]
fn feature_info() {
    let finfo = FeatureInfo {
        eax: 198313,
        ebx: 34605056,
        edx_ecx: FeatureInfoFlags {
            bits: 2109399999 | 3219913727 << 32,
        },
    };

    assert!(finfo.model_id() == 10);
    assert!(finfo.extended_model_id() == 3);
    assert!(finfo.stepping_id() == 9);
    assert!(finfo.extended_family_id() == 0);
    assert!(finfo.family_id() == 6);
    assert!(finfo.stepping_id() == 9);
    assert!(finfo.brand_index() == 0);

    assert!(finfo.edx_ecx.contains(FeatureInfoFlags::SSE2));
    assert!(finfo.edx_ecx.contains(FeatureInfoFlags::SSE41));
}

#[test]
fn cache_info() {
    let cinfos = CacheInfoIter {
        current: 1,
        eax: 1979931137,
        ebx: 15774463,
        ecx: 0,
        edx: 13238272,
    };
    for (idx, cache) in cinfos.enumerate() {
        match idx {
            0 => assert!(cache.num == 0xff),
            1 => assert!(cache.num == 0x5a),
            2 => assert!(cache.num == 0xb2),
            3 => assert!(cache.num == 0x03),
            4 => assert!(cache.num == 0xf0),
            5 => assert!(cache.num == 0xca),
            6 => assert!(cache.num == 0x76),
            _ => unreachable!(),
        }
    }
}

#[test]
fn cache_parameters() {
    let caches: [CacheParameter; 4] = [
        CacheParameter {
            eax: 469778721,
            ebx: 29360191,
            ecx: 63,
            edx: 0,
        },
        CacheParameter {
            eax: 469778722,
            ebx: 29360191,
            ecx: 63,
            edx: 0,
        },
        CacheParameter {
            eax: 469778755,
            ebx: 29360191,
            ecx: 511,
            edx: 0,
        },
        CacheParameter {
            eax: 470008163,
            ebx: 46137407,
            ecx: 4095,
            edx: 6,
        },
    ];

    for (idx, cache) in caches.into_iter().enumerate() {
        match idx {
            0 => {
                assert!(cache.cache_type() == CacheType::DATA);
                assert!(cache.level() == 1);
                assert!(cache.is_self_initializing());
                assert!(!cache.is_fully_associative());
                assert!(cache.max_cores_for_cache() == 2);
                assert!(cache.max_cores_for_package() == 8);
                assert!(cache.coherency_line_size() == 64);
                assert!(cache.physical_line_partitions() == 1);
                assert!(cache.associativity() == 8);
                assert!(!cache.is_write_back_invalidate());
                assert!(!cache.is_inclusive());
                assert!(!cache.has_complex_indexing());
                assert!(cache.sets() == 64);
            }
            1 => {
                assert!(cache.cache_type() == CacheType::INSTRUCTION);
                assert!(cache.level() == 1);
                assert!(cache.is_self_initializing());
                assert!(!cache.is_fully_associative());
                assert!(cache.max_cores_for_cache() == 2);
                assert!(cache.max_cores_for_package() == 8);
                assert!(cache.coherency_line_size() == 64);
                assert!(cache.physical_line_partitions() == 1);
                assert!(cache.associativity() == 8);
                assert!(!cache.is_write_back_invalidate());
                assert!(!cache.is_inclusive());
                assert!(!cache.has_complex_indexing());
                assert!(cache.sets() == 64);
            }
            2 => {
                assert!(cache.cache_type() == CacheType::UNIFIED);
                assert!(cache.level() == 2);
                assert!(cache.is_self_initializing());
                assert!(!cache.is_fully_associative());
                assert!(cache.max_cores_for_cache() == 2);
                assert!(cache.max_cores_for_package() == 8);
                assert!(cache.coherency_line_size() == 64);
                assert!(cache.physical_line_partitions() == 1);
                assert!(cache.associativity() == 8);
                assert!(!cache.is_write_back_invalidate());
                assert!(!cache.is_inclusive());
                assert!(!cache.has_complex_indexing());
                assert!(cache.sets() == 512);
            }
            3 => {
                assert!(cache.cache_type() == CacheType::UNIFIED);
                assert!(cache.level() == 3);
                assert!(cache.is_self_initializing());
                assert!(!cache.is_fully_associative());
                assert!(cache.max_cores_for_cache() == 16);
                assert!(cache.max_cores_for_package() == 8);
                assert!(cache.coherency_line_size() == 64);
                assert!(cache.physical_line_partitions() == 1);
                assert!(cache.associativity() == 12);
                assert!(!cache.is_write_back_invalidate());
                assert!(cache.is_inclusive());
                assert!(cache.has_complex_indexing());
                assert!(cache.sets() == 4096);
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn monitor_mwait_features() {
    let mmfeatures = MonitorMwaitInfo {
        eax: 64,
        ebx: 64,
        ecx: 3,
        edx: 135456,
    };
    assert!(mmfeatures.smallest_monitor_line() == 64);
    assert!(mmfeatures.largest_monitor_line() == 64);
    assert!(mmfeatures.extensions_supported());
    assert!(mmfeatures.interrupts_as_break_event());
    assert!(mmfeatures.supported_c0_states() == 0);
    assert!(mmfeatures.supported_c1_states() == 2);
    assert!(mmfeatures.supported_c2_states() == 1);
    assert!(mmfeatures.supported_c3_states() == 1);
    assert!(mmfeatures.supported_c4_states() == 2);
    assert!(mmfeatures.supported_c5_states() == 0);
    assert!(mmfeatures.supported_c6_states() == 0);
    assert!(mmfeatures.supported_c7_states() == 0);
}

#[test]
fn thermal_power_features() {
    let tpfeatures = ThermalPowerInfo {
        eax: ThermalPowerFeaturesEax { bits: 119 },
        ebx: 2,
        ecx: ThermalPowerFeaturesEcx { bits: 9 },
        edx: 0,
    };

    assert!(tpfeatures.eax.contains(ThermalPowerFeaturesEax::DTS));
    assert!(
        tpfeatures
            .eax
            .contains(ThermalPowerFeaturesEax::TURBO_BOOST)
    );
    assert!(tpfeatures.eax.contains(ThermalPowerFeaturesEax::ARAT));
    assert!(tpfeatures.eax.contains(ThermalPowerFeaturesEax::PLN));
    assert!(tpfeatures.eax.contains(ThermalPowerFeaturesEax::ECMD));
    assert!(tpfeatures.eax.contains(ThermalPowerFeaturesEax::PTM));

    assert!(
        tpfeatures
            .ecx
            .contains(ThermalPowerFeaturesEcx::HW_COORD_FEEDBACK)
    );
    assert!(
        tpfeatures
            .ecx
            .contains(ThermalPowerFeaturesEcx::ENERGY_BIAS_PREF)
    );

    assert!(tpfeatures.dts_irq_threshold() == 0x2);
}

#[test]
fn extended_features() {
    let tpfeatures = ExtendedFeatures {
        eax: 0,
        ebx: ExtendedFeaturesEbx { bits: 641 },
        ecx: ExtendedFeaturesEcx { bits: 0 },
        edx: 0,
    };
    assert!(tpfeatures.eax == 0);
    assert!(tpfeatures.has_fsgsbase());
    assert!(!tpfeatures.has_tsc_adjust_msr());
    assert!(!tpfeatures.has_bmi1());
    assert!(!tpfeatures.has_hle());
    assert!(!tpfeatures.has_avx2());
    assert!(tpfeatures.has_smep());
    assert!(!tpfeatures.has_bmi2());
    assert!(tpfeatures.has_rep_movsb_stosb());
    assert!(!tpfeatures.has_invpcid());
    assert!(!tpfeatures.has_rtm());
    assert!(!tpfeatures.has_rdtm());
    assert!(!tpfeatures.has_fpu_cs_ds_deprecated());

    let tpfeatures2 = ExtendedFeatures {
        eax: 0,
        ebx: ExtendedFeaturesEbx::FSGSBASE
            | ExtendedFeaturesEbx::ADJUST_MSR
            | ExtendedFeaturesEbx::BMI1
            | ExtendedFeaturesEbx::AVX2
            | ExtendedFeaturesEbx::SMEP
            | ExtendedFeaturesEbx::BMI2
            | ExtendedFeaturesEbx::REP_MOVSB_STOSB
            | ExtendedFeaturesEbx::INVPCID
            | ExtendedFeaturesEbx::DEPRECATE_FPU_CS_DS
            | ExtendedFeaturesEbx::MPX
            | ExtendedFeaturesEbx::RDSEED
            | ExtendedFeaturesEbx::ADX
            | ExtendedFeaturesEbx::SMAP
            | ExtendedFeaturesEbx::CLFLUSHOPT
            | ExtendedFeaturesEbx::PROCESSOR_TRACE,
        ecx: ExtendedFeaturesEcx { bits: 0 },
        edx: 201326592,
    };

    assert!(tpfeatures2.has_fsgsbase());
    assert!(tpfeatures2.has_tsc_adjust_msr());
    assert!(tpfeatures2.has_bmi1());
    assert!(tpfeatures2.has_avx2());
    assert!(tpfeatures2.has_smep());
    assert!(tpfeatures2.has_bmi2());
    assert!(tpfeatures2.has_rep_movsb_stosb());
    assert!(tpfeatures2.has_invpcid());
    assert!(tpfeatures2.has_fpu_cs_ds_deprecated());
    assert!(tpfeatures2.has_mpx());
    assert!(tpfeatures2.has_rdseed());
    assert!(tpfeatures2.has_adx());
    assert!(tpfeatures2.has_smap());
    assert!(tpfeatures2.has_clflushopt());
    assert!(tpfeatures2.has_processor_trace());
}

#[test]
fn direct_cache_access_info() {
    let dca = DirectCacheAccessInfo { eax: 0x1 };
    assert!(dca.get_dca_cap_value() == 0x1);
}

#[test]
fn performance_monitoring_info() {
    let pm = PerformanceMonitoringInfo {
        eax: 120587267,
        ebx: PerformanceMonitoringFeaturesEbx { bits: 0 },
        ecx: 0,
        edx: 1539,
    };

    assert!(pm.version_id() == 3);
    assert!(pm.number_of_counters() == 4);
    assert!(pm.counter_bit_width() == 48);
    assert!(pm.ebx_length() == 7);
    assert!(pm.fixed_function_counters() == 3);
    assert!(pm.fixed_function_counters_bit_width() == 48);

    assert!(
        !pm.ebx
            .contains(PerformanceMonitoringFeaturesEbx::CORE_CYC_EV_UNAVAILABLE)
    );
    assert!(
        !pm.ebx
            .contains(PerformanceMonitoringFeaturesEbx::INST_RET_EV_UNAVAILABLE)
    );
    assert!(
        !pm.ebx
            .contains(PerformanceMonitoringFeaturesEbx::REF_CYC_EV_UNAVAILABLE)
    );
    assert!(
        !pm.ebx
            .contains(PerformanceMonitoringFeaturesEbx::CACHE_REF_EV_UNAVAILABLE)
    );
    assert!(
        !pm.ebx
            .contains(PerformanceMonitoringFeaturesEbx::LL_CACHE_MISS_EV_UNAVAILABLE)
    );
    assert!(
        !pm.ebx
            .contains(PerformanceMonitoringFeaturesEbx::BRANCH_INST_RET_EV_UNAVAILABLE)
    );
    assert!(
        !pm.ebx
            .contains(PerformanceMonitoringFeaturesEbx::BRANCH_MISPRED_EV_UNAVAILABLE)
    );
}

#[cfg(test)]
#[test]
fn extended_topology_info() {
    let l1 = ExtendedTopologyLevel {
        eax: 1,
        ebx: 2,
        ecx: 256,
        edx: 3,
    };
    let l2 = ExtendedTopologyLevel {
        eax: 4,
        ebx: 4,
        ecx: 513,
        edx: 3,
    };

    assert!(l1.processors() == 2);
    assert!(l1.level_number() == 0);
    assert!(l1.level_type() == TopologyType::SMT);
    assert!(l1.x2apic_id() == 3);
    assert!(l1.shift_right_for_next_apic_id() == 1);

    assert!(l2.processors() == 4);
    assert!(l2.level_number() == 1);
    assert!(l2.level_type() == TopologyType::CORE);
    assert!(l2.x2apic_id() == 3);
    assert!(l2.shift_right_for_next_apic_id() == 4);
}

#[test]
fn extended_state_info() {
    let es = ExtendedStateInfo {
        eax: 7,
        ebx: 832,
        ecx: 832,
        edx: 0,
        eax1: 1,
    };

    assert!(es.xcr0_supported() == 7);
    assert!(es.maximum_size_enabled_features() == 832);
    assert!(es.maximum_size_supported_features() == 832);
    assert!(es.has_xsaveopt());

    for (idx, e) in es.iter().enumerate() {
        match idx {
            0 => {
                assert!(e.subleaf == 2);
                assert!(e.size() == 256);
                assert!(e.offset() == 576);
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn quality_of_service_info() {
    let qos = RdtMonitoringInfo { ebx: 832, edx: 0 };

    assert!(qos.rmid_range() == 832);
    assert!(!qos.has_l3_monitoring());
}

#[test]
fn extended_functions() {
    let ef = ExtendedFunctionInfo {
        max_eax_value: 8,
        data: [
            CpuIdResult {
                eax: 2147483656,
                ebx: 0,
                ecx: 0,
                edx: 0,
            },
            CpuIdResult {
                eax: 0,
                ebx: 0,
                ecx: 1,
                edx: 672139264,
            },
            CpuIdResult {
                eax: 538976288,
                ebx: 1226842144,
                ecx: 1818588270,
                edx: 539578920,
            },
            CpuIdResult {
                eax: 1701998403,
                ebx: 692933672,
                ecx: 758475040,
                edx: 926102323,
            },
            CpuIdResult {
                eax: 1346576469,
                ebx: 541073493,
                ecx: 808988209,
                edx: 8013895,
            },
            CpuIdResult {
                eax: 0,
                ebx: 0,
                ecx: 0,
                edx: 0,
            },
            CpuIdResult {
                eax: 0,
                ebx: 0,
                ecx: 16801856,
                edx: 0,
            },
            CpuIdResult {
                eax: 0,
                ebx: 0,
                ecx: 0,
                edx: 256,
            },
            CpuIdResult {
                eax: 12324,
                ebx: 0,
                ecx: 0,
                edx: 0,
            },
        ],
    };

    assert_eq!(
        ef.processor_brand_string().unwrap(),
        "       Intel(R) Core(TM) i5-3337U CPU @ 1.80GHz"
    );
    assert!(ef.has_lahf_sahf());
    assert!(!ef.has_lzcnt());
    assert!(!ef.has_prefetchw());
    assert!(ef.has_syscall_sysret());
    assert!(ef.has_execute_disable());
    assert!(!ef.has_1gib_pages());
    assert!(ef.has_rdtscp());
    assert!(ef.has_64bit_mode());
    assert!(ef.has_invariant_tsc());

    assert!(ef.extended_signature().unwrap() == 0x0);
    assert!(ef.cache_line_size().unwrap() == 64);
    assert!(ef.l2_associativity().unwrap() == L2Associativity::EightWay);
    assert!(ef.cache_size().unwrap() == 256);
    assert!(ef.physical_address_bits().unwrap() == 36);
    assert!(ef.linear_address_bits().unwrap() == 48);
}

#[cfg(test)]
#[test]
fn test_serializability() {
    #[derive(Debug, Default, Serialize, Deserialize)]
    struct SerializeDeserializeTest {
        _x1: CpuId,
        _x2: CpuIdResult,
        _x3: VendorInfo,
        _x4: CacheInfoIter,
        _x5: CacheInfo,
        _x6: ProcessorSerial,
        _x7: FeatureInfo,
        _x8: CacheParametersIter,
        _x9: CacheParameter,
        _x10: MonitorMwaitInfo,
        _x11: ThermalPowerInfo,
        _x12: ExtendedFeatures,
        _x13: DirectCacheAccessInfo,
        _x14: PerformanceMonitoringInfo,
        _x15: ExtendedTopologyIter,
        _x16: ExtendedTopologyLevel,
        _x17: ExtendedStateInfo,
        _x18: ExtendedStateIter,
        _x19: ExtendedState,
        _x20: RdtAllocationInfo,
        _x21: RdtMonitoringInfo,
        _x22: L3CatInfo,
        _x23: L2CatInfo,
        _x24: ProcessorTraceInfo,
        _x25: ProcessorTraceIter,
        _x26: ProcessorTrace,
        _x27: TscInfo,
        _x28: ProcessorFrequencyInfo,
        _x29: SoCVendorInfo,
        _x30: SoCVendorAttributesIter,
        _x31: SoCVendorBrand,
        _x32: ExtendedFunctionInfo,
        _x33: MemBwAllocationInfo,
        _x34: L3MonitoringInfo,
    }

    let st: SerializeDeserializeTest = Default::default();
    let test = serde_json::to_string(&st).unwrap();
    let _st: SerializeDeserializeTest = serde_json::from_str(&test).unwrap();
}

#[cfg(test)]
#[test]
fn readme_test() {
    // let cpuid = CpuId::new();
    //
    // match cpuid.get_vendor_info() {
    // Some(vf) => assert!(vf.as_string() == "GenuineIntel"),
    // None => ()
    // }
    //
    // let has_sse = match cpuid.get_feature_info() {
    // Some(finfo) => finfo.has_sse(),
    // None => false
    // };
    //
    // if has_sse {
    // println!("CPU supports SSE!");
    // }
    //
    // match cpuid.get_cache_parameters() {
    // Some(cparams) => {
    // for cache in cparams {
    // let size = cache.associativity() * cache.physical_line_partitions() * cache.coherency_line_size() * cache.sets();
    // println!("L{}-Cache size is {}", cache.level(), size);
    // }
    // },
    // None => println!("No cache parameter information available"),
    // }
    //
}

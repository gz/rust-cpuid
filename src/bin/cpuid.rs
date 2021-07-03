extern crate raw_cpuid;

use raw_cpuid::CpuId;

fn main() {
    let cpuid = CpuId::new();
    // Implement Display for each of those structs
    if let Some(info) = cpuid.get_vendor_info() {
        println!("Vendor");
        println!("{}", info);
    };
    if let Some(info) = cpuid.get_feature_info() {
        println!("Feature");
        println!("{:?}", info);
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
    if let Some(dats) = cpuid.deterministic_address_translation_info() {
        println!("Deterministic Address Translation");
        for dat in dats {
            println!("{:?}", dat);
        }
    }
    if let Some(info) = cpuid.get_soc_vendor_info() {
        println!("SoC Vendor Info");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_extended_function_info() {
        println!("Extended Function Info");
        println!("{:?}", info);
    }
    if let Some(info) = cpuid.get_memory_encryption_info() {
        println!("Memory Encryption Info");
        println!("{:?}", info);
    }
}

extern crate raw_cpuid;
use raw_cpuid::{CpuId, ExtendedTopologyLevel, TopologyType};

fn main() {
    let cpuid = CpuId::new();

    cpuid.get_extended_function_info().map_or_else(
        || println!("Couldn't find processor serial number."),
        |extfuninfo| {
            println!(
                "CPU Model is: {}",
                extfuninfo.processor_brand_string().unwrap_or("Unknown CPU")
            )
        },
    );
    cpuid.get_extended_topology_info().map_or_else(
        || println!("No topology information available."),
        |topoiter| {
            let mut topology: Vec<ExtendedTopologyLevel> = topoiter.collect();
            topology.reverse();

            for topolevel in topology.iter() {
                let typ = if topolevel.level_type() == TopologyType::SMT {
                    "SMT-threads"
                } else {
                    "cores"
                };

                println!(
                    "At level {} it has: {} {}",
                    topolevel.level_number(),
                    topolevel.processors(),
                    typ
                );
            }
        },
    );
}

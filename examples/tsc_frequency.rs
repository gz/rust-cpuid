//! An example that displays RDTSC frequency.
extern crate raw_cpuid;
extern crate x86;

use std::time;
use x86::time::rdtscp;

const MHZ_TO_HZ: u64 = 1000000;
const KHZ_TO_HZ: u64 = 1000;

fn main() {
    let cpuid = raw_cpuid::CpuId::new();
    let has_tsc = cpuid
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_tsc());

    let has_invariant_tsc = cpuid
        .get_extended_function_info()
        .map_or(false, |efinfo| efinfo.has_invariant_tsc());

    let tsc_frequency_hz = cpuid.get_tsc_info().map(|tinfo| {
        if tinfo.nominal_frequency() != 0 {
            Some(tinfo.tsc_frequency())
        } else if tinfo.numerator() != 0 && tinfo.denominator() != 0 {
            // Skylake and Kabylake don't report the crystal clock, approximate with base frequency:
            cpuid
                .get_processor_frequency_info()
                .map(|pinfo| pinfo.processor_base_frequency() as u64 * MHZ_TO_HZ)
                .map(|cpu_base_freq_hz| {
                    let crystal_hz =
                        cpu_base_freq_hz * tinfo.denominator() as u64 / tinfo.numerator() as u64;
                    crystal_hz * tinfo.numerator() as u64 / tinfo.denominator() as u64
                })
        } else {
            None
        }
    });

    if has_tsc {
        println!(
            "TSC Frequency is: {} ({})",
            match tsc_frequency_hz {
                Some(x) => format!("{} Hz", x.unwrap_or(0)),
                None => String::from("unknown"),
            },
            if has_invariant_tsc {
                "invariant"
            } else {
                "TSC frequency varies with speed-stepping"
            }
        );
    }

    cpuid.get_hypervisor_info().map(|hv| {
        hv.tsc_frequency().map(|tsc_khz| {
            let virtual_tsc_frequency_hz = tsc_khz as u64 * KHZ_TO_HZ;
            println!(
                "Hypervisor reports TSC Frequency at: {} Hz",
                virtual_tsc_frequency_hz
            );
        })
    });

    let one_second = time::Duration::from_secs(1);
    unsafe {
        let now = time::Instant::now();
        let start = rdtscp();
        loop {
            if now.elapsed() >= one_second {
                break;
            }
        }
        let end = rdtscp();
        println!("Empirical measurement of TSC was: {} Hz", (end - start));
    }
}

#![feature(no_std, prelude_import, core, core_prelude, asm, raw)]
#![no_std]

#![crate_name = "raw_cpuid"]
#![crate_type = "lib"]

#[macro_use]
extern crate core;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
#[macro_use]
extern crate std;

use core::prelude::*;
use core::raw;
use core::str;
use core::mem::transmute;
use core::fmt;
use core::slice;

#[cfg(not(test))]
mod std {
    pub use core::ops;
    pub use core::option;
}


#[macro_export]
macro_rules! cpuid {
    ($eax:expr)
        => ( $crate::cpuid1($eax as u32) );

    ($eax:expr, $ecx:expr)
        => ( $crate::cpuid2($eax as u32, $ecx as u32) );

}

/// Execute CPUID instruction with eax and ecx register set.
pub fn cpuid2(eax: u32, ecx: u32) -> CpuIdResult {
    let mut res = CpuIdResult { eax: 0, ebx: 0, ecx: 0, edx: 0 };

    unsafe {
        asm!("movl $0, %eax" : : "{eax}" (eax) : "eax");
        asm!("movl $0, %ecx" : : "{ecx}" (ecx) : "ecx");
        asm!("cpuid" : "={eax}"(res.eax) "={ebx}"(res.ebx)
                       "={ecx}"(res.ecx) "={edx}"(res.edx)
                     :: "eax", "ebx", "ecx", "edx");
    }

    res
}

/// Execute CPUID instruction with eax register set.
pub fn cpuid1(eax: u32) -> CpuIdResult {
    let mut res = CpuIdResult { eax: 0, ebx: 0, ecx: 0, edx: 0 };

    unsafe {
        asm!("movl $0, %eax" : : "{eax}" (eax) : "eax");
        asm!("cpuid" : "={eax}"(res.eax) "={ebx}"(res.ebx)
                       "={ecx}"(res.ecx) "={edx}"(res.edx)
                     :: "eax", "ebx", "ecx", "edx");
    }

    res
}

fn as_bytes(v: &u32) -> &[u8] {
    let start = v as *const u32 as *const u8;
    unsafe { slice::from_raw_parts(start, 4) }
}

fn get_bits(r: u32, from: u32, to: u32) -> u32 {
    assert!(from <= 31);
    assert!(to <= 31);
    assert!(from <= to);

    let mask = match to {
        31 => 0xffffffff,
        _ => (1 << (to+1)) - 1,
    };

    (r & mask) >> from
}

macro_rules! check_flag {
    ($fun:ident, $flags:ident, $flag:ident) => (
        pub fn $fun(&self) -> bool {
            self.$flags.contains($flag)
        }
    )
}

#[derive(Debug)]
pub struct CpuId {
    max_eax_value: u32,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct CpuIdResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

const EAX_VENDOR_INFO: u32 = 0x0;
const EAX_FEATURE_INFO: u32 = 0x1;
const EAX_CACHE_INFO: u32 = 0x2;
const EAX_PROCESSOR_SERIAL: u32 = 0x3;
const EAX_CACHE_PARAMETERS: u32 = 0x4;
const EAX_MONITOR_MWAIT_INFO: u32 = 0x5;
const EAX_THERMAL_POWER_INFO: u32 = 0x6;
const EAX_STRUCTURED_EXTENDED_FEATURE_INFO: u32 = 0x7;
const EAX_DIRECT_CACHE_ACCESS_INFO: u32 = 0x9;
const EAX_PERFORMANCE_MONITOR_INFO: u32 = 0xA;
const EAX_EXTENDED_TOPOLOGY_INFO: u32 = 0xB;
const EAX_EXTENDED_STATE_INFO: u32 = 0xD;
const EAX_QOS_INFO: u32 = 0xF;
const EAX_EXTENDED_FUNCTION_INFO: u32 = 0x80000000;

impl CpuId {

    pub fn new() -> CpuId {
        let res = cpuid!(EAX_VENDOR_INFO);
        CpuId { max_eax_value: res.eax }
    }

    fn leaf_is_supported(&self, val: u32) -> bool {
        val <= self.max_eax_value
    }

    pub fn get_vendor_info(&self) -> Option<VendorInfo> {
        if self.leaf_is_supported(EAX_VENDOR_INFO) {
            let res = cpuid!(EAX_VENDOR_INFO);
            Some(VendorInfo { ebx: res.ebx, ecx: res.ecx, edx: res.edx })
        }
        else {
            None
        }
    }

    pub fn get_feature_info(&self) -> Option<FeatureInfo> {
        if self.leaf_is_supported(EAX_FEATURE_INFO) {
            let res = cpuid!(EAX_FEATURE_INFO);
            Some(FeatureInfo { eax: res.eax,
                               ebx: res.ebx,
                               ecx: FeatureInfoEcx { bits: res.ecx },
                               edx: FeatureInfoEdx { bits: res.edx },
            })
        }
        else {
            None
        }
    }

    pub fn get_cache_info(&self) -> Option<CacheInfoIter> {
        if self.leaf_is_supported(EAX_CACHE_INFO) {
            let res = cpuid!(EAX_CACHE_INFO);
            Some(CacheInfoIter { current: 1,
                            eax: res.eax,
                            ebx: res.ebx,
                            ecx: res.ecx,
                            edx: res.edx })
        }
        else {
            None
        }
    }

    pub fn get_processor_serial(&self) -> Option<ProcessorSerial> {
        if self.leaf_is_supported(EAX_PROCESSOR_SERIAL) {
            let res = cpuid!(EAX_PROCESSOR_SERIAL);
            Some(ProcessorSerial { ecx: res.ecx, edx: res.edx })
        }
        else {
            None
        }

    }

    pub fn get_cache_parameters(&self) -> Option<CacheParametersIter> {
        if self.leaf_is_supported(EAX_CACHE_PARAMETERS) {
            Some(CacheParametersIter { current: 0 })
        }
        else {
            None
        }
    }

    pub fn get_monitor_mwait_info(&self) -> Option<MonitorMwaitInfo> {
        if self.leaf_is_supported(EAX_MONITOR_MWAIT_INFO) {
            let res = cpuid!(EAX_MONITOR_MWAIT_INFO);
            Some(MonitorMwaitInfo { eax: res.eax, ebx: res.ebx, ecx: res.ecx, edx: res.edx })
        }
        else {
            None
        }
    }

    pub fn get_thermal_power_info(&self) -> Option<ThermalPowerInfo> {
        if self.leaf_is_supported(EAX_THERMAL_POWER_INFO) {
            let res = cpuid!(EAX_THERMAL_POWER_INFO);
            Some(ThermalPowerInfo { eax: ThermalPowerFeaturesEax { bits: res.eax },
                            ebx: res.ebx,
                            ecx: ThermalPowerFeaturesEcx { bits: res.ecx },
                            edx: res.edx })
        }
        else {
            None
        }
    }

    pub fn get_extended_feature_info(&self) -> Option<ExtendedFeatures> {
        if self.leaf_is_supported(EAX_STRUCTURED_EXTENDED_FEATURE_INFO) {
            let res = cpuid!(EAX_STRUCTURED_EXTENDED_FEATURE_INFO);
            assert!(res.eax == 0);
            Some(ExtendedFeatures { eax: res.eax,
                               ebx: ExtendedFeaturesEbx { bits: res.ebx },
                               ecx: res.ecx,
                               edx: res.edx })
        }
        else {
            None
        }

    }

    pub fn get_direct_cache_access_info(&self) -> Option<DirectCacheAccessInfo> {
        if self.leaf_is_supported(EAX_DIRECT_CACHE_ACCESS_INFO) {
            let res = cpuid!(EAX_DIRECT_CACHE_ACCESS_INFO);
            Some(DirectCacheAccessInfo{ eax: res.eax })
        }
        else {
            None
        }
    }

    pub fn get_performance_monitoring_info(&self) -> Option<PerformanceMonitoringInfo> {
        if self.leaf_is_supported(EAX_PERFORMANCE_MONITOR_INFO) {
            let res = cpuid!(EAX_PERFORMANCE_MONITOR_INFO);
            Some(PerformanceMonitoringInfo{ eax: res.eax,
                                            ebx: PerformanceMonitoringFeaturesEbx{ bits: res.ebx },
                                            ecx: res.ecx,
                                            edx: res.edx })
        }
        else {
            None
        }
    }

    pub fn get_extended_topology_info(&self) -> Option<ExtendedTopologyIter> {
        if self.leaf_is_supported(EAX_EXTENDED_TOPOLOGY_INFO) {
            Some(ExtendedTopologyIter { level: 0 })
        }
        else {
            None
        }
    }

    pub fn get_extended_state_info(&self) -> Option<ExtendedStateInfo> {
        if self.leaf_is_supported(EAX_EXTENDED_STATE_INFO) {
            let res = cpuid!(EAX_EXTENDED_STATE_INFO, 0);
            let res1 = cpuid!(EAX_EXTENDED_STATE_INFO, 1);
            Some(ExtendedStateInfo { eax: res.eax, ebx: res.ebx,
                                     ecx: res.ecx, edx: res.edx,
                                     eax1: res1.eax })
        }
        else {
            None
        }
    }

    pub fn get_quality_of_service_info(&self) -> Option<QoSInfo> {
        let res = cpuid!(EAX_QOS_INFO, 0);
        let res1 = cpuid!(EAX_QOS_INFO, 1);

        if self.leaf_is_supported(EAX_QOS_INFO) {
            Some(QoSInfo { ebx0: res.ebx, edx0: res.edx,
                           ebx1: res1.ebx, ecx1: res1.ecx,
                           edx1: res1.edx })
        }
        else {
            None
        }
    }

    pub fn get_extended_function_info(&self) -> Option<ExtendedFunctionInfo> {
        let res = cpuid!(EAX_EXTENDED_FUNCTION_INFO);

        if res.eax == 0 {
            return None;
        }

        let mut ef = ExtendedFunctionInfo { max_eax_value: res.eax - EAX_EXTENDED_FUNCTION_INFO,
                                            data: [
                CpuIdResult{eax: res.eax, ebx: res.ebx, ecx: res.ecx, edx: res.edx},
                CpuIdResult{eax: 0, ebx: 0, ecx: 0, edx: 0},
                CpuIdResult{eax: 0, ebx: 0, ecx: 0, edx: 0},
                CpuIdResult{eax: 0, ebx: 0, ecx: 0, edx: 0},
                CpuIdResult{eax: 0, ebx: 0, ecx: 0, edx: 0},
                CpuIdResult{eax: 0, ebx: 0, ecx: 0, edx: 0},
                CpuIdResult{eax: 0, ebx: 0, ecx: 0, edx: 0},
                CpuIdResult{eax: 0, ebx: 0, ecx: 0, edx: 0},
                CpuIdResult{eax: 0, ebx: 0, ecx: 0, edx: 0}
            ], };

        for i in 1..ef.max_eax_value+1 {
            ef.data[i as usize] = cpuid!(EAX_EXTENDED_FUNCTION_INFO + i);
        }

        Some(ef)
    }
}

#[derive(Debug)]
pub struct VendorInfo {
    ebx: u32,
    edx: u32,
    ecx: u32,
}

impl VendorInfo {
    pub fn as_string(&self) -> &str {
        unsafe {
            let brand_string_start = transmute::<&VendorInfo, *const u8>(&self);
            let slice = raw::Slice { data: brand_string_start, len: 3*4 };
            let byte_array: &'static [u8] = transmute(slice);
            str::from_utf8_unchecked(byte_array)
        }
    }
}

#[derive(Debug)]
pub struct CacheInfoIter {
    current: u32,
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl Iterator for CacheInfoIter {
    type Item = CacheInfo;

    fn next(&mut self) -> Option<CacheInfo> {
        if self.current >= 4*4 {
            return None;
        }
        let reg_index = self.current % 4;
        let byte_index = self.current / 4;

        let reg = match reg_index {
            0 => self.eax,
            1 => self.ebx,
            2 => self.ecx,
            3 => self.edx,
            _ => unreachable!()
        };

        let byte = as_bytes(&reg)[byte_index as usize];
        if byte == 0 {
            self.current += 1;
            return self.next();
        }

        for cache_info in CACHE_INFO_TABLE.into_iter() {
            if cache_info.num == byte {
                self.current += 1;
                return Some(*cache_info);
            }
        }

        None
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CacheInfoType {
    GENERAL,
    CACHE,
    TLB,
    STLB,
    DTLB,
    PREFETCH,
}

#[derive(Copy, Clone, Debug)]
pub struct CacheInfo {
    pub num: u8,
    pub typ: CacheInfoType,
    pub desc: &'static str,
}

impl fmt::Display for CacheInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typ = match self.typ {
            CacheInfoType::GENERAL => "N/A",
            CacheInfoType::CACHE => "Cache",
            CacheInfoType::TLB => "TLB",
            CacheInfoType::STLB => "STLB",
            CacheInfoType::DTLB => "DTLB",
            CacheInfoType::PREFETCH => "Prefetcher"
        };

        write!(f, "{:x}:\t {}: {}", self.num, typ, self.desc)
    }
}

pub const CACHE_INFO_TABLE: [CacheInfo; 103] = [
    CacheInfo{num: 0x00, typ: CacheInfoType::GENERAL, desc: "Null descriptor, this byte contains no information"},
    CacheInfo{num: 0x01, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4 KByte pages, 4-way set associative, 32 entries"},
    CacheInfo{num: 0x02, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4 MByte pages, fully associative, 2 entries"},
    CacheInfo{num: 0x03, typ: CacheInfoType::TLB, desc: "Data TLB: 4 KByte pages, 4-way set associative, 64 entries"},
    CacheInfo{num: 0x04, typ: CacheInfoType::TLB, desc: "Data TLB: 4 MByte pages, 4-way set associative, 8 entries"},
    CacheInfo{num: 0x05, typ: CacheInfoType::TLB, desc: "Data TLB1: 4 MByte pages, 4-way set associative, 32 entries"},
    CacheInfo{num: 0x06, typ: CacheInfoType::CACHE, desc: "1st-level instruction cache: 8 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x08, typ: CacheInfoType::CACHE, desc: "1st-level instruction cache: 16 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x09, typ: CacheInfoType::CACHE, desc: "1st-level instruction cache: 32KBytes, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x0A, typ: CacheInfoType::CACHE, desc: "1st-level data cache: 8 KBytes, 2-way set associative, 32 byte line size"},
    CacheInfo{num: 0x0B, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4 MByte pages, 4-way set associative, 4 entries"},
    CacheInfo{num: 0x0C, typ: CacheInfoType::CACHE, desc: "1st-level data cache: 16 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x0D, typ: CacheInfoType::CACHE, desc: "1st-level data cache: 16 KBytes, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x0E, typ: CacheInfoType::CACHE, desc: "1st-level data cache: 24 KBytes, 6-way set associative, 64 byte line size"},
    CacheInfo{num: 0x21, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 256 KBytes, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x22, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 512 KBytes, 4-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x23, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 1 MBytes, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x24, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 1 MBytes, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0x25, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 2 MBytes, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x29, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 4 MBytes, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x2C, typ: CacheInfoType::CACHE, desc: "1st-level data cache: 32 KBytes, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x30, typ: CacheInfoType::CACHE, desc: "1st-level instruction cache: 32 KBytes, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x40, typ: CacheInfoType::CACHE, desc: "No 2nd-level cache or, if processor contains a valid 2nd-level cache, no 3rd-level cache"},
    CacheInfo{num: 0x41, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 128 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x42, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 256 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x43, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 512 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x44, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 1 MByte, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x45, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 2 MByte, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x46, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 4 MByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x47, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 8 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x48, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 3MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0x49, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 4MB, 16-way set associative, 64-byte line size (Intel Xeon processor MP, Family 0FH, Model 06H); 2nd-level cache: 4 MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4A, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 6MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4B, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 8MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4C, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 12MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4D, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 16MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4E, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 6MByte, 24-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4F, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4 KByte pages, 32 entries"},
    CacheInfo{num: 0x50, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4 KByte and 2-MByte or 4-MByte pages, 64 entries"},
    CacheInfo{num: 0x51, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4 KByte and 2-MByte or 4-MByte pages, 128 entries"},
    CacheInfo{num: 0x52, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4 KByte and 2-MByte or 4-MByte pages, 256 entries"},
    CacheInfo{num: 0x55, typ: CacheInfoType::TLB, desc: "Instruction TLB: 2-MByte or 4-MByte pages, fully associative, 7 entries"},
    CacheInfo{num: 0x56, typ: CacheInfoType::TLB, desc: "Data TLB0: 4 MByte pages, 4-way set associative, 16 entries"},
    CacheInfo{num: 0x57, typ: CacheInfoType::TLB, desc: "Data TLB0: 4 KByte pages, 4-way associative, 16 entries"},
    CacheInfo{num: 0x59, typ: CacheInfoType::TLB, desc: "Data TLB0: 4 KByte pages, fully associative, 16 entries"},
    CacheInfo{num: 0x5A, typ: CacheInfoType::TLB, desc: "Data TLB0: 2-MByte or 4 MByte pages, 4-way set associative, 32 entries"},
    CacheInfo{num: 0x5B, typ: CacheInfoType::TLB, desc: "Data TLB: 4 KByte and 4 MByte pages, 64 entries"},
    CacheInfo{num: 0x5C, typ: CacheInfoType::TLB, desc: "Data TLB: 4 KByte and 4 MByte pages,128 entries"},
    CacheInfo{num: 0x5D, typ: CacheInfoType::TLB, desc: "Data TLB: 4 KByte and 4 MByte pages,256 entries"},
    CacheInfo{num: 0x60, typ: CacheInfoType::CACHE, desc: "1st-level data cache: 16 KByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x61, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4 KByte pages, fully associative, 48 entries"},
    CacheInfo{num: 0x63, typ: CacheInfoType::TLB, desc: "Data TLB: 1 GByte pages, 4-way set associative, 4 entries"},
    CacheInfo{num: 0x66, typ: CacheInfoType::CACHE, desc: "1st-level data cache: 8 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x67, typ: CacheInfoType::CACHE, desc: "1st-level data cache: 16 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x68, typ: CacheInfoType::CACHE, desc: "1st-level data cache: 32 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x70, typ: CacheInfoType::CACHE, desc: "Trace cache: 12 K-μop, 8-way set associative"},
    CacheInfo{num: 0x71, typ: CacheInfoType::CACHE, desc: "Trace cache: 16 K-μop, 8-way set associative"},
    CacheInfo{num: 0x72, typ: CacheInfoType::CACHE, desc: "Trace cache: 32 K-μop, 8-way set associative"},
    CacheInfo{num: 0x76, typ: CacheInfoType::TLB, desc: "Instruction TLB: 2M/4M pages, fully associative, 8 entries"},
    CacheInfo{num: 0x78, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 1 MByte, 4-way set associative, 64byte line size"},
    CacheInfo{num: 0x79, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 128 KByte, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x7A, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 256 KByte, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x7B, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 512 KByte, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x7C, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 1 MByte, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x7D, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 2 MByte, 8-way set associative, 64byte line size"},
    CacheInfo{num: 0x7F, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 512 KByte, 2-way set associative, 64-byte line size"},
    CacheInfo{num: 0x80, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 512 KByte, 8-way set associative, 64-byte line size"},
    CacheInfo{num: 0x82, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 256 KByte, 8-way set associative, 32 byte line size"},
    CacheInfo{num: 0x83, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 512 KByte, 8-way set associative, 32 byte line size"},
    CacheInfo{num: 0x84, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 1 MByte, 8-way set associative, 32 byte line size"},
    CacheInfo{num: 0x85, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 2 MByte, 8-way set associative, 32 byte line size"},
    CacheInfo{num: 0x86, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 512 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x87, typ: CacheInfoType::CACHE, desc: "2nd-level cache: 1 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0xB0, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4 KByte pages, 4-way set associative, 128 entries"},
    CacheInfo{num: 0xB1, typ: CacheInfoType::TLB, desc: "Instruction TLB: 2M pages, 4-way, 8 entries or 4M pages, 4-way, 4 entries"},
    CacheInfo{num: 0xB2, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4KByte pages, 4-way set associative, 64 entries"},
    CacheInfo{num: 0xB3, typ: CacheInfoType::TLB, desc: "Data TLB: 4 KByte pages, 4-way set associative, 128 entries"},
    CacheInfo{num: 0xB4, typ: CacheInfoType::TLB, desc: "Data TLB1: 4 KByte pages, 4-way associative, 256 entries"},
    CacheInfo{num: 0xB5, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4KByte pages, 8-way set associative, 64 entries"},
    CacheInfo{num: 0xB6, typ: CacheInfoType::TLB, desc: "Instruction TLB: 4KByte pages, 8-way set associative, 128 entries"},
    CacheInfo{num: 0xBA, typ: CacheInfoType::TLB, desc: "Data TLB1: 4 KByte pages, 4-way associative, 64 entries"},
    CacheInfo{num: 0xC0, typ: CacheInfoType::TLB, desc: "Data TLB: 4 KByte and 4 MByte pages, 4-way associative, 8 entries"},
    CacheInfo{num: 0xC1, typ: CacheInfoType::STLB, desc: "Shared 2nd-Level TLB: 4 KByte/2MByte pages, 8-way associative, 1024 entries"},
    CacheInfo{num: 0xC2, typ: CacheInfoType::DTLB, desc: "DTLB: 2 MByte/$MByte pages, 4-way associative, 16 entries"},
    CacheInfo{num: 0xCA, typ: CacheInfoType::STLB, desc: "Shared 2nd-Level TLB: 4 KByte pages, 4-way associative, 512 entries"},
    CacheInfo{num: 0xD0, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 512 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD1, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 1 MByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD2, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 2 MByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD6, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 1 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD7, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 2 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD8, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 4 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0xDC, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 1.5 MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0xDD, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 3 MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0xDE, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 6 MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0xE2, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 2 MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0xE3, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 4 MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0xE4, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 8 MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0xEA, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 12MByte, 24-way set associative, 64 byte line size"},
    CacheInfo{num: 0xEB, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 18MByte, 24-way set associative, 64 byte line size"},
    CacheInfo{num: 0xEC, typ: CacheInfoType::CACHE, desc: "3rd-level cache: 24MByte, 24-way set associative, 64 byte line size"},
    CacheInfo{num: 0xF0, typ: CacheInfoType::PREFETCH, desc: "64-Byte prefetching"},
    CacheInfo{num: 0xF1, typ: CacheInfoType::PREFETCH, desc: "128-Byte prefetching"},
    CacheInfo{num: 0xFF, typ: CacheInfoType::GENERAL, desc: "CPUID leaf 2 does not report cache descriptor information, use CPUID leaf 4 to query cache parameters"},
];

impl fmt::Display for VendorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

pub struct ProcessorSerial {
    ecx: u32,
    edx: u32,
}

impl ProcessorSerial {
    /// Bits 00-31 of 96 bit processor serial number.
    /// (Available in Pentium III processor only; otherwise, the value in this register is reserved.)
    pub fn serial_lower(&self) -> u32 {
        self.ecx
    }

    /// Bits 32-63 of 96 bit processor serial number.
    /// (Available in Pentium III processor only; otherwise, the value in this register is reserved.)
    pub fn serial_middle(&self) -> u32 {
        self.edx
    }
}

#[derive(Debug)]
pub struct FeatureInfo {
    eax: u32,
    ebx: u32,
    ecx: FeatureInfoEcx,
    edx: FeatureInfoEdx,
}

impl FeatureInfo {
    /// Streaming SIMD Extensions 3 (SSE3). A value of 1 indicates the processor supports this technology.
    check_flag!(has_sse3, ecx, CPU_FEATURE_SSE3);

    /// PCLMULQDQ. A value of 1 indicates the processor supports the PCLMULQDQ instruction
    check_flag!(has_pclmulqdq, ecx, CPU_FEATURE_PCLMULQDQ);

    /// 64-bit DS Area. A value of 1 indicates the processor supports DS area using 64-bit layout
    check_flag!(has_ds_area, ecx, CPU_FEATURE_DTES64);

    /// MONITOR/MWAIT. A value of 1 indicates the processor supports this feature.
    check_flag!(has_monitor_mwait, ecx, CPU_FEATURE_MONITOR);

    /// CPL Qualified Debug Store. A value of 1 indicates the processor supports the extensions to the  Debug Store feature to allow for branch message storage qualified by CPL.
    check_flag!(has_cpl, ecx, CPU_FEATURE_DSCPL);

    /// Virtual Machine Extensions. A value of 1 indicates that the processor supports this technology.
    check_flag!(has_vmx, ecx, CPU_FEATURE_VMX);

    /// Safer Mode Extensions. A value of 1 indicates that the processor supports this technology. See Chapter 5, Safer Mode Extensions Reference.
    check_flag!(has_smx, ecx, CPU_FEATURE_SMX);

    /// Enhanced Intel SpeedStep® technology. A value of 1 indicates that the processor supports this technology.
    check_flag!(has_eist, ecx, CPU_FEATURE_EIST);

    /// Thermal Monitor 2. A value of 1 indicates whether the processor supports this technology.
    check_flag!(has_tm2, ecx, CPU_FEATURE_TM2);

    /// A value of 1 indicates the presence of the Supplemental Streaming SIMD Extensions 3 (SSSE3). A value of 0 indicates the instruction extensions are not present in the processor
    check_flag!(has_ssse3, ecx, CPU_FEATURE_SSSE3);

    /// L1 Context ID. A value of 1 indicates the L1 data cache mode can be set to either adaptive mode or shared mode. A value of 0 indicates this feature is not supported. See definition of the IA32_MISC_ENABLE MSR Bit 24 (L1 Data Cache Context Mode) for details.
    check_flag!(has_cnxtid, ecx, CPU_FEATURE_CNXTID);

    /// A value of 1 indicates the processor supports FMA extensions using YMM state.
    check_flag!(has_fma, ecx, CPU_FEATURE_FMA);

    /// CMPXCHG16B Available. A value of 1 indicates that the feature is available. See the CMPXCHG8B/CMPXCHG16B Compare and Exchange Bytes section. 14
    check_flag!(has_cmpxchg16b, ecx, CPU_FEATURE_CMPXCHG16B);

    /// Perfmon and Debug Capability: A value of 1 indicates the processor supports the performance   and debug feature indication MSR IA32_PERF_CAPABILITIES.
    check_flag!(has_pdcm, ecx, CPU_FEATURE_PDCM);

    /// Process-context identifiers. A value of 1 indicates that the processor supports PCIDs and the software may set CR4.PCIDE to 1.
    check_flag!(has_pcid, ecx, CPU_FEATURE_PCID);

    /// A value of 1 indicates the processor supports the ability to prefetch data from a memory mapped device.
    check_flag!(has_dca, ecx, CPU_FEATURE_DCA);

    /// A value of 1 indicates that the processor supports SSE4.1.
    check_flag!(has_sse41, ecx, CPU_FEATURE_SSE41);

    /// A value of 1 indicates that the processor supports SSE4.2.
    check_flag!(has_sse42, ecx, CPU_FEATURE_SSE42);

    /// A value of 1 indicates that the processor supports x2APIC feature.
    check_flag!(has_x2apic, ecx, CPU_FEATURE_X2APIC);

    /// A value of 1 indicates that the processor supports MOVBE instruction.
    check_flag!(has_movbe, ecx, CPU_FEATURE_MOVBE);

    /// A value of 1 indicates that the processor supports the POPCNT instruction.
    check_flag!(has_popcnt, ecx, CPU_FEATURE_POPCNT);

    /// A value of 1 indicates that the processors local APIC timer supports one-shot operation using a TSC deadline value.
    check_flag!(has_tsc_deadline, ecx, CPU_FEATURE_TSC_DEADLINE);

    /// A value of 1 indicates that the processor supports the AESNI instruction extensions.
    check_flag!(has_aesni, ecx, CPU_FEATURE_AESNI);

    /// A value of 1 indicates that the processor supports the XSAVE/XRSTOR processor extended states feature, the XSETBV/XGETBV instructions, and XCR0.
    check_flag!(has_xsave, ecx, CPU_FEATURE_XSAVE);

    /// A value of 1 indicates that the OS has enabled XSETBV/XGETBV instructions to access XCR0, and support for processor extended state management using XSAVE/XRSTOR.
    check_flag!(has_oxsave, ecx, CPU_FEATURE_OSXSAVE);

    /// A value of 1 indicates the processor supports the AVX instruction extensions.
    check_flag!(has_avx, ecx, CPU_FEATURE_AVX);

    /// A value of 1 indicates that processor supports 16-bit floating-point conversion instructions.
    check_flag!(has_f16c, ecx, CPU_FEATURE_F16C);

    /// A value of 1 indicates that processor supports RDRAND instruction.
    check_flag!(has_rdrand, ecx, CPU_FEATURE_RDRAND);

    /// Floating Point Unit On-Chip. The processor contains an x87 FPU.
    check_flag!(has_fpu, edx, CPU_FEATURE_FPU);

    /// Virtual 8086 Mode Enhancements. Virtual 8086 mode enhancements, including CR4.VME for controlling the feature, CR4.PVI for protected mode virtual interrupts, software interrupt indirection, expansion of the TSS with the software indirection bitmap, and EFLAGS.VIF and EFLAGS.VIP flags.
    check_flag!(has_vme, edx, CPU_FEATURE_VME);

    /// Debugging Extensions. Support for I/O breakpoints, including CR4.DE for controlling the feature, and optional trapping of accesses to DR4 and DR5.
    check_flag!(has_de, edx, CPU_FEATURE_DE);

    /// Page Size Extension. Large pages of size 4 MByte are supported, including CR4.PSE for controlling the feature, the defined dirty bit in PDE (Page Directory Entries), optional reserved bit trapping in CR3, PDEs, and PTEs.
    check_flag!(has_pse, edx, CPU_FEATURE_PSE);

    /// Time Stamp Counter. The RDTSC instruction is supported, including CR4.TSD for controlling privilege.
    check_flag!(has_tsc, edx, CPU_FEATURE_TSC);

    /// Model Specific Registers RDMSR and WRMSR Instructions. The RDMSR and WRMSR instructions are supported. Some of the MSRs are implementation dependent.
    check_flag!(has_msr, edx, CPU_FEATURE_MSR);

    /// Physical Address Extension. Physical addresses greater than 32 bits are supported: extended page table entry formats, an extra level in the page translation tables is defined, 2-MByte pages are supported instead of 4 Mbyte pages if PAE bit is 1.
    check_flag!(has_pae, edx, CPU_FEATURE_PAE);

    /// Machine Check Exception. Exception 18 is defined for Machine Checks, including CR4.MCE for controlling the feature. This feature does not define the model-specific implementations of machine-check error logging, reporting, and processor shutdowns. Machine Check exception handlers may have to depend on processor version to do model specific processing of the exception, or test for the presence of the Machine Check feature.
    check_flag!(has_mce, edx, CPU_FEATURE_MCE);

    /// CMPXCHG8B Instruction. The compare-and-exchange 8 bytes (64 bits) instruction is supported (implicitly locked and atomic).
    check_flag!(has_cmpxchg8b, edx, CPU_FEATURE_CX8);

    /// APIC On-Chip. The processor contains an Advanced Programmable Interrupt Controller (APIC), responding to memory mapped commands in the physical address range FFFE0000H to FFFE0FFFH (by default - some processors permit the APIC to be relocated).
    check_flag!(has_apic, edx, CPU_FEATURE_APIC);

    /// SYSENTER and SYSEXIT Instructions. The SYSENTER and SYSEXIT and associated MSRs are supported.
    check_flag!(has_sysenter_sysexit, edx, CPU_FEATURE_SEP);

    /// Memory Type Range Registers. MTRRs are supported. The MTRRcap MSR contains feature bits that describe what memory types are supported, how many variable MTRRs are supported, and whether fixed MTRRs are supported.
    check_flag!(has_mtrr, edx, CPU_FEATURE_MTRR);

    /// Page Global Bit. The global bit is supported in paging-structure entries that map a page, indicating TLB entries that are common to different processes and need not be flushed. The CR4.PGE bit controls this feature.
    check_flag!(has_pge, edx, CPU_FEATURE_PGE);

    /// Machine Check Architecture. The Machine Check Architecture, which provides a compatible mechanism for error reporting in P6 family, Pentium 4, Intel Xeon processors, and future processors, is supported. The MCG_CAP MSR contains feature bits describing how many banks of error reporting MSRs are supported.
    check_flag!(has_mca, edx, CPU_FEATURE_MCA);

    /// Conditional Move Instructions. The conditional move instruction CMOV is supported. In addition, if x87 FPU is present as indicated by the CPUID.FPU feature bit, then the FCOMI and FCMOV instructions are supported
    check_flag!(has_cmov, edx, CPU_FEATURE_CMOV);

    /// Page Attribute Table. Page Attribute Table is supported. This feature augments the Memory Type Range Registers (MTRRs), allowing an operating system to specify attributes of memory accessed through a linear address on a 4KB granularity.
    check_flag!(has_pat, edx, CPU_FEATURE_PAT);

    /// 36-Bit Page Size Extension. 4-MByte pages addressing physical memory beyond 4 GBytes are supported with 32-bit paging. This feature indicates that upper bits of the physical address of a 4-MByte page are encoded in bits 20:13 of the page-directory entry. Such physical addresses are limited by MAXPHYADDR and may be up to 40 bits in size.
    check_flag!(has_pse36, edx, CPU_FEATURE_PSE36);

    /// Processor Serial Number. The processor supports the 96-bit processor identification number feature and the feature is enabled.
    check_flag!(has_psn, edx, CPU_FEATURE_PSN);

    /// CLFLUSH Instruction. CLFLUSH Instruction is supported.
    check_flag!(has_clflush, edx, CPU_FEATURE_CLFSH);

    /// Debug Store. The processor supports the ability to write debug information into a memory resident buffer. This feature is used by the branch trace store (BTS) and precise event-based sampling (PEBS) facilities (see Chapter 23, Introduction to Virtual-Machine Extensions, in the Intel® 64 and IA-32 Architectures Software Developers Manual, Volume 3C).
    check_flag!(has_ds, edx, CPU_FEATURE_DS);

    /// Thermal Monitor and Software Controlled Clock Facilities. The processor implements internal MSRs that allow processor temperature to be monitored and processor performance to be modulated in predefined duty cycles under software control.
    check_flag!(has_acpi, edx, CPU_FEATURE_ACPI);

    /// Intel MMX Technology. The processor supports the Intel MMX technology.
    check_flag!(has_mmx, edx, CPU_FEATURE_MMX);

    /// FXSAVE and FXRSTOR Instructions. The FXSAVE and FXRSTOR instructions are supported for fast save and restore of the floating point context. Presence of this bit also indicates that CR4.OSFXSR is available for an operating system to indicate that it supports the FXSAVE and FXRSTOR instructions.
    check_flag!(has_fxsave_fxstor, edx, CPU_FEATURE_FXSR);

    /// SSE. The processor supports the SSE extensions.
    check_flag!(has_sse, edx, CPU_FEATURE_SSE);

    /// SSE2. The processor supports the SSE2 extensions.
    check_flag!(has_sse2, edx, CPU_FEATURE_SSE2);

    /// Self Snoop. The processor supports the management of conflicting memory types by performing a snoop of its own cache structure for transactions issued to the bus.
    check_flag!(has_ss, edx, CPU_FEATURE_SS);

    /// Max APIC IDs reserved field is Valid. A value of 0 for HTT indicates there is only a single logical processor in the package and software should assume only a single APIC ID is reserved.  A value of 1 for HTT indicates the value in CPUID.1.EBX[23:16] (the Maximum number of addressable IDs for logical processors in this package) is valid for the package.
    check_flag!(has_htt, edx, CPU_FEATURE_HTT);

    /// Thermal Monitor. The processor implements the thermal monitor automatic thermal control circuitry (TCC).
    check_flag!(has_tm, edx, CPU_FEATURE_TM);

    /// Pending Break Enable. The processor supports the use of the FERR#/PBE# pin when the processor is in the stop-clock state (STPCLK# is asserted) to signal the processor that an interrupt is pending and that the processor should return to normal operation to handle the interrupt. Bit 10 (PBE enable) in the IA32_MISC_ENABLE MSR enables this capability.
    check_flag!(has_pbe, edx, CPU_FEATURE_PBE);



}

bitflags! {
    #[derive(Debug)]
    flags FeatureInfoEcx: u32 {
        /// Streaming SIMD Extensions 3 (SSE3). A value of 1 indicates the processor supports this technology.
        const CPU_FEATURE_SSE3 = 1 << 0,
        /// PCLMULQDQ. A value of 1 indicates the processor supports the PCLMULQDQ instruction
        const CPU_FEATURE_PCLMULQDQ = 1 << 1,
        /// 64-bit DS Area. A value of 1 indicates the processor supports DS area using 64-bit layout
        const CPU_FEATURE_DTES64 = 1 << 2,
        /// MONITOR/MWAIT. A value of 1 indicates the processor supports this feature.
        const CPU_FEATURE_MONITOR = 1 << 3,
        /// CPL Qualified Debug Store. A value of 1 indicates the processor supports the extensions to the  Debug Store feature to allow for branch message storage qualified by CPL.
        const CPU_FEATURE_DSCPL = 1 << 4,
        /// Virtual Machine Extensions. A value of 1 indicates that the processor supports this technology.
        const CPU_FEATURE_VMX = 1 << 5,
        /// Safer Mode Extensions. A value of 1 indicates that the processor supports this technology. See Chapter 5, Safer Mode Extensions Reference.
        const CPU_FEATURE_SMX = 1 << 6,
        /// Enhanced Intel SpeedStep® technology. A value of 1 indicates that the processor supports this technology.
        const CPU_FEATURE_EIST = 1 << 7,
        /// Thermal Monitor 2. A value of 1 indicates whether the processor supports this technology.
        const CPU_FEATURE_TM2 = 1 << 8,
        /// A value of 1 indicates the presence of the Supplemental Streaming SIMD Extensions 3 (SSSE3). A value of 0 indicates the instruction extensions are not present in the processor
        const CPU_FEATURE_SSSE3 = 1 << 9,
        /// L1 Context ID. A value of 1 indicates the L1 data cache mode can be set to either adaptive mode or shared mode. A value of 0 indicates this feature is not supported. See definition of the IA32_MISC_ENABLE MSR Bit 24 (L1 Data Cache Context Mode) for details.
        const CPU_FEATURE_CNXTID = 1 << 10,
        /// A value of 1 indicates the processor supports FMA extensions using YMM state.
        const CPU_FEATURE_FMA = 1 << 12,
        /// CMPXCHG16B Available. A value of 1 indicates that the feature is available. See the CMPXCHG8B/CMPXCHG16B Compare and Exchange Bytes section. 14
        const CPU_FEATURE_CMPXCHG16B = 1 << 13,
        /// Perfmon and Debug Capability: A value of 1 indicates the processor supports the performance   and debug feature indication MSR IA32_PERF_CAPABILITIES.
        const CPU_FEATURE_PDCM = 1 << 15,
        /// Process-context identifiers. A value of 1 indicates that the processor supports PCIDs and the software may set CR4.PCIDE to 1.
        const CPU_FEATURE_PCID = 1 << 17,
        /// A value of 1 indicates the processor supports the ability to prefetch data from a memory mapped device.
        const CPU_FEATURE_DCA = 1 << 18,
        /// A value of 1 indicates that the processor supports SSE4.1.
        const CPU_FEATURE_SSE41 = 1 << 19,
        /// A value of 1 indicates that the processor supports SSE4.2.
        const CPU_FEATURE_SSE42 = 1 << 20,
        /// A value of 1 indicates that the processor supports x2APIC feature.
        const CPU_FEATURE_X2APIC = 1 << 21,
        /// A value of 1 indicates that the processor supports MOVBE instruction.
        const CPU_FEATURE_MOVBE = 1 << 22,
        /// A value of 1 indicates that the processor supports the POPCNT instruction.
        const CPU_FEATURE_POPCNT = 1 << 23,
        /// A value of 1 indicates that the processors local APIC timer supports one-shot operation using a TSC deadline value.
        const CPU_FEATURE_TSC_DEADLINE = 1 << 24,
        /// A value of 1 indicates that the processor supports the AESNI instruction extensions.
        const CPU_FEATURE_AESNI = 1 << 25,
        /// A value of 1 indicates that the processor supports the XSAVE/XRSTOR processor extended states feature, the XSETBV/XGETBV instructions, and XCR0.
        const CPU_FEATURE_XSAVE = 1 << 26,
        /// A value of 1 indicates that the OS has enabled XSETBV/XGETBV instructions to access XCR0, and support for processor extended state management using XSAVE/XRSTOR.
        const CPU_FEATURE_OSXSAVE = 1 << 27,
        /// A value of 1 indicates the processor supports the AVX instruction extensions.
        const CPU_FEATURE_AVX = 1 << 28,
        /// A value of 1 indicates that processor supports 16-bit floating-point conversion instructions.
        const CPU_FEATURE_F16C = 1 << 29,
        /// A value of 1 indicates that processor supports RDRAND instruction.
        const CPU_FEATURE_RDRAND = 1 << 30,
    }
}


bitflags! {
    #[derive(Debug)]
    flags FeatureInfoEdx: u32 {
        /// Floating Point Unit On-Chip. The processor contains an x87 FPU.
        const CPU_FEATURE_FPU = 1 << 0,
        /// Virtual 8086 Mode Enhancements. Virtual 8086 mode enhancements, including CR4.VME for controlling the feature, CR4.PVI for protected mode virtual interrupts, software interrupt indirection, expansion of the TSS with the software indirection bitmap, and EFLAGS.VIF and EFLAGS.VIP flags.
        const CPU_FEATURE_VME = 1 << 1,
        /// Debugging Extensions. Support for I/O breakpoints, including CR4.DE for controlling the feature, and optional trapping of accesses to DR4 and DR5.
        const CPU_FEATURE_DE = 1 << 2,
        /// Page Size Extension. Large pages of size 4 MByte are supported, including CR4.PSE for controlling the feature, the defined dirty bit in PDE (Page Directory Entries), optional reserved bit trapping in CR3, PDEs, and PTEs.
        const CPU_FEATURE_PSE = 1 << 3,
        /// Time Stamp Counter. The RDTSC instruction is supported, including CR4.TSD for controlling privilege.
        const CPU_FEATURE_TSC = 1 << 4,
        /// Model Specific Registers RDMSR and WRMSR Instructions. The RDMSR and WRMSR instructions are supported. Some of the MSRs are implementation dependent.
        const CPU_FEATURE_MSR = 1 << 5,
        /// Physical Address Extension. Physical addresses greater than 32 bits are supported: extended page table entry formats, an extra level in the page translation tables is defined, 2-MByte pages are supported instead of 4 Mbyte pages if PAE bit is 1.
        const CPU_FEATURE_PAE = 1 << 6,
        /// Machine Check Exception. Exception 18 is defined for Machine Checks, including CR4.MCE for controlling the feature. This feature does not define the model-specific implementations of machine-check error logging, reporting, and processor shutdowns. Machine Check exception handlers may have to depend on processor version to do model specific processing of the exception, or test for the presence of the Machine Check feature.
        const CPU_FEATURE_MCE = 1 << 7,
        /// CMPXCHG8B Instruction. The compare-and-exchange 8 bytes (64 bits) instruction is supported (implicitly locked and atomic).
        const CPU_FEATURE_CX8 = 1 << 8,
        /// APIC On-Chip. The processor contains an Advanced Programmable Interrupt Controller (APIC), responding to memory mapped commands in the physical address range FFFE0000H to FFFE0FFFH (by default - some processors permit the APIC to be relocated).
        const CPU_FEATURE_APIC = 1 << 9,
        /// SYSENTER and SYSEXIT Instructions. The SYSENTER and SYSEXIT and associated MSRs are supported.
        const CPU_FEATURE_SEP = 1 << 11,
        /// Memory Type Range Registers. MTRRs are supported. The MTRRcap MSR contains feature bits that describe what memory types are supported, how many variable MTRRs are supported, and whether fixed MTRRs are supported.
        const CPU_FEATURE_MTRR = 1 << 12,
        /// Page Global Bit. The global bit is supported in paging-structure entries that map a page, indicating TLB entries that are common to different processes and need not be flushed. The CR4.PGE bit controls this feature.
        const CPU_FEATURE_PGE = 1 << 13,
        /// Machine Check Architecture. The Machine Check Architecture, which provides a compatible mechanism for error reporting in P6 family, Pentium 4, Intel Xeon processors, and future processors, is supported. The MCG_CAP MSR contains feature bits describing how many banks of error reporting MSRs are supported.
        const CPU_FEATURE_MCA = 1 << 14,
        /// Conditional Move Instructions. The conditional move instruction CMOV is supported. In addition, if x87 FPU is present as indicated by the CPUID.FPU feature bit, then the FCOMI and FCMOV instructions are supported
        const CPU_FEATURE_CMOV = 1 << 15,
        /// Page Attribute Table. Page Attribute Table is supported. This feature augments the Memory Type Range Registers (MTRRs), allowing an operating system to specify attributes of memory accessed through a linear address on a 4KB granularity.
        const CPU_FEATURE_PAT = 1 << 16,
        /// 36-Bit Page Size Extension. 4-MByte pages addressing physical memory beyond 4 GBytes are supported with 32-bit paging. This feature indicates that upper bits of the physical address of a 4-MByte page are encoded in bits 20:13 of the page-directory entry. Such physical addresses are limited by MAXPHYADDR and may be up to 40 bits in size.
        const CPU_FEATURE_PSE36 = 1 << 17,
        /// Processor Serial Number. The processor supports the 96-bit processor identification number feature and the feature is enabled.
        const CPU_FEATURE_PSN = 1 << 18,
        /// CLFLUSH Instruction. CLFLUSH Instruction is supported.
        const CPU_FEATURE_CLFSH = 1 << 19,
        /// Debug Store. The processor supports the ability to write debug information into a memory resident buffer. This feature is used by the branch trace store (BTS) and precise event-based sampling (PEBS) facilities (see Chapter 23, Introduction to Virtual-Machine Extensions, in the Intel® 64 and IA-32 Architectures Software Developers Manual, Volume 3C).
        const CPU_FEATURE_DS = 1 << 21,
        /// Thermal Monitor and Software Controlled Clock Facilities. The processor implements internal MSRs that allow processor temperature to be monitored and processor performance to be modulated in predefined duty cycles under software control.
        const CPU_FEATURE_ACPI = 1 << 22,
        /// Intel MMX Technology. The processor supports the Intel MMX technology.
        const CPU_FEATURE_MMX = 1 << 23,
        /// FXSAVE and FXRSTOR Instructions. The FXSAVE and FXRSTOR instructions are supported for fast save and restore of the floating point context. Presence of this bit also indicates that CR4.OSFXSR is available for an operating system to indicate that it supports the FXSAVE and FXRSTOR instructions.
        const CPU_FEATURE_FXSR = 1 << 24,
        /// SSE. The processor supports the SSE extensions.
        const CPU_FEATURE_SSE = 1 << 25,
        /// SSE2. The processor supports the SSE2 extensions.
        const CPU_FEATURE_SSE2 = 1 << 26,
        /// Self Snoop. The processor supports the management of conflicting memory types by performing a snoop of its own cache structure for transactions issued to the bus.
        const CPU_FEATURE_SS = 1 << 27,
        /// Max APIC IDs reserved field is Valid. A value of 0 for HTT indicates there is only a single logical processor in the package and software should assume only a single APIC ID is reserved.  A value of 1 for HTT indicates the value in CPUID.1.EBX[23:16] (the Maximum number of addressable IDs for logical processors in this package) is valid for the package.
        const CPU_FEATURE_HTT = 1 << 28,
        /// Thermal Monitor. The processor implements the thermal monitor automatic thermal control circuitry (TCC).
        const CPU_FEATURE_TM = 1 << 29,
        /// Pending Break Enable. The processor supports the use of the FERR#/PBE# pin when the processor is in the stop-clock state (STPCLK# is asserted) to signal the processor that an interrupt is pending and that the processor should return to normal operation to handle the interrupt. Bit 10 (PBE enable) in the IA32_MISC_ENABLE MSR enables this capability.
        const CPU_FEATURE_PBE = 1 << 31,
    }
}



impl FeatureInfo {

    /// Version Information: Extended Family
    pub fn extended_family_id(&self) -> u8 {
        get_bits(self.eax, 20, 27) as u8
    }

    /// Version Information: Extended Model
    pub fn extended_model_id(&self) -> u8 {
        get_bits(self.eax, 16, 19) as u8
    }

    /// Version Information: Family
    pub fn family_id(&self) -> u8 {
        get_bits(self.eax, 8, 11) as u8
    }

    /// Version Information: Model
    pub fn model_id(&self) -> u8 {
        get_bits(self.eax, 4, 7) as u8
    }

    /// Version Information: Stepping ID
    pub fn stepping_id(&self) -> u8 {
        get_bits(self.eax, 0, 3) as u8
    }

    /// Brand Index
    pub fn brand_index(&self) -> u8 {
        get_bits(self.ebx, 0, 7) as u8
    }

    /// CLFLUSH line size (Value ∗ 8 = cache line size in bytes)
    pub fn cflush_cache_line_size(&self) -> u8 {
        get_bits(self.ebx, 8, 15) as u8
    }

    /// Initial APIC ID
    pub fn initial_local_apic_id(&self) -> u8 {
        get_bits(self.ebx, 24, 31) as u8
    }

    /// Maximum number of addressable IDs for logical processors in this physical package.
    pub fn max_logical_processor_ids(&self) -> u8 {
        get_bits(self.ebx, 16, 23) as u8
    }
}

pub struct CacheParametersIter {
    current: u32,
}

impl Iterator for CacheParametersIter {
    type Item = CacheParameter;

    fn next(&mut self) -> Option<CacheParameter> {
        let res = cpuid!(EAX_CACHE_PARAMETERS, self.current);
        let cp = CacheParameter { eax: res.eax, ebx: res.ebx, ecx: res.ecx, edx: res.edx };

        match cp.cache_type() {
            CacheType::NULL => None,
            CacheType::RESERVED => None,
            _ => {
                self.current += 1;
                Some(cp)
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CacheParameter {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

#[derive(PartialEq, Eq)]
pub enum CacheType {
    /// Null - No more caches
    NULL = 0,
    DATA,
    INSTRUCTION,
    UNIFIED,
    /// 4-31 = Reserved
    RESERVED,
}

impl CacheParameter {

    /// Cache Type
    pub fn cache_type(&self) -> CacheType {
        let typ = get_bits(self.eax, 0, 4) as u8;
        match typ {
            0 => CacheType::NULL,
            1 => CacheType::DATA,
            2 => CacheType::INSTRUCTION,
            3 => CacheType::UNIFIED,
            _ => CacheType::RESERVED
        }
    }

    /// Cache Level (starts at 1)
    pub fn level(&self) -> u8 {
        get_bits(self.eax, 5, 7) as u8
    }

    /// Self Initializing cache level (does not need SW initialization).
    pub fn is_self_initializing(&self) -> bool {
        get_bits(self.eax, 8, 8) == 1
    }

    /// Fully Associative cache
    pub fn is_fully_associative(&self) -> bool {
        get_bits(self.eax, 9, 9) == 1
    }

    /// Maximum number of addressable IDs for logical processors sharing this cache
    pub fn max_cores_for_cache(&self) -> usize {
        (get_bits(self.eax, 14, 25) + 1) as usize
    }

    /// Maximum number of addressable IDs for processor cores in the physical package
    pub fn max_cores_for_package(&self) -> usize {
        (get_bits(self.eax, 26, 31) + 1) as usize
    }

    /// System Coherency Line Size (Bits 11-00)
    pub fn coherency_line_size(&self) -> usize {
        (get_bits(self.ebx, 0, 11) + 1) as usize
    }

    /// Physical Line partitions (Bits 21-12)
    pub fn physical_line_partitions(&self) -> usize {
        (get_bits(self.ebx, 12, 21) + 1) as usize
    }

    /// Ways of associativity (Bits 31-22)
    pub fn associativity(&self) -> usize {
        (get_bits(self.ebx, 22, 31) + 1) as usize
    }

    /// Number of Sets (Bits 31-00)
    pub fn sets(&self) -> usize {
        (self.ecx + 1) as usize
    }

    /// Write-Back Invalidate/Invalidate (Bit 0)
    /// False: WBINVD/INVD from threads sharing this cache acts upon lower level caches for threads sharing this cache.
    /// True: WBINVD/INVD is not guaranteed to act upon lower level caches of non-originating threads sharing this cache.
    pub fn is_write_back_invalidate(&self) -> bool {
        get_bits(self.edx, 0, 0) == 1
    }

    /// Cache Inclusiveness (Bit 1)
    /// False: Cache is not inclusive of lower cache levels.
    /// True: Cache is inclusive of lower cache levels.
    pub fn is_inclusive(&self) -> bool {
        get_bits(self.edx, 1, 1) == 1
    }

    /// Complex Cache Indexing (Bit 2)
    /// False: Direct mapped cache.
    /// True: A complex function is used to index the cache, potentially using all address bits.
    pub fn has_complex_indexing(&self) -> bool {
        get_bits(self.edx, 2, 2) == 1
    }
}

#[derive(Debug)]
pub struct MonitorMwaitInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl MonitorMwaitInfo {

    /// Smallest monitor-line size in bytes (default is processor's monitor granularity)
    pub fn smallest_monitor_line(&self) -> u16 {
        get_bits(self.eax, 0, 15) as u16
    }

    /// Largest monitor-line size in bytes (default is processor's monitor granularity
    pub fn largest_monitor_line(&self) -> u16 {
        get_bits(self.ebx, 0, 15) as u16
    }

    ///  Enumeration of Monitor-Mwait extensions (beyond EAX and EBX registers) supported
    pub fn extensions_supported(&self) -> bool {
        get_bits(self.ecx, 0, 0) == 1
    }

    ///  Supports treating interrupts as break-event for MWAIT, even when interrupts disabled
    pub fn interrupts_as_break_event(&self) -> bool {
        get_bits(self.ecx, 1, 1) == 1
    }

    /// Number of C0 sub C-states supported using MWAIT (Bits 03 - 00)
    pub fn supported_c0_states(&self) -> u16 {
        get_bits(self.edx, 0, 3) as u16
    }

    /// Number of C1 sub C-states supported using MWAIT (Bits 07 - 04)
    pub fn supported_c1_states(&self) -> u16 {
        get_bits(self.edx, 4, 7) as u16
    }

    /// Number of C2 sub C-states supported using MWAIT (Bits 11 - 08)
    pub fn supported_c2_states(&self) -> u16 {
        get_bits(self.edx, 8, 11) as u16
    }

    /// Number of C3 sub C-states supported using MWAIT (Bits 15 - 12)
    pub fn supported_c3_states(&self) -> u16 {
        get_bits(self.edx, 12, 15) as u16
    }

    /// Number of C4 sub C-states supported using MWAIT (Bits 19 - 16)
    pub fn supported_c4_states(&self) -> u16 {
        get_bits(self.edx, 16, 19) as u16
    }

    /// Number of C5 sub C-states supported using MWAIT (Bits 23 - 20)
    pub fn supported_c5_states(&self) -> u16 {
        get_bits(self.edx, 20, 23) as u16
    }

    /// Number of C6 sub C-states supported using MWAIT (Bits 27 - 24)
    pub fn supported_c6_states(&self) -> u16 {
        get_bits(self.edx, 24, 27) as u16
    }

    /// Number of C7 sub C-states supported using MWAIT (Bits 31 - 28)
    pub fn supported_c7_states(&self) -> u16 {
        get_bits(self.edx, 28, 31) as u16
    }
}

#[derive(Debug)]
pub struct ThermalPowerInfo {
    eax: ThermalPowerFeaturesEax,
    ebx: u32,
    ecx: ThermalPowerFeaturesEcx,
    edx: u32,
}

impl ThermalPowerInfo {
    /// Digital temperature sensor is supported if set.
    check_flag!(has_dts, eax, CPU_FEATURE_DTS);

    /// Intel Turbo Boost Technology Available (see description of IA32_MISC_ENABLE[38]).
    check_flag!(has_turbo_boost, eax, CPU_FEATURE_TURBO_BOOST);

    /// ARAT. APIC-Timer-always-running feature is supported if set.
    check_flag!(has_arat, eax, CPU_FEATURE_ARAT);

    /// PLN. Power limit notification controls are supported if set.
    check_flag!(has_pln, eax, CPU_FEATURE_PLN);

    /// ECMD. Clock modulation duty cycle extension is supported if set.
    check_flag!(has_ecmd, eax, CPU_FEATURE_ECMD);

    /// PTM. Package thermal management is supported if set.
    check_flag!(has_ptm, eax, CPU_FEATURE_PTM);

    /// Hardware Coordination Feedback Capability (Presence of IA32_MPERF
    /// and IA32_APERF). The capability to provide a measure of
    /// delivered processor performance (since last reset of the counters),
    /// as a percentage of expected processor performance at frequency specified
    /// in CPUID Brand String Bits 02 - 01
    check_flag!(has_hw_coord_feedback, ecx, CPU_FEATURE_HW_COORD_FEEDBACK);

    /// The processor supports performance-energy bias preference
    /// if CPUID.06H:ECX.SETBH[bit 3] is set and it also implies the
    /// presence of a new architectural MSR called IA32_ENERGY_PERF_BIAS (1B0H)
    check_flag!(has_energy_bias_pref, ecx, CPU_FEATURE_ENERGY_BIAS_PREF);
}

bitflags! {
    #[derive(Debug)]
    flags ThermalPowerFeaturesEax: u32 {

        /// Digital temperature sensor is supported if set. (Bit 00)
        const CPU_FEATURE_DTS = 1 << 0,

        /// Intel Turbo Boost Technology Available (see description of IA32_MISC_ENABLE[38]). (Bit 01)
        const CPU_FEATURE_TURBO_BOOST = 1 << 1,

        /// ARAT. APIC-Timer-always-running feature is supported if set. (Bit 02)
        const CPU_FEATURE_ARAT = 1 << 2,

        /// PLN. Power limit notification controls are supported if set. (Bit 04)
        const CPU_FEATURE_PLN = 1 << 4,

        /// ECMD. Clock modulation duty cycle extension is supported if set. (Bit 05)
        const CPU_FEATURE_ECMD = 1 << 5,

        /// PTM. Package thermal management is supported if set. (Bit 06)
        const CPU_FEATURE_PTM = 1 << 6,
    }
}

bitflags! {
    #[derive(Debug)]
    flags ThermalPowerFeaturesEcx: u32 {
        /// Hardware Coordination Feedback Capability (Presence of IA32_MPERF and IA32_APERF). The capability to provide a measure of delivered processor performance (since last reset of the counters), as a percentage of expected processor performance at frequency specified in CPUID Brand String Bits 02 - 01
        const CPU_FEATURE_HW_COORD_FEEDBACK = 1 << 0,

        /// The processor supports performance-energy bias preference if CPUID.06H:ECX.SETBH[bit 3] is set and it also implies the presence of a new architectural MSR called IA32_ENERGY_PERF_BIAS (1B0H)
        const CPU_FEATURE_ENERGY_BIAS_PREF = 1 << 3,
    }
}

impl ThermalPowerInfo {

    /// Number of Interrupt Thresholds in Digital Thermal Sensor
    pub fn dts_irq_threshold(&self) -> u8 {
        get_bits(self.ebx, 0, 3) as u8
    }

}

#[derive(Debug)]
pub struct ExtendedFeatures {
    eax: u32,
    ebx: ExtendedFeaturesEbx,
    ecx: u32,
    edx: u32,
}

impl ExtendedFeatures {
    /// FSGSBASE. Supports RDFSBASE/RDGSBASE/WRFSBASE/WRGSBASE if 1.
    check_flag!(has_fsgsbase, ebx, CPU_FEATURE_FSGSBASE);

    /// IA32_TSC_ADJUST MSR is supported if 1.
    check_flag!(has_tsc_adjust_msr, ebx, CPU_FEATURE_ADJUST_MSR);

    /// BMI1
    check_flag!(has_bmi1, ebx, CPU_FEATURE_BMI1);

    /// HLE
    check_flag!(has_hle, ebx, CPU_FEATURE_HLE);

    /// AVX2
    check_flag!(has_avx2, ebx, CPU_FEATURE_AVX2);

    /// SMEP. Supports Supervisor-Mode Execution Prevention if 1.
    check_flag!(has_smep, ebx, CPU_FEATURE_SMEP);

    /// BMI2
    check_flag!(has_bmi2, ebx, CPU_FEATURE_BMI2);

    /// Supports Enhanced REP MOVSB/STOSB if 1.
    check_flag!(has_rep_movsb_stosb, ebx, CPU_FEATURE_REP_MOVSB_STOSB);

    /// INVPCID. If 1, supports INVPCID instruction for system software that manages process-context identifiers.
    check_flag!(has_invpcid, ebx, CPU_FEATURE_INVPCID);

    /// RTM
    check_flag!(has_rtm, ebx, CPU_FEATURE_RTM);

    /// Supports Quality of Service Monitoring (QM) capability if 1.
    check_flag!(has_qm, ebx, CPU_FEATURE_QM);

    /// Deprecates FPU CS and FPU DS values if 1.
    check_flag!(has_fpu_cs_ds_deprecated, ebx, CPU_FEATURE_DEPRECATE_FPU_CS_DS);
}


bitflags! {
    #[derive(Debug)]
    flags ExtendedFeaturesEbx: u32 {

        /// FSGSBASE. Supports RDFSBASE/RDGSBASE/WRFSBASE/WRGSBASE if 1. (Bit 00)
        const CPU_FEATURE_FSGSBASE = 1 << 0,

        /// IA32_TSC_ADJUST MSR is supported if 1. (Bit 01)
        const CPU_FEATURE_ADJUST_MSR = 1 << 1,

        /// BMI1 (Bit 03)
        const CPU_FEATURE_BMI1 = 1 << 3,

        /// HLE (Bit 04)
        const CPU_FEATURE_HLE = 1 << 4,

        /// AVX2 (Bit 05)
        const CPU_FEATURE_AVX2 = 1 << 5,

        /// SMEP. Supports Supervisor-Mode Execution Prevention if 1. (Bit 07)
        const CPU_FEATURE_SMEP = 1 << 7,

        /// BMI2 (Bit 08)
        const CPU_FEATURE_BMI2 = 1 << 8,

        /// Supports Enhanced REP MOVSB/STOSB if 1. (Bit 09)
        const CPU_FEATURE_REP_MOVSB_STOSB = 1 << 9,

        /// INVPCID. If 1, supports INVPCID instruction for system software that manages process-context identifiers. (Bit 10)
        const CPU_FEATURE_INVPCID = 1 << 10,

        /// RTM (Bit 11)
        const CPU_FEATURE_RTM = 1 << 11,

        /// Supports Quality of Service Monitoring (QM) capability if 1. (Bit 12)
        const CPU_FEATURE_QM = 1 << 12,

        /// Deprecates FPU CS and FPU DS values if 1. (Bit 13)
        const CPU_FEATURE_DEPRECATE_FPU_CS_DS = 1 << 13,

    }
}

#[derive(Debug)]
pub struct DirectCacheAccessInfo {
    eax: u32,
}

impl DirectCacheAccessInfo {

    /// Value of bits [31:0] of IA32_PLATFORM_DCA_CAP MSR (address 1F8H)
    pub fn get_dca_cap_value(&self) -> u32 {
        self.eax
    }
}


#[derive(Debug)]
pub struct PerformanceMonitoringInfo {
    eax: u32,
    ebx: PerformanceMonitoringFeaturesEbx,
    ecx: u32,
    edx: u32,
}

impl PerformanceMonitoringInfo {

    /// Version ID of architectural performance monitoring. (Bits 07 - 00)
    pub fn version_id(&self) -> u8 {
        get_bits(self.eax, 0, 7) as u8
    }

    /// Number of general-purpose performance monitoring counter per logical processor. (Bits 15- 08)
    pub fn number_of_counters(&self) -> u8 {
        get_bits(self.eax, 8, 15) as u8
    }

    /// Bit width of general-purpose, performance monitoring counter. (Bits 23 - 16)
    pub fn counter_bit_width(&self) -> u8 {
        get_bits(self.eax, 16, 23) as u8
    }

    /// Length of EBX bit vector to enumerate architectural performance monitoring events. (Bits 31 - 24)
    pub fn ebx_length(&self) -> u8 {
        get_bits(self.eax, 24, 31) as u8
    }

    /// Number of fixed-function performance counters (if Version ID > 1). (Bits 04 - 00)
    pub fn fixed_function_counters(&self) -> u8 {
        get_bits(self.edx, 0, 4) as u8
    }

    /// Bit width of fixed-function performance counters (if Version ID > 1). (Bits 12- 05)
    pub fn fixed_function_counters_bit_width(&self) -> u8 {
        get_bits(self.edx, 5, 12) as u8
    }

    /// Core cycle event not available if 1.
    check_flag!(is_core_cyc_ev_unavailable, ebx, CPU_FEATURE_CORE_CYC_EV_UNAVAILABLE);

    /// Instruction retired event not available if 1.
    check_flag!(is_inst_ret_ev_unavailable, ebx, CPU_FEATURE_INST_RET_EV_UNAVAILABLE);

    /// Reference cycles event not available if 1.
    check_flag!(is_ref_cycle_ev_unavailable, ebx, CPU_FEATURE_REF_CYC_EV_UNAVAILABLE);

    /// Last-level cache reference event not available if 1.
    check_flag!(is_cache_ref_ev_unavailable, ebx, CPU_FEATURE_CACHE_REF_EV_UNAVAILABLE);

    /// Last-level cache misses event not available if 1.
    check_flag!(is_ll_cache_miss_ev_unavailable, ebx, CPU_FEATURE_LL_CACHE_MISS_EV_UNAVAILABLE);

    /// Branch instruction retired event not available if 1.
    check_flag!(is_branch_inst_ret_ev_unavailable, ebx, CPU_FEATURE_BRANCH_INST_RET_EV_UNAVAILABLE);

    /// Branch mispredict retired event not available if 1.
    check_flag!(is_branch_midpred_ev_unavailable, ebx, CPU_FEATURE_BRANCH_MISPRED_EV_UNAVAILABLE);
}

bitflags! {
    #[derive(Debug)]
    flags PerformanceMonitoringFeaturesEbx: u32 {
        /// Core cycle event not available if 1. (Bit 0)
        const CPU_FEATURE_CORE_CYC_EV_UNAVAILABLE = 1 << 0,

        /// Instruction retired event not available if 1. (Bit 01)
        const CPU_FEATURE_INST_RET_EV_UNAVAILABLE = 1 << 1,

        /// Reference cycles event not available if 1. (Bit 02)
        const CPU_FEATURE_REF_CYC_EV_UNAVAILABLE = 1 << 2,

        /// Last-level cache reference event not available if 1. (Bit 03)
        const CPU_FEATURE_CACHE_REF_EV_UNAVAILABLE = 1 << 3,

        /// Last-level cache misses event not available if 1. (Bit 04)
        const CPU_FEATURE_LL_CACHE_MISS_EV_UNAVAILABLE = 1 << 4,

        /// Branch instruction retired event not available if 1. (Bit 05)
        const CPU_FEATURE_BRANCH_INST_RET_EV_UNAVAILABLE = 1 << 5,

        /// Branch mispredict retired event not available if 1. (Bit 06)
        const CPU_FEATURE_BRANCH_MISPRED_EV_UNAVAILABLE = 1 << 6,
    }
}

#[derive(Debug)]
pub struct ExtendedTopologyIter {
    level: u32,
}

#[derive(Debug)]
pub struct ExtendedTopologyLevel {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl ExtendedTopologyLevel {

    /// Number of logical processors at this level type.
    /// The number reflects configuration as shipped.
    pub fn processors(&self) -> u16 {
        get_bits(self.ebx, 0, 15) as u16
    }

    /// Level number.
    pub fn level_number(&self) -> u8 {
        get_bits(self.ecx, 0, 7) as u8
    }

    // Level type.
    pub fn level_type(&self) -> TopologyType {
        match get_bits(self.ecx, 8, 15) {
            0 => TopologyType::INVALID,
            1 => TopologyType::SMT,
            2 => TopologyType::CORE,
            _ => unreachable!()
        }
    }

    /// x2APIC ID the current logical processor. (Bits 31-00)
    pub fn x2apic_id(&self) -> u32 {
        self.edx
    }

    /// Number of bits to shift right on x2APIC ID to get a unique topology ID of the next level type. (Bits 04-00)
    /// All logical processors with the same next level ID share current level.
    pub fn shift_right_for_next_apic_id(&self) -> u32 {
        get_bits(self.eax, 0, 4)
    }

}

#[derive(PartialEq, Eq, Debug)]
pub enum TopologyType {
    INVALID = 0,
    SMT = 1,
    CORE = 2,
}

impl Iterator for ExtendedTopologyIter {
    type Item = ExtendedTopologyLevel;

    fn next(&mut self) -> Option<ExtendedTopologyLevel> {
        let res = cpuid!(EAX_EXTENDED_TOPOLOGY_INFO, self.level);
        self.level += 1;

        let et = ExtendedTopologyLevel { eax: res.eax, ebx: res.ebx, ecx: res.ecx, edx: res.edx };

        match et.level_type() {
            TopologyType::INVALID => None,
            _ => Some(et)
        }
    }
}


#[derive(Debug)]
pub struct ExtendedStateInfo {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    eax1: u32,
}

impl ExtendedStateInfo {

    /// Reports the valid bit fields of the lower 32 bits of XCR0. If a bit is 0,
    /// the corresponding bit field in XCR0 is reserved.
    pub fn xcr0(&self) -> u64 {
        (self.edx as u64) << 32 | self.eax as u64
    }

    /// Reports the valid bit fields of the upper 32 bits of XCR0. If a bit is 0,
    /// the corresponding bit field in XCR0 is reserved.
    pub fn xcr0_upper_bits(&self) -> u32 {
        self.edx
    }

    /// Maximum size (bytes, from the beginning of the XSAVE/XRSTOR save area) required by
    /// enabled features in XCR0. May be different than ECX if some features at the end of the XSAVE save area
    /// are not enabled.
    pub fn maximum_size_enabled_features(&self) -> u32 {
        self.ebx
    }

    /// Maximum size (bytes, from the beginning of the XSAVE/XRSTOR save area) of the
    /// XSAVE/XRSTOR save area required by all supported features in the processor,
    /// i.e all the valid bit fields in XCR0.
    pub fn maximum_size_supported_features(&self) -> u32 {
        self.ecx
    }

    /// CPU has xsaveopt feature.
    pub fn has_xsaveopt(&self) -> bool {
        self.eax1 & 0x1 == 1
    }

    /// Iterator over extended state enumeration levels >= 2.
    pub fn iter(&self) -> ExtendedStateIter {
        ExtendedStateIter { level: 2, xcr0: self.xcr0() }
    }

}


pub struct ExtendedStateIter {
    level: u32,
    xcr0: u64,
}

impl Iterator for ExtendedStateIter {
    type Item = ExtendedState;

    fn next(&mut self) -> Option<ExtendedState> {
        if self.level > 62 {
            return None;
        }

        let bit = 1 << self.level;
        self.level += 1;

        if self.xcr0 & bit > 0 {
            let res = cpuid!(EAX_EXTENDED_STATE_INFO, bit);
            if res.eax > 0 {
                let ident = match bit {
                    0 => ExtendedStateIdent::LegacyX87,
                    1 => ExtendedStateIdent::SSE128,
                    2 => ExtendedStateIdent::AVX256,
                    _ => panic!("Unknown bit, consider updating ExtendedStateIdent enum!")
                };

                return Some(ExtendedState { ident: ident, eax: res.eax, ebx: res.ebx });
            }
        }

        self.next()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExtendedStateIdent {
    /// legacy x87 (Bit 00).
    LegacyX87 = 1 << 0,

    /// 128-bit SSE (Bit 01).
    SSE128 = 1 << 1,

    /// 256-bit AVX (Bit 02).
    AVX256 = 1 << 2,
}

#[derive(Debug)]
pub struct ExtendedState {
    pub ident: ExtendedStateIdent,
    eax: u32,
    ebx: u32,
}

impl ExtendedState {
    /// The size in bytes (from the offset specified in EBX) of the save area
    /// for an extended state feature associated with a valid sub-leaf index, n.
    /// This field reports 0 if the sub-leaf index, n, is invalid.
    pub fn size(&self) -> u32 {
        self.eax
    }

    /// The offset in bytes of this extended state components save area
    /// from the beginning of the XSAVE/XRSTOR area.
    pub fn offset(&self) -> u32 {
        self.ebx
    }

}

#[derive(Debug)]
pub struct QoSInfo {
    ebx0: u32,
    edx0: u32,
    ebx1: u32,
    ecx1: u32,
    edx1: u32,
}

impl QoSInfo {

    /// Maximum range (zero-based) of RMID within this physical processor of all types.
    pub fn maximum_rmid_range(&self) -> u32 {
        self.ebx0
    }

    /// Supports L3 Cache QoS if true.
    pub fn has_l3_qos(&self) -> bool {
        self.edx0 & (1 << 1) > 0
    }

    /// Conversion factor from reported IA32_QM_CTR value to occupancy metric (bytes).
    pub fn conversion_factor(&self) -> u32 {
        self.ebx1
    }

    /// Maximum range (zero-based) of RMID of L3.
    pub fn maximum_range_l3_rmid(&self) -> u32 {
        self.ecx1
    }

    /// Supports L3 occupancy monitoring if true.
    pub fn has_l3_occupancy_monitoring(&self) -> bool {
        self.edx1 & 0x1 > 0
    }
}


#[derive(Debug)]
pub struct ExtendedFunctionInfo {
    max_eax_value: u32,
    data: [CpuIdResult; 9],
}

#[derive(PartialEq, Eq, Debug)]
pub enum L2Associativity {
    Disabled = 0x0,
    DirectMapped = 0x1,
    TwoWay = 0x2,
    FourWay = 0x4,
    EightWay = 0x6,
    SixteenWay = 0x8,
    FullyAssiciative = 0xF,
    Unknown,
}

const EAX_EXTENDED_PROC_SIGNATURE: u32 = 0x1;
const EAX_EXTENDED_BRAND_STRING: u32 = 0x4;
const EAX_EXTENDED_CACHE_INFO: u32 = 0x6;

impl ExtendedFunctionInfo {

    fn leaf_is_supported(&self, val: u32) -> bool {
        val <= self.max_eax_value
    }

    /// Retrieve processor brand string.
    pub fn processor_brand_string(&self) -> Option<&str> {
        if self.leaf_is_supported(EAX_EXTENDED_BRAND_STRING) {
            Some(unsafe {
                let brand_string_start = transmute::<&CpuIdResult, *const u8>(&self.data[2]);
                let slice = raw::Slice { data: brand_string_start, len: 3*4*4 };
                let byte_array: &'static [u8] = transmute(slice);
                str::from_utf8_unchecked(byte_array)
            })
        }
        else {
            None
        }
    }

    /// Extended Processor Signature and Feature Bits.
    pub fn extended_signature(&self) -> Option<u32> {
        if self.leaf_is_supported(EAX_EXTENDED_PROC_SIGNATURE) {
            Some(self.data[1].eax)
        }
        else {
            None
        }
    }

    /// Cache Line size in bytes
    pub fn cache_line_size(&self) -> Option<u8> {
        if self.leaf_is_supported(EAX_EXTENDED_CACHE_INFO) {
            Some(get_bits(self.data[6].ecx, 0, 7) as u8)
        }
        else {
            None
        }
    }

    /// L2 Associativity field
    pub fn l2_associativity(&self) -> Option<L2Associativity> {
        if self.leaf_is_supported(EAX_EXTENDED_CACHE_INFO) {
            Some(match get_bits(self.data[6].ecx, 12, 15) {
                0x0 => L2Associativity::Disabled,
                0x1 => L2Associativity::DirectMapped,
                0x2 => L2Associativity::TwoWay,
                0x4 => L2Associativity::FourWay,
                0x6 => L2Associativity::EightWay,
                0x8 => L2Associativity::SixteenWay,
                0xF => L2Associativity::FullyAssiciative,
                _ => L2Associativity::Unknown,
            })
        }
        else {
            None
        }
    }

    /// Cache size in 1K units
    pub fn cache_size(&self) -> Option<u16> {
        if self.leaf_is_supported(EAX_EXTENDED_CACHE_INFO) {
            Some(get_bits(self.data[6].ecx, 16, 31) as u16)
        }
        else {
            None
        }
    }

    /// #Physical Address Bits
    pub fn physical_address_bits(&self) -> Option<u8> {
        if self.leaf_is_supported(8) {
            Some(get_bits(self.data[8].eax, 0, 7) as u8)
        }
        else {
            None
        }
    }

    /// #Linear Address Bits
    pub fn linear_address_bits(&self) -> Option<u8> {
        if self.leaf_is_supported(8) {
            Some(get_bits(self.data[8].eax, 8, 15) as u8)
        }
        else {
            None
        }
    }

    /// Is Invariant TSC available?
    pub fn has_invariant_tsc(&self) -> bool {
        self.leaf_is_supported(7) && self.data[7].edx & (1 << 8) > 0
    }

    /// Is LAHF/SAHF available in 64-bit mode?
    pub fn has_lahf_sahf(&self) -> bool {
        self.leaf_is_supported(1) &&
        ExtendedFunctionInfoEcx{ bits: self.data[1].ecx }.contains(CPU_FEATURE_LAHF_SAHF)
    }

    /// Is LZCNT available?
    pub fn has_lzcnt(&self) -> bool {
        self.leaf_is_supported(1) &&
        ExtendedFunctionInfoEcx{ bits: self.data[1].ecx }.contains(CPU_FEATURE_LZCNT)
    }

    /// Is PREFETCHW available?
    pub fn has_prefetchw(&self) -> bool {
        self.leaf_is_supported(1) &&
        ExtendedFunctionInfoEcx{ bits: self.data[1].ecx }.contains(CPU_FEATURE_PREFETCHW)
    }

    /// Are fast system calls available.
    pub fn has_syscall_sysret(&self) -> bool {
        self.leaf_is_supported(1) &&
        ExtendedFunctionInfoEdx{ bits: self.data[1].edx }.contains(CPU_FEATURE_SYSCALL_SYSRET)
    }

    /// Is there support for execute disable bit.
    pub fn has_execute_disable(&self) -> bool {
        self.leaf_is_supported(1) &&
        ExtendedFunctionInfoEdx{ bits: self.data[1].edx }.contains(CPU_FEATURE_EXECUTE_DISABLE)
    }

    /// Is there support for 1GiB pages.
    pub fn has_1gib_pages(&self) -> bool {
        self.leaf_is_supported(1) &&
        ExtendedFunctionInfoEdx{ bits: self.data[1].edx }.contains(CPU_FEATURE_1GIB_PAGES)
    }

    /// Check support for rdtscp instruction.
    pub fn has_rdtscp(&self) -> bool {
        self.leaf_is_supported(1) &&
        ExtendedFunctionInfoEdx{ bits: self.data[1].edx }.contains(CPU_FEATURE_RDTSCP)
    }

    /// Check support for 64-bit mode.
    pub fn has_64bit_mode(&self) -> bool {
        self.leaf_is_supported(1) &&
        ExtendedFunctionInfoEdx{ bits: self.data[1].edx }.contains(CPU_FEATURE_64BIT_MODE)
    }


}

bitflags! {
    #[derive(Debug)]
    flags ExtendedFunctionInfoEcx: u32 {
        /// LAHF/SAHF available in 64-bit mode.
        const CPU_FEATURE_LAHF_SAHF = 1 << 0,

        /// Bit 05: LZCNT
        const CPU_FEATURE_LZCNT = 1 << 5,

        /// Bit 08: PREFETCHW
        const CPU_FEATURE_PREFETCHW = 1 << 8,
    }
}

bitflags! {
    #[derive(Debug)]
    flags ExtendedFunctionInfoEdx: u32 {
        /// SYSCALL/SYSRET available in 64-bit mode (Bit 11).
        const CPU_FEATURE_SYSCALL_SYSRET = 1 << 11,

        /// Execute Disable Bit available (Bit 20).
        const CPU_FEATURE_EXECUTE_DISABLE = 1 << 20,

        /// 1-GByte pages are available if 1 (Bit 26).
        const CPU_FEATURE_1GIB_PAGES = 1 << 26,

        /// RDTSCP and IA32_TSC_AUX are available if 1 (Bit 27).
        const CPU_FEATURE_RDTSCP = 1 << 27,

        /// Intel ® 64 Architecture available if 1 (Bit 29).
        const CPU_FEATURE_64BIT_MODE = 1 << 29,
    }
}

#[cfg(test)]
#[test]
fn genuine_intel() {
    let vf = VendorInfo { ebx: 1970169159, edx: 1231384169, ecx: 1818588270 };
    assert!(vf.as_string() == "GenuineIntel");
}

#[test]
fn feature_info() {
    let finfo = FeatureInfo { eax: 198313,
                              ebx: 34605056,
                              ecx: FeatureInfoEcx { bits: 2109399999 },
                              edx: FeatureInfoEdx { bits: 3219913727 }, };

    assert!(finfo.model_id() == 10);
    assert!(finfo.extended_model_id() == 3);
    assert!(finfo.stepping_id() == 9);
    assert!(finfo.extended_family_id() == 0);
    assert!(finfo.family_id() == 6);
    assert!(finfo.stepping_id() == 9);
    assert!(finfo.brand_index() == 0);

    assert!(finfo.edx.contains(CPU_FEATURE_SSE2));
    assert!(finfo.ecx.contains(CPU_FEATURE_SSE41));
}

#[test]
fn cache_info() {
    let cinfos = CacheInfoIter { current: 1,
                                 eax: 1979931137,
                                 ebx: 15774463,
                                 ecx: 0,
                                 edx: 13238272, };
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
            CacheParameter { eax: 469778721, ebx: 29360191, ecx: 63, edx: 0 },
            CacheParameter { eax: 469778722, ebx: 29360191, ecx: 63, edx: 0 },
            CacheParameter { eax: 469778755, ebx: 29360191, ecx: 511, edx: 0 },
            CacheParameter { eax: 470008163, ebx: 46137407, ecx: 4095, edx: 6 },
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
            },
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
            },
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
            },
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
            },
            _ => unreachable!()
        }
    }
}

#[test]
fn monitor_mwait_features() {
    let mmfeatures = MonitorMwaitInfo { eax: 64, ebx: 64, ecx: 3, edx: 135456 };
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
    let tpfeatures = ThermalPowerInfo { eax: ThermalPowerFeaturesEax { bits: 119 },
                                        ebx: 2,
                                        ecx: ThermalPowerFeaturesEcx { bits: 9 },
                                        edx: 0, };

    assert!(tpfeatures.eax.contains(CPU_FEATURE_DTS));
    assert!(tpfeatures.eax.contains(CPU_FEATURE_TURBO_BOOST));
    assert!(tpfeatures.eax.contains(CPU_FEATURE_ARAT));
    assert!(tpfeatures.eax.contains(CPU_FEATURE_PLN));
    assert!(tpfeatures.eax.contains(CPU_FEATURE_ECMD));
    assert!(tpfeatures.eax.contains(CPU_FEATURE_PTM));

    assert!(tpfeatures.ecx.contains(CPU_FEATURE_HW_COORD_FEEDBACK));
    assert!(tpfeatures.ecx.contains(CPU_FEATURE_ENERGY_BIAS_PREF));

    assert!(tpfeatures.dts_irq_threshold() == 0x2);
}

#[test]
fn extended_features() {
    let tpfeatures = ExtendedFeatures { eax: 0,
                                        ebx: ExtendedFeaturesEbx { bits: 641 },
                                        ecx: 0,
                                        edx: 0, };

    assert!(tpfeatures.eax == 0);

    assert!(tpfeatures.ebx.contains(CPU_FEATURE_FSGSBASE));
    assert!(!tpfeatures.ebx.contains(CPU_FEATURE_ADJUST_MSR));
    assert!(!tpfeatures.ebx.contains(CPU_FEATURE_BMI1));
    assert!(!tpfeatures.ebx.contains(CPU_FEATURE_HLE));
    assert!(!tpfeatures.ebx.contains(CPU_FEATURE_AVX2));
    assert!(tpfeatures.ebx.contains(CPU_FEATURE_SMEP));
    assert!(!tpfeatures.ebx.contains(CPU_FEATURE_BMI2));
    assert!(tpfeatures.ebx.contains(CPU_FEATURE_REP_MOVSB_STOSB));
    assert!(!tpfeatures.ebx.contains(CPU_FEATURE_INVPCID));
    assert!(!tpfeatures.ebx.contains(CPU_FEATURE_RTM));
    assert!(!tpfeatures.ebx.contains(CPU_FEATURE_QM));
    assert!(!tpfeatures.ebx.contains(CPU_FEATURE_DEPRECATE_FPU_CS_DS));

}

#[test]
fn direct_cache_access_info() {
    let dca = DirectCacheAccessInfo { eax: 0x1 };
    assert!(dca.get_dca_cap_value() == 0x1);
}

#[test]
fn performance_monitoring_info() {
    let pm = PerformanceMonitoringInfo { eax: 120587267,
                                         ebx: PerformanceMonitoringFeaturesEbx { bits: 0 },
                                         ecx: 0,
                                         edx: 1539, };

    assert!(pm.version_id() == 3);
    assert!(pm.number_of_counters() == 4);
    assert!(pm.counter_bit_width() == 48);
    assert!(pm.ebx_length() == 7);
    assert!(pm.fixed_function_counters() == 3);
    assert!(pm.fixed_function_counters_bit_width() == 48);

    assert!(!pm.ebx.contains(CPU_FEATURE_CORE_CYC_EV_UNAVAILABLE));
    assert!(!pm.ebx.contains(CPU_FEATURE_INST_RET_EV_UNAVAILABLE));
    assert!(!pm.ebx.contains(CPU_FEATURE_REF_CYC_EV_UNAVAILABLE));
    assert!(!pm.ebx.contains(CPU_FEATURE_CACHE_REF_EV_UNAVAILABLE));
    assert!(!pm.ebx.contains(CPU_FEATURE_LL_CACHE_MISS_EV_UNAVAILABLE));
    assert!(!pm.ebx.contains(CPU_FEATURE_BRANCH_INST_RET_EV_UNAVAILABLE));
    assert!(!pm.ebx.contains(CPU_FEATURE_BRANCH_MISPRED_EV_UNAVAILABLE));
}


#[cfg(test)]
#[test]
fn extended_topology_info() {
    let l1 = ExtendedTopologyLevel { eax: 1, ebx: 2, ecx: 256, edx: 3 };
    let l2 = ExtendedTopologyLevel { eax: 4, ebx: 4, ecx: 513, edx: 3 };

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
    let es = ExtendedStateInfo { eax: 7, ebx: 832, ecx: 832, edx: 0, eax1: 1 };

    assert!(es.xcr0() == 7);
    assert!(es.maximum_size_enabled_features() == 832);
    assert!(es.maximum_size_supported_features() == 832);
    assert!(es.has_xsaveopt());

    for (idx, e) in es.iter().enumerate() {
        match idx {
            0 => {
                assert!(e.ident == ExtendedStateIdent::AVX256);
                assert!(e.size() == 256);
                assert!(e.offset() == 576);
            }
            _ => unreachable!()
        }
    }
}

#[test]
fn quality_of_service_info() {
    let qos = QoSInfo { ebx0: 832, edx0: 0, ebx1: 0, ecx1: 0, edx1: 0 };

    assert!(qos.maximum_rmid_range() == 832);
    assert!(!qos.has_l3_qos());
    assert!(qos.conversion_factor() == 0x0);
    assert!(qos.maximum_range_l3_rmid() == 0x0);
    assert!(!qos.has_l3_occupancy_monitoring());
}

#[test]
fn extended_functions() {
    let ef = ExtendedFunctionInfo { max_eax_value: 8,
                                    data: [
            CpuIdResult { eax: 2147483656, ebx: 0, ecx: 0, edx: 0 },
            CpuIdResult { eax: 0, ebx: 0, ecx: 1, edx: 672139264 },
            CpuIdResult { eax: 538976288, ebx: 1226842144, ecx: 1818588270, edx: 539578920 },
            CpuIdResult { eax: 1701998403, ebx: 692933672, ecx: 758475040, edx: 926102323 },
            CpuIdResult { eax: 1346576469, ebx: 541073493, ecx: 808988209, edx: 8013895 },
            CpuIdResult { eax:0, ebx: 0, ecx: 0, edx: 0 },
            CpuIdResult { eax: 0, ebx: 0, ecx: 16801856, edx: 0 },
            CpuIdResult { eax: 0, ebx: 0, ecx: 0, edx: 256 },
            CpuIdResult { eax: 12324, ebx: 0, ecx: 0, edx: 0 }
        ], };

    assert!(ef.processor_brand_string().unwrap() == "       Intel(R) Core(TM) i5-3337U CPU @ 1.80GHz\0");
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
fn readme_test() {
    /*
    let cpuid = CpuId::new();

    match cpuid.get_vendor_info() {
        Some(vf) => assert!(vf.as_string() == "GenuineIntel"),
        None => ()
    }

    let has_sse = match cpuid.get_feature_info() {
        Some(finfo) => finfo.has_sse(),
        None => false
    };

    if has_sse {
        println!("CPU supports SSE!");
    }

    match cpuid.get_cache_parameters() {
        Some(cparams) => {
            for cache in cparams {
                let size = cache.associativity() * cache.physical_line_partitions() * cache.coherency_line_size() * cache.sets();
                println!("L{}-Cache size is {}", cache.level(), size);
            }
        },
        None => println!("No cache parameter information available"),
    }*/
}
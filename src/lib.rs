#![feature(no_std, core, core_prelude, asm, raw)]
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

#[cfg(test)]
#[prelude_import]
use std::prelude::v1::*;

use core::prelude::*;
use core::iter;
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

const MAX_ENTRIES: usize = 32;

#[macro_export]
macro_rules! cpuid {
    ($eax:expr)
        => ( $crate::cpuid1($eax as u32) );

    ($eax:expr, $ecx:expr)
        => ( $crate::cpuid2($eax as u32, $ecx as u32) );

}

fn cpuid2(eax: u32, ecx: u32) -> CpuIdResult {
    let mut res = CpuIdResult { eax: 0, ebx: 0, ecx: 0, edx: 0 };

    unsafe {
        asm!("movl $0, %eax" : : "r" (eax) : "eax");
        asm!("movl $0, %ecx" : : "r" (ecx) : "ecx");
        asm!("cpuid" : "={eax}"(res.eax) "={ebx}"(res.ebx)
                       "={ecx}"(res.ecx) "={edx}"(res.edx)
                     :: "eax", "ebx", "ecx", "edx");
    }

    res
}

fn cpuid1(eax: u32) -> CpuIdResult {
    let mut res = CpuIdResult { eax: 0, ebx: 0, ecx: 0, edx: 0 };

    unsafe {
        asm!("movl $0, %eax" : : "r" (eax) : "eax");
        asm!("cpuid" : "={eax}"(res.eax) "={ebx}"(res.ebx)
                       "={ecx}"(res.ecx) "={edx}"(res.edx)
                     :: "eax", "ebx", "ecx", "edx");
    }

    res
}

enum CpuIdLeaf {
    VendorInformation,
    FeatureInformation,
    CacheInformation,
    ProcessorSerial,
    CacheParameters,
    MonitorMwait,
    ThermalPowerManagement,
    StructuredExtendedFeature,
    DirectCacheAccess,
    PerformanceMonitoring,
    ExtendedTopology,
    ProcessorExtendedState,
    QualityofService,
    ExtendedFunction,
}

struct LeafData(CpuIdLeaf, &'static str, u32);

const LEAF_INFORMATION: [LeafData; 14] = [
    LeafData(CpuIdLeaf::VendorInformation, "GenuineIntel", 0x0),
    LeafData(CpuIdLeaf::FeatureInformation, "Version/Feature Information", 0x1),
    LeafData(CpuIdLeaf::CacheInformation, "Cache and TLB Information", 0x2),
    LeafData(CpuIdLeaf::ProcessorSerial, "Processor serial number", 0x3),
    LeafData(CpuIdLeaf::CacheParameters, "Deterministic Cache Parameters", 0x4),
    LeafData(CpuIdLeaf::MonitorMwait, "MONITOR/MWAIT", 0x5),
    LeafData(CpuIdLeaf::ThermalPowerManagement, "Thermal and Power Management", 0x6),
    LeafData(CpuIdLeaf::StructuredExtendedFeature, "Structured Extended Feature Flags", 0x7),
    LeafData(CpuIdLeaf::DirectCacheAccess, "Direct Cache Access Information", 0x9),
    LeafData(CpuIdLeaf::PerformanceMonitoring, "Architectural Performance Monitoring", 0xA),
    LeafData(CpuIdLeaf::ExtendedTopology, "Extended Topology Enumeration", 0xB),
    LeafData(CpuIdLeaf::ProcessorExtendedState, "Processor Extended State Enumeration", 0xD),
    LeafData(CpuIdLeaf::QualityofService, "Quality of Service", 0xF),
    LeafData(CpuIdLeaf::ExtendedFunction, "Extended Function CPUID Information", 0x80000000),
];

#[derive(Debug)]
pub struct CpuId;

#[derive(Debug, Copy, Clone)]
pub struct CpuIdResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl CpuId {

    pub fn get_vendor_information(&self) -> CpuIdVendorInfo {
        let res = cpuid!(0);
        CpuIdVendorInfo { ebx: res.ebx, ecx: res.ecx, edx: res.edx }
    }

    pub fn get_feature_information(&self) -> CpuIdFeatureInfo {
        let res = cpuid!(1);
        CpuIdFeatureInfo { eax: res.eax,
                           ebx: res.ebx,
                           ecx: FeatureInfoEcx { bits: res.ecx },
                           edx: FeatureInfoEdx { bits: res.edx },
        }
    }

    #[cfg(test)]
    pub fn get_cache_information(&self) -> CpuIdCacheInfo {
        let res = cpuid!(2);
        CpuIdCacheInfo { eax: res.eax,
                         ebx: res.ebx,
                         ecx: res.ecx,
                         edx: res.edx }
    }
}

pub struct CpuIdVendorInfo {
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

pub struct CpuIdCacheInfo {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl CpuIdCacheInfo {
    pub fn iter<'a>(&'a self) -> CacheInfoIter<'a> {
        CacheInfoIter{
            current: 0,
            regs: self
        }
    }
}

pub struct CacheInfoIter<'a> {
    current: u32,
    regs: &'a CpuIdCacheInfo
}

impl<'a> Iterator for CacheInfoIter<'a> {
    type Item = CacheInfo;

    fn next(&mut self) -> Option<CacheInfo> {
        if self.current >= 4*4 {
            return None;
        }

        let reg_index = self.current % 4;
        let byte_index = self.current / 4;

        let reg = match reg_index {
            0 => self.regs.eax,
            1 => self.regs.ebx,
            2 => self.regs.ecx,
            3 => self.regs.edx,
            _ => unreachable!()
        };

        let byte = as_bytes(&reg)[byte_index as usize];

        for cache_info in CACHE_INFO_TABLE.into_iter() {
            if cache_info.num == byte {
                self.current += 1;
                return Some(*cache_info);
            }
        }

        None
    }
}

#[derive(Copy, Clone)]
enum CacheType {
    GENERAL,
    CACHE,
    TLB,
    STLB,
    DTLB,
    PREFETCH
}

#[derive(Copy, Clone)]
pub struct CacheInfo{
    num: u8,
    typ: CacheType,
    desc: &'static str
}

impl fmt::Display for CacheInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let typ = match self.typ {
            CacheType::GENERAL => "N/A",
            CacheType::CACHE => "Cache",
            CacheType::TLB => "TLB",
            CacheType::STLB => "STLB",
            CacheType::DTLB => "DTLB",
            CacheType::PREFETCH => "Prefetcher"
        };

        write!(f, "{:x}:\t {}: {}", self.num, typ, self.desc)
    }
}

pub const CACHE_INFO_TABLE: [CacheInfo; 103] = [
    CacheInfo{num: 0x00, typ: CacheType::GENERAL, desc: "Null descriptor, this byte contains no information"},
    CacheInfo{num: 0x01, typ: CacheType::TLB, desc: "Instruction TLB: 4 KByte pages, 4-way set associative, 32 entries"},
    CacheInfo{num: 0x02, typ: CacheType::TLB, desc: "Instruction TLB: 4 MByte pages, fully associative, 2 entries"},
    CacheInfo{num: 0x03, typ: CacheType::TLB, desc: "Data TLB: 4 KByte pages, 4-way set associative, 64 entries"},
    CacheInfo{num: 0x04, typ: CacheType::TLB, desc: "Data TLB: 4 MByte pages, 4-way set associative, 8 entries"},
    CacheInfo{num: 0x05, typ: CacheType::TLB, desc: "Data TLB1: 4 MByte pages, 4-way set associative, 32 entries"},
    CacheInfo{num: 0x06, typ: CacheType::CACHE, desc: "1st-level instruction cache: 8 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x08, typ: CacheType::CACHE, desc: "1st-level instruction cache: 16 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x09, typ: CacheType::CACHE, desc: "1st-level instruction cache: 32KBytes, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x0A, typ: CacheType::CACHE, desc: "1st-level data cache: 8 KBytes, 2-way set associative, 32 byte line size"},
    CacheInfo{num: 0x0B, typ: CacheType::TLB, desc: "Instruction TLB: 4 MByte pages, 4-way set associative, 4 entries"},
    CacheInfo{num: 0x0C, typ: CacheType::CACHE, desc: "1st-level data cache: 16 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x0D, typ: CacheType::CACHE, desc: "1st-level data cache: 16 KBytes, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x0E, typ: CacheType::CACHE, desc: "1st-level data cache: 24 KBytes, 6-way set associative, 64 byte line size"},
    CacheInfo{num: 0x21, typ: CacheType::CACHE, desc: "2nd-level cache: 256 KBytes, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x22, typ: CacheType::CACHE, desc: "3rd-level cache: 512 KBytes, 4-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x23, typ: CacheType::CACHE, desc: "3rd-level cache: 1 MBytes, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x24, typ: CacheType::CACHE, desc: "2nd-level cache: 1 MBytes, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0x25, typ: CacheType::CACHE, desc: "3rd-level cache: 2 MBytes, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x29, typ: CacheType::CACHE, desc: "3rd-level cache: 4 MBytes, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x2C, typ: CacheType::CACHE, desc: "1st-level data cache: 32 KBytes, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x30, typ: CacheType::CACHE, desc: "1st-level instruction cache: 32 KBytes, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x40, typ: CacheType::CACHE, desc: "No 2nd-level cache or, if processor contains a valid 2nd-level cache, no 3rd-level cache"},
    CacheInfo{num: 0x41, typ: CacheType::CACHE, desc: "2nd-level cache: 128 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x42, typ: CacheType::CACHE, desc: "2nd-level cache: 256 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x43, typ: CacheType::CACHE, desc: "2nd-level cache: 512 KBytes, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x44, typ: CacheType::CACHE, desc: "2nd-level cache: 1 MByte, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x45, typ: CacheType::CACHE, desc: "2nd-level cache: 2 MByte, 4-way set associative, 32 byte line size"},
    CacheInfo{num: 0x46, typ: CacheType::CACHE, desc: "3rd-level cache: 4 MByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x47, typ: CacheType::CACHE, desc: "3rd-level cache: 8 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x48, typ: CacheType::CACHE, desc: "2nd-level cache: 3MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0x49, typ: CacheType::CACHE, desc: "3rd-level cache: 4MB, 16-way set associative, 64-byte line size (Intel Xeon processor MP, Family 0FH, Model 06H); 2nd-level cache: 4 MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4A, typ: CacheType::CACHE, desc: "3rd-level cache: 6MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4B, typ: CacheType::CACHE, desc: "3rd-level cache: 8MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4C, typ: CacheType::CACHE, desc: "3rd-level cache: 12MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4D, typ: CacheType::CACHE, desc: "3rd-level cache: 16MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4E, typ: CacheType::CACHE, desc: "2nd-level cache: 6MByte, 24-way set associative, 64 byte line size"},
    CacheInfo{num: 0x4F, typ: CacheType::TLB, desc: "Instruction TLB: 4 KByte pages, 32 entries"},
    CacheInfo{num: 0x50, typ: CacheType::TLB, desc: "Instruction TLB: 4 KByte and 2-MByte or 4-MByte pages, 64 entries"},
    CacheInfo{num: 0x51, typ: CacheType::TLB, desc: "Instruction TLB: 4 KByte and 2-MByte or 4-MByte pages, 128 entries"},
    CacheInfo{num: 0x52, typ: CacheType::TLB, desc: "Instruction TLB: 4 KByte and 2-MByte or 4-MByte pages, 256 entries"},
    CacheInfo{num: 0x55, typ: CacheType::TLB, desc: "Instruction TLB: 2-MByte or 4-MByte pages, fully associative, 7 entries"},
    CacheInfo{num: 0x56, typ: CacheType::TLB, desc: "Data TLB0: 4 MByte pages, 4-way set associative, 16 entries"},
    CacheInfo{num: 0x57, typ: CacheType::TLB, desc: "Data TLB0: 4 KByte pages, 4-way associative, 16 entries"},
    CacheInfo{num: 0x59, typ: CacheType::TLB, desc: "Data TLB0: 4 KByte pages, fully associative, 16 entries"},
    CacheInfo{num: 0x5A, typ: CacheType::TLB, desc: "Data TLB0: 2-MByte or 4 MByte pages, 4-way set associative, 32 entries"},
    CacheInfo{num: 0x5B, typ: CacheType::TLB, desc: "Data TLB: 4 KByte and 4 MByte pages, 64 entries"},
    CacheInfo{num: 0x5C, typ: CacheType::TLB, desc: "Data TLB: 4 KByte and 4 MByte pages,128 entries"},
    CacheInfo{num: 0x5D, typ: CacheType::TLB, desc: "Data TLB: 4 KByte and 4 MByte pages,256 entries"},
    CacheInfo{num: 0x60, typ: CacheType::CACHE, desc: "1st-level data cache: 16 KByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0x61, typ: CacheType::TLB, desc: "Instruction TLB: 4 KByte pages, fully associative, 48 entries"},
    CacheInfo{num: 0x63, typ: CacheType::TLB, desc: "Data TLB: 1 GByte pages, 4-way set associative, 4 entries"},
    CacheInfo{num: 0x66, typ: CacheType::CACHE, desc: "1st-level data cache: 8 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x67, typ: CacheType::CACHE, desc: "1st-level data cache: 16 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x68, typ: CacheType::CACHE, desc: "1st-level data cache: 32 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x70, typ: CacheType::CACHE, desc: "Trace cache: 12 K-μop, 8-way set associative"},
    CacheInfo{num: 0x71, typ: CacheType::CACHE, desc: "Trace cache: 16 K-μop, 8-way set associative"},
    CacheInfo{num: 0x72, typ: CacheType::CACHE, desc: "Trace cache: 32 K-μop, 8-way set associative"},
    CacheInfo{num: 0x76, typ: CacheType::TLB, desc: "Instruction TLB: 2M/4M pages, fully associative, 8 entries"},
    CacheInfo{num: 0x78, typ: CacheType::CACHE, desc: "2nd-level cache: 1 MByte, 4-way set associative, 64byte line size"},
    CacheInfo{num: 0x79, typ: CacheType::CACHE, desc: "2nd-level cache: 128 KByte, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x7A, typ: CacheType::CACHE, desc: "2nd-level cache: 256 KByte, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x7B, typ: CacheType::CACHE, desc: "2nd-level cache: 512 KByte, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x7C, typ: CacheType::CACHE, desc: "2nd-level cache: 1 MByte, 8-way set associative, 64 byte line size, 2 lines per sector"},
    CacheInfo{num: 0x7D, typ: CacheType::CACHE, desc: "2nd-level cache: 2 MByte, 8-way set associative, 64byte line size"},
    CacheInfo{num: 0x7F, typ: CacheType::CACHE, desc: "2nd-level cache: 512 KByte, 2-way set associative, 64-byte line size"},
    CacheInfo{num: 0x80, typ: CacheType::CACHE, desc: "2nd-level cache: 512 KByte, 8-way set associative, 64-byte line size"},
    CacheInfo{num: 0x82, typ: CacheType::CACHE, desc: "2nd-level cache: 256 KByte, 8-way set associative, 32 byte line size"},
    CacheInfo{num: 0x83, typ: CacheType::CACHE, desc: "2nd-level cache: 512 KByte, 8-way set associative, 32 byte line size"},
    CacheInfo{num: 0x84, typ: CacheType::CACHE, desc: "2nd-level cache: 1 MByte, 8-way set associative, 32 byte line size"},
    CacheInfo{num: 0x85, typ: CacheType::CACHE, desc: "2nd-level cache: 2 MByte, 8-way set associative, 32 byte line size"},
    CacheInfo{num: 0x86, typ: CacheType::CACHE, desc: "2nd-level cache: 512 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0x87, typ: CacheType::CACHE, desc: "2nd-level cache: 1 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0xB0, typ: CacheType::TLB, desc: "Instruction TLB: 4 KByte pages, 4-way set associative, 128 entries"},
    CacheInfo{num: 0xB1, typ: CacheType::TLB, desc: "Instruction TLB: 2M pages, 4-way, 8 entries or 4M pages, 4-way, 4 entries"},
    CacheInfo{num: 0xB2, typ: CacheType::TLB, desc: "Instruction TLB: 4KByte pages, 4-way set associative, 64 entries"},
    CacheInfo{num: 0xB3, typ: CacheType::TLB, desc: "Data TLB: 4 KByte pages, 4-way set associative, 128 entries"},
    CacheInfo{num: 0xB4, typ: CacheType::TLB, desc: "Data TLB1: 4 KByte pages, 4-way associative, 256 entries"},
    CacheInfo{num: 0xB5, typ: CacheType::TLB, desc: "Instruction TLB: 4KByte pages, 8-way set associative, 64 entries"},
    CacheInfo{num: 0xB6, typ: CacheType::TLB, desc: "Instruction TLB: 4KByte pages, 8-way set associative, 128 entries"},
    CacheInfo{num: 0xBA, typ: CacheType::TLB, desc: "Data TLB1: 4 KByte pages, 4-way associative, 64 entries"},
    CacheInfo{num: 0xC0, typ: CacheType::TLB, desc: "Data TLB: 4 KByte and 4 MByte pages, 4-way associative, 8 entries"},
    CacheInfo{num: 0xC1, typ: CacheType::STLB, desc: "Shared 2nd-Level TLB: 4 KByte/2MByte pages, 8-way associative, 1024 entries"},
    CacheInfo{num: 0xC2, typ: CacheType::DTLB, desc: "DTLB: 2 MByte/$MByte pages, 4-way associative, 16 entries"},
    CacheInfo{num: 0xCA, typ: CacheType::STLB, desc: "Shared 2nd-Level TLB: 4 KByte pages, 4-way associative, 512 entries"},
    CacheInfo{num: 0xD0, typ: CacheType::CACHE, desc: "3rd-level cache: 512 KByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD1, typ: CacheType::CACHE, desc: "3rd-level cache: 1 MByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD2, typ: CacheType::CACHE, desc: "3rd-level cache: 2 MByte, 4-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD6, typ: CacheType::CACHE, desc: "3rd-level cache: 1 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD7, typ: CacheType::CACHE, desc: "3rd-level cache: 2 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0xD8, typ: CacheType::CACHE, desc: "3rd-level cache: 4 MByte, 8-way set associative, 64 byte line size"},
    CacheInfo{num: 0xDC, typ: CacheType::CACHE, desc: "3rd-level cache: 1.5 MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0xDD, typ: CacheType::CACHE, desc: "3rd-level cache: 3 MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0xDE, typ: CacheType::CACHE, desc: "3rd-level cache: 6 MByte, 12-way set associative, 64 byte line size"},
    CacheInfo{num: 0xE2, typ: CacheType::CACHE, desc: "3rd-level cache: 2 MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0xE3, typ: CacheType::CACHE, desc: "3rd-level cache: 4 MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0xE4, typ: CacheType::CACHE, desc: "3rd-level cache: 8 MByte, 16-way set associative, 64 byte line size"},
    CacheInfo{num: 0xEA, typ: CacheType::CACHE, desc: "3rd-level cache: 12MByte, 24-way set associative, 64 byte line size"},
    CacheInfo{num: 0xEB, typ: CacheType::CACHE, desc: "3rd-level cache: 18MByte, 24-way set associative, 64 byte line size"},
    CacheInfo{num: 0xEC, typ: CacheType::CACHE, desc: "3rd-level cache: 24MByte, 24-way set associative, 64 byte line size"},
    CacheInfo{num: 0xF0, typ: CacheType::PREFETCH, desc: "64-Byte prefetching"},
    CacheInfo{num: 0xF1, typ: CacheType::PREFETCH, desc: "128-Byte prefetching"},
    CacheInfo{num: 0xFF, typ: CacheType::GENERAL, desc: "CPUID leaf 2 does not report cache descriptor information, use CPUID leaf 4 to query cache parameters"},
];

fn as_bytes(v: &u32) -> &[u8] {
    let start = v as *const u32 as *const u8;
    unsafe { slice::from_raw_parts(start, 4) }
}

impl fmt::Display for CpuIdVendorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "{}{}{}",
               str::from_utf8_unchecked(as_bytes(&self.ebx)),
               str::from_utf8_unchecked(as_bytes(&self.edx)),
               str::from_utf8_unchecked(as_bytes(&self.ecx)))
        }
    }
}

pub struct CpuIdFeatureInfo {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: FeatureInfoEcx,
    pub edx: FeatureInfoEdx,
}

bitflags! {
    flags FeatureInfoEcx: u32 {
        /// Streaming SIMD Extensions 3 (SSE3). A value of 1 indicates the processor supports this technology.
        const CPU_HAS_SSE3 = 1 << 0,
        /// PCLMULQDQ. A value of 1 indicates the processor supports the PCLMULQDQ instruction
        const CPU_HAS_PCLMULQDQ = 1 << 1,
        /// 64-bit DS Area. A value of 1 indicates the processor supports DS area using 64-bit layout
        const CPU_HAS_DTES64 = 1 << 2,
        /// MONITOR/MWAIT. A value of 1 indicates the processor supports this feature.
        const CPU_HAS_MONITOR = 1 << 3,
        /// CPL Qualified Debug Store. A value of 1 indicates the processor supports the extensions to the  Debug Store feature to allow for branch message storage qualified by CPL.
        const CPU_HAS_DSCPL = 1 << 4,
        /// Virtual Machine Extensions. A value of 1 indicates that the processor supports this technology.
        const CPU_HAS_VMX = 1 << 5,
        /// Safer Mode Extensions. A value of 1 indicates that the processor supports this technology. See Chapter 5, Safer Mode Extensions Reference.
        const CPU_HAS_SMX = 1 << 6,
        /// Enhanced Intel SpeedStep® technology. A value of 1 indicates that the processor supports this technology.
        const CPU_HAS_EIST = 1 << 7,
        /// Thermal Monitor 2. A value of 1 indicates whether the processor supports this technology.
        const CPU_HAS_TM2 = 1 << 8,
        /// A value of 1 indicates the presence of the Supplemental Streaming SIMD Extensions 3 (SSSE3). A value of 0 indicates the instruction extensions are not present in the processor
        const CPU_HAS_SSSE3 = 1 << 9,
        /// L1 Context ID. A value of 1 indicates the L1 data cache mode can be set to either adaptive mode or shared mode. A value of 0 indicates this feature is not supported. See definition of the IA32_MISC_ENABLE MSR Bit 24 (L1 Data Cache Context Mode) for details.
        const CPU_HAS_CNXTID = 1 << 10,
        /// A value of 1 indicates the processor supports FMA extensions using YMM state.
        const CPU_HAS_FMA = 1 << 12,
        /// CMPXCHG16B Available. A value of 1 indicates that the feature is available. See the CMPXCHG8B/CMPXCHG16B Compare and Exchange Bytes section. 14
        const CPU_HAS_CMPXCHG16B = 1 << 13,
        /// Perfmon and Debug Capability: A value of 1 indicates the processor supports the performance   and debug feature indication MSR IA32_PERF_CAPABILITIES.
        const CPU_HAS_PDCM = 1 << 15,
        /// Process-context identifiers. A value of 1 indicates that the processor supports PCIDs and the software may set CR4.PCIDE to 1.
        const CPU_HAS_PCID = 1 << 17,
        /// A value of 1 indicates the processor supports the ability to prefetch data from a memory mapped device.
        const CPU_HAS_DCA = 1 << 18,
        /// A value of 1 indicates that the processor supports SSE4.1.
        const CPU_HAS_SSE41 = 1 << 19,
        /// A value of 1 indicates that the processor supports SSE4.2.
        const CPU_HAS_SSE42 = 1 << 20,
        /// A value of 1 indicates that the processor supports x2APIC feature.
        const CPU_HAS_X2APIC = 1 << 21,
        /// A value of 1 indicates that the processor supports MOVBE instruction.
        const CPU_HAS_MOVBE = 1 << 22,
        /// A value of 1 indicates that the processor supports the POPCNT instruction.
        const CPU_HAS_POPCNT = 1 << 23,
        /// A value of 1 indicates that the processors local APIC timer supports one-shot operation using a TSC deadline value.
        const CPU_HAS_TSC_DEADLINE = 1 << 24,
        /// A value of 1 indicates that the processor supports the AESNI instruction extensions.
        const CPU_HAS_AESNI = 1 << 25,
        /// A value of 1 indicates that the processor supports the XSAVE/XRSTOR processor extended states feature, the XSETBV/XGETBV instructions, and XCR0.
        const CPU_HAS_XSAVE = 1 << 26,
        /// A value of 1 indicates that the OS has enabled XSETBV/XGETBV instructions to access XCR0, and support for processor extended state management using XSAVE/XRSTOR.
        const CPU_HAS_OSXSAVE = 1 << 27,
        /// A value of 1 indicates the processor supports the AVX instruction extensions.
        const CPU_HAS_AVX = 1 << 28,
        /// A value of 1 indicates that processor supports 16-bit floating-point conversion instructions.
        const CPU_HAS_F16C = 1 << 29,
        /// A value of 1 indicates that processor supports RDRAND instruction.
        const CPU_HAS_RDRAND = 1 << 30,
    }
}


bitflags! {
    flags FeatureInfoEdx: u32 {
        /// Floating Point Unit On-Chip. The processor contains an x87 FPU.
        const CPU_HAS_FPU = 1 << 0,
        /// Virtual 8086 Mode Enhancements. Virtual 8086 mode enhancements, including CR4.VME for controlling the feature, CR4.PVI for protected mode virtual interrupts, software interrupt indirection, expansion of the TSS with the software indirection bitmap, and EFLAGS.VIF and EFLAGS.VIP flags.
        const CPU_HAS_VME = 1 << 1,
        /// Debugging Extensions. Support for I/O breakpoints, including CR4.DE for controlling the feature, and optional trapping of accesses to DR4 and DR5.
        const CPU_HAS_DE = 1 << 2,
        /// Page Size Extension. Large pages of size 4 MByte are supported, including CR4.PSE for controlling the feature, the defined dirty bit in PDE (Page Directory Entries), optional reserved bit trapping in CR3, PDEs, and PTEs.
        const CPU_HAS_PSE = 1 << 3,
        /// Time Stamp Counter. The RDTSC instruction is supported, including CR4.TSD for controlling privilege.
        const CPU_HAS_TSC = 1 << 4,
        /// Model Specific Registers RDMSR and WRMSR Instructions. The RDMSR and WRMSR instructions are supported. Some of the MSRs are implementation dependent.
        const CPU_HAS_MSR = 1 << 5,
        /// Physical Address Extension. Physical addresses greater than 32 bits are supported: extended page table entry formats, an extra level in the page translation tables is defined, 2-MByte pages are supported instead of 4 Mbyte pages if PAE bit is 1.
        const CPU_HAS_PAE = 1 << 6,
        /// Machine Check Exception. Exception 18 is defined for Machine Checks, including CR4.MCE for controlling the feature. This feature does not define the model-specific implementations of machine-check error logging, reporting, and processor shutdowns. Machine Check exception handlers may have to depend on processor version to do model specific processing of the exception, or test for the presence of the Machine Check feature.
        const CPU_HAS_MCE = 1 << 7,
        /// CMPXCHG8B Instruction. The compare-and-exchange 8 bytes (64 bits) instruction is supported (implicitly locked and atomic).
        const CPU_HAS_CX8 = 1 << 8,
        /// APIC On-Chip. The processor contains an Advanced Programmable Interrupt Controller (APIC), responding to memory mapped commands in the physical address range FFFE0000H to FFFE0FFFH (by default - some processors permit the APIC to be relocated).
        const CPU_HAS_APIC = 1 << 9,
        /// SYSENTER and SYSEXIT Instructions. The SYSENTER and SYSEXIT and associated MSRs are supported.
        const CPU_HAS_SEP = 1 << 11,
        /// Memory Type Range Registers. MTRRs are supported. The MTRRcap MSR contains feature bits that describe what memory types are supported, how many variable MTRRs are supported, and whether fixed MTRRs are supported.
        const CPU_HAS_MTRR = 1 << 12,
        /// Page Global Bit. The global bit is supported in paging-structure entries that map a page, indicating TLB entries that are common to different processes and need not be flushed. The CR4.PGE bit controls this feature.
        const CPU_HAS_PGE = 1 << 13,
        /// Machine Check Architecture. The Machine Check Architecture, which provides a compatible mechanism for error reporting in P6 family, Pentium 4, Intel Xeon processors, and future processors, is supported. The MCG_CAP MSR contains feature bits describing how many banks of error reporting MSRs are supported.
        const CPU_HAS_MCA = 1 << 14,
        /// Conditional Move Instructions. The conditional move instruction CMOV is supported. In addition, if x87 FPU is present as indicated by the CPUID.FPU feature bit, then the FCOMI and FCMOV instructions are supported
        const CPU_HAS_CMOV = 1 << 15,
        /// Page Attribute Table. Page Attribute Table is supported. This feature augments the Memory Type Range Registers (MTRRs), allowing an operating system to specify attributes of memory accessed through a linear address on a 4KB granularity.
        const CPU_HAS_PAT = 1 << 16,
        /// 36-Bit Page Size Extension. 4-MByte pages addressing physical memory beyond 4 GBytes are supported with 32-bit paging. This feature indicates that upper bits of the physical address of a 4-MByte page are encoded in bits 20:13 of the page-directory entry. Such physical addresses are limited by MAXPHYADDR and may be up to 40 bits in size.
        const CPU_HAS_PSE36 = 1 << 17,
        /// Processor Serial Number. The processor supports the 96-bit processor identification number feature and the feature is enabled.
        const CPU_HAS_PSN = 1 << 18,
        /// CLFLUSH Instruction. CLFLUSH Instruction is supported.
        const CPU_HAS_CLFSH = 1 << 19,
        /// Debug Store. The processor supports the ability to write debug information into a memory resident buffer. This feature is used by the branch trace store (BTS) and precise event-based sampling (PEBS) facilities (see Chapter 23, Introduction to Virtual-Machine Extensions, in the Intel® 64 and IA-32 Architectures Software Developers Manual, Volume 3C).
        const CPU_HAS_DS = 1 << 21,
        /// Thermal Monitor and Software Controlled Clock Facilities. The processor implements internal MSRs that allow processor temperature to be monitored and processor performance to be modulated in predefined duty cycles under software control.
        const CPU_HAS_ACPI = 1 << 22,
        /// Intel MMX Technology. The processor supports the Intel MMX technology.
        const CPU_HAS_MMX = 1 << 23,
        /// FXSAVE and FXRSTOR Instructions. The FXSAVE and FXRSTOR instructions are supported for fast save and restore of the floating point context. Presence of this bit also indicates that CR4.OSFXSR is available for an operating system to indicate that it supports the FXSAVE and FXRSTOR instructions.
        const CPU_HAS_FXSR = 1 << 24,
        /// SSE. The processor supports the SSE extensions.
        const CPU_HAS_SSE = 1 << 25,
        /// SSE2. The processor supports the SSE2 extensions.
        const CPU_HAS_SSE2 = 1 << 26,
        /// Self Snoop. The processor supports the management of conflicting memory types by performing a snoop of its own cache structure for transactions issued to the bus.
        const CPU_HAS_SS = 1 << 27,
        /// Max APIC IDs reserved field is Valid. A value of 0 for HTT indicates there is only a single logical processor in the package and software should assume only a single APIC ID is reserved.  A value of 1 for HTT indicates the value in CPUID.1.EBX[23:16] (the Maximum number of addressable IDs for logical processors in this package) is valid for the package.
        const CPU_HAS_HTT = 1 << 28,
        /// Thermal Monitor. The processor implements the thermal monitor automatic thermal control circuitry (TCC).
        const CPU_HAS_TM = 1 << 29,
        /// Pending Break Enable. The processor supports the use of the FERR#/PBE# pin when the processor is in the stop-clock state (STPCLK# is asserted) to signal the processor that an interrupt is pending and that the processor should return to normal operation to handle the interrupt. Bit 10 (PBE enable) in the IA32_MISC_ENABLE MSR enables this capability.
        const CPU_HAS_PBE = 1 << 31,
    }
}



impl CpuIdFeatureInfo {

    pub fn get_extended_family_id(&self) -> u8 {
        ((self.eax >> 20) & 0xff) as u8
    }

    pub fn get_extended_model_id(&self) -> u8 {
        ((self.eax >> 16) & 0b1111) as u8
    }

    pub fn get_family_id(&self) -> u8 {
        ((self.eax >> 8) & 0b1111) as u8
    }

    pub fn get_model(&self) -> u8 {
        ((self.eax >> 4) & 0b1111) as u8
    }

    pub fn get_stepping_id(&self) -> u8 {
        ((self.eax & 0b1111)) as u8
    }

    pub fn get_brand_index(&self) -> u8 {
        (self.ebx) as u8
    }

    pub fn get_cflush_cache_line_size(&self) -> u8 {
        ((self.ebx >> 8)) as u8
    }

    pub fn get_local_apic_id(&self) -> u8 {
        ((self.ebx >> 24)) as u8
    }
}

#[cfg(test)]
#[test]
fn genuine_intel() {
    let cpu: CpuId = CpuId;
    let vinfo = cpu.get_vendor_information();
    println!("{}", vinfo);

    // GenuineIntel
    assert!(vinfo.ebx == 0x756e6547);
    assert!(vinfo.edx == 0x49656e69);
    assert!(vinfo.ecx == 0x6c65746e);
}

#[cfg(test)]
#[test]
fn feature_info() {
    let cpu: CpuId = CpuId;
    let finfo = cpu.get_feature_information();

    println!("{}", finfo.get_extended_family_id());
    println!("{}", finfo.get_extended_model_id());
    println!("{}", finfo.get_family_id());
    println!("{}", finfo.get_model());
    println!("{}", finfo.get_stepping_id());
    println!("{}", finfo.get_brand_index());
    println!("{}", finfo.get_cflush_cache_line_size());
    println!("{}", finfo.get_local_apic_id());
}

#[cfg(test)]
#[test]
fn cache_info() {
    let cpu: CpuId = CpuId;
    let cinfos = cpu.get_cache_information();

    for cache in cinfos.iter() {
        if cache.num != 0x0 {
            println!("{}", cache);
        }
    }
}

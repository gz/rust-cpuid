use crate::{CpuId, CpuIdResult};
use phf::phf_map;

/// Raw dump of ryzen mantisse cpuid values.
///
// Key format is (eax << 32 | ecx) e.g., two 32 bit values packed in one 64 bit value
static CPUID_VALUE_MAP: phf::Map<u64, CpuIdResult> = phf_map! {
    0x00000000_00000000u64 => CpuIdResult { eax: 0x00000010, ebx: 0x68747541, ecx: 0x444d4163, edx: 0x69746e65 },
    0x00000001_00000000u64 => CpuIdResult { eax: 0x00870f10, ebx: 0x000c0800, ecx: 0x7ed8320b, edx: 0x178bfbff },
    0x00000002_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x00000003_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x00000005_00000000u64 => CpuIdResult { eax: 0x00000040, ebx: 0x00000040, ecx: 0x00000003, edx: 0x00000011 },
    0x00000006_00000000u64 => CpuIdResult { eax: 0x00000004, ebx: 0x00000000, ecx: 0x00000001, edx: 0x00000000 },
    0x00000007_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x219c91a9, ecx: 0x00400004, edx: 0x00000000 },
    0x00000008_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x00000009_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000a_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000b_00000000u64 => CpuIdResult { eax: 0x00000001, ebx: 0x00000002, ecx: 0x00000100, edx: 0x00000000 },
    0x0000000b_00000001u64 => CpuIdResult { eax: 0x00000007, ebx: 0x0000000c, ecx: 0x00000201, edx: 0x00000000 },
    0x0000000c_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000d_00000000u64 => CpuIdResult { eax: 0x00000207, ebx: 0x00000340, ecx: 0x00000380, edx: 0x00000000 },
    0x0000000d_00000001u64 => CpuIdResult { eax: 0x0000000f, ebx: 0x00000340, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000d_00000002u64 => CpuIdResult { eax: 0x00000100, ebx: 0x00000240, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000d_00000009u64 => CpuIdResult { eax: 0x00000040, ebx: 0x00000340, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000e_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x0000000f_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x000000ff, ecx: 0x00000000, edx: 0x00000002 },
    0x0000000f_00000001u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000040, ecx: 0x000000ff, edx: 0x00000007 },
    0x00000010_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000002, ecx: 0x00000000, edx: 0x00000000 },
    0x00000010_00000001u64 => CpuIdResult { eax: 0x0000000f, ebx: 0x00000000, ecx: 0x00000004, edx: 0x0000000f },
    0x20000000_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000000_00000000u64 => CpuIdResult { eax: 0x80000020, ebx: 0x68747541, ecx: 0x444d4163, edx: 0x69746e65 },
    0x80000001_00000000u64 => CpuIdResult { eax: 0x00870f10, ebx: 0x20000000, ecx: 0x75c237ff, edx: 0x2fd3fbff },
    0x80000002_00000000u64 => CpuIdResult { eax: 0x20444d41, ebx: 0x657a7952, ecx: 0x2035206e, edx: 0x30303633 },
    0x80000003_00000000u64 => CpuIdResult { eax: 0x2d362058, ebx: 0x65726f43, ecx: 0x6f725020, edx: 0x73736563 },
    0x80000004_00000000u64 => CpuIdResult { eax: 0x2020726f, ebx: 0x20202020, ecx: 0x20202020, edx: 0x00202020 },
    0x80000005_00000000u64 => CpuIdResult { eax: 0xff40ff40, ebx: 0xff40ff40, ecx: 0x20080140, edx: 0x20080140 },
    0x80000006_00000000u64 => CpuIdResult { eax: 0x48006400, ebx: 0x68006400, ecx: 0x02006140, edx: 0x01009140 },
    0x80000007_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x0000001b, ecx: 0x00000000, edx: 0x00006799 },
    0x80000008_00000000u64 => CpuIdResult { eax: 0x00003030, ebx: 0x010eb757, ecx: 0x0000700b, edx: 0x00010000 },
    0x80000009_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000a_00000000u64 => CpuIdResult { eax: 0x00000001, ebx: 0x00008000, ecx: 0x00000000, edx: 0x0013bcff },
    0x8000000b_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000c_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000d_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000e_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000000f_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000010_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000011_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000012_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000013_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000014_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000015_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000016_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000017_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000018_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x80000019_00000000u64 => CpuIdResult { eax: 0xf040f040, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001a_00000000u64 => CpuIdResult { eax: 0x00000006, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001b_00000000u64 => CpuIdResult { eax: 0x000003ff, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001c_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001d_00000000u64 => CpuIdResult { eax: 0x00004121, ebx: 0x01c0003f, ecx: 0x0000003f, edx: 0x00000000 },
    0x8000001d_00000001u64 => CpuIdResult { eax: 0x00004122, ebx: 0x01c0003f, ecx: 0x0000003f, edx: 0x00000000 },
    0x8000001d_00000002u64 => CpuIdResult { eax: 0x00004143, ebx: 0x01c0003f, ecx: 0x000003ff, edx: 0x00000002 },
    0x8000001d_00000003u64 => CpuIdResult { eax: 0x00014163, ebx: 0x03c0003f, ecx: 0x00003fff, edx: 0x00000001 },
    0x8000001e_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000100, ecx: 0x00000000, edx: 0x00000000 },
    0x8000001f_00000000u64 => CpuIdResult { eax: 0x0001000f, ebx: 0x0000016f, ecx: 0x000001fd, edx: 0x00000001 },
    0x80000020_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000002, ecx: 0x00000000, edx: 0x00000000 },
    0x80000020_00000001u64 => CpuIdResult { eax: 0x0000000b, ebx: 0x00000000, ecx: 0x00000000, edx: 0x0000000f },
    0x80860000_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
    0xc0000000_00000000u64 => CpuIdResult { eax: 0x00000000, ebx: 0x00000000, ecx: 0x00000000, edx: 0x00000000 },
};

fn cpuid_reader(eax: u32, ecx: u32) -> CpuIdResult {
    let key = (eax as u64) << u32::BITS | ecx as u64;
    CPUID_VALUE_MAP[&key]
}

/// Check that vendor is AuthenticAMD.
#[test]
fn vendor_check() {
    let cpuid = CpuId::with_cpuid_fn(cpuid_reader);
    let v = cpuid.get_vendor_info().expect("Need to find vendor info");
    assert_eq!(v.as_string(), "AuthenticAMD");
}

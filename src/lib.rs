#![feature(no_std)]
#![feature(core)]
#![feature(asm)]
#![no_std]

#![crate_name = "raw-cpuid"]
#![crate_type = "lib"]

#[macro_use]
extern crate core;

#[cfg(test)]
#[macro_use]
extern crate std;

const MAX_ENTRIES: usize = 32;

#[derive(Debug)]
pub struct CpuId {
    values: [CpuIdResult; MAX_ENTRIES]
}

#[derive(Debug, Copy)]
pub struct CpuIdResult {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32
}

impl CpuId {
    pub fn new() -> CpuId {
        let mut cpu = CpuId{ values: [ CpuIdResult{ eax: 0, ebx: 0, ecx: 0, edx: 0}; MAX_ENTRIES] };

        unsafe {
            let mut res: CpuIdResult;
            res = cpuid(0x0);
            cpu.values[0] = res;
            assert!( (res.eax as usize) < MAX_ENTRIES);
            for i in 1..(res.eax as usize) {
                res = cpuid(i as u32);
                cpu.values[i] = res;
            }
        }

        cpu
    }
}

pub unsafe fn cpuid(eax: u32) -> CpuIdResult {
    asm!("movl $0, %eax" : : "r" (eax) : "eax");

    let mut res = CpuIdResult{eax: 0, ebx: 0, ecx: 0, edx: 0};
    asm!("cpuid" : "={eax}"(res.eax) "={ebx}"(res.ebx) "={ecx}"(res.ecx) "={edx}"(res.edx) 
                 :
                 : "eax", "ebx", "ecx", "edx");

    res
}

fn to_bytes(val: u32) -> [u8; 4] {
    let mut res: [u8; 4] = [0; 4];

    res[0] = val as u8;
    res[1] = (val >> 8) as u8;
    res[2] = (val >> 16) as u8;
    res[3] = (val >> 24) as u8;
    res
}

fn to_str(t: [u8; 4]) -> [char; 4] {
    let mut arr: [char; 4] = ['\0'; 4];
    for i in 0..4 {
        arr[i] = t[i] as char;
    }

    arr
}

#[test]
fn genuine_intel() {
    let cpu: CpuId = CpuId::new();

    let b = to_str(to_bytes(cpu.values[0].ebx));
    assert!(b[0] == 'G');
    assert!(b[1] == 'e');
    assert!(b[2] == 'n');
    assert!(b[3] == 'u');
}
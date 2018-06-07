#![feature(start, lang_items, panic_implementation, core_intrinsics)]
#![no_std]

use core::intrinsics;
use core::panic::PanicInfo;

// Pull in the system libc library for what crt0.o likely requires
extern crate libc;
extern crate raw_cpuid;

// Entry point for this program
#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let _c = raw_cpuid::CpuId::new();
    0
}

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

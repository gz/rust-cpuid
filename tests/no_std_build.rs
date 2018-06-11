#![cfg_attr(feature = "nightly", feature(lang_items, core_intrinsics, panic_implementation))]
#![cfg_attr(feature = "nightly", no_std)]
#![cfg_attr(feature = "nightly", no_main)]

// Pull in the system libc library for what crt0.o likely requires.
extern crate libc;
extern crate raw_cpuid;

#[cfg(feature = "nightly")]
use core::panic::PanicInfo;

#[cfg(feature = "nightly")]
#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let _c = raw_cpuid::CpuId::new();
    0
}

#[cfg(not(feature = "nightly"))]
fn main() {
    let _c = raw_cpuid::CpuId::new();
}

#[cfg(feature = "nightly")]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[cfg(feature = "nightly")]
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern "C" fn rust_eh_unwind_resume() {}

#[cfg(feature = "nightly")]
#[panic_implementation]
#[no_mangle]
pub fn panic_impl(_info: &PanicInfo) -> ! {
    loop {}
}

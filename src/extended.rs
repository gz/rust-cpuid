//! Data-structures / interpretation for extended leafs (>= 0x8000_0000)
use core::fmt::{Debug, Formatter};
use core::mem::size_of;
use core::slice;
use core::str;

use crate::CpuIdResult;

/// ASCII string up to 48 characters in length corresponding to the processor name.
/// (LEAF = 0x8000_0002..=0x8000_0004)
pub struct ProcessorBrandString {
    data: [CpuIdResult; 3],
}

impl ProcessorBrandString {
    pub(crate) fn new(data: [CpuIdResult; 3]) -> Self {
        Self { data }
    }

    /// Return the processor brand string as a rust string.
    ///
    /// For example:
    /// "11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz".
    pub fn as_str(&self) -> &str {
        // Safety: CpuIdResult is laid out with repr(C), and the array
        // self.data contains 3 continguous elements.
        let slice: &[u8] = unsafe {
            slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.data.len() * size_of::<CpuIdResult>(),
            )
        };

        // Brand terminated at nul byte or end, whichever comes first.
        let slice = slice.split(|&x| x == 0).next().unwrap();
        str::from_utf8(slice).unwrap_or("Invalid Processor Brand String")
    }
}

impl Debug for ProcessorBrandString {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ProcessorBrandString")
            .field("as_str", &self.as_str())
            .finish()
    }
}

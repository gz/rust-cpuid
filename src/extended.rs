//! Data-structures / interpretation for extended leafs (>= 0x8000_0000)
use core::fmt::{Debug, Formatter};
use core::mem::size_of;
use core::slice;
use core::str;

use crate::{get_bits, CpuIdResult, Vendor};

/// Extended Processor and Processor Feature Identifiers (LEAF=0x8000_0001)
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct ExtendedProcessorFeatureIdentifiers {
    vendor: Vendor,
    eax: u32,
    ebx: u32,
    ecx: ExtendedFunctionInfoEcx,
    edx: ExtendedFunctionInfoEdx,
}

impl ExtendedProcessorFeatureIdentifiers {
    pub(crate) fn new(vendor: Vendor, data: CpuIdResult) -> Self {
        Self {
            vendor,
            eax: data.eax,
            ebx: data.ebx,
            // Safety: Ok we don't care/want to preseve extra bits from CpuId that we don't support (yet)
            ecx: unsafe { ExtendedFunctionInfoEcx::from_bits_unchecked(data.ecx) },
            // Safety: Ok we don't care/want to preseve extra bits from CpuId that we don't support (yet)
            edx: unsafe { ExtendedFunctionInfoEdx::from_bits_unchecked(data.edx) },
        }
    }

    /// Extended Processor Signature.
    ///
    /// # AMD
    /// The value returned is the same as the value returned in EAX for LEAF=0x0000_0001
    /// (use `CpuId.get_feature_info` instead)
    ///
    /// # Intel
    /// Vague mention of "Extended Processor Signature", not clear what it's supposed to represent.
    pub fn extended_signature(&self) -> u32 {
        self.eax
    }

    /// Returns package type on AMD.
    ///
    /// # Intel
    /// This field is not used (reseved).
    ///
    /// # AMD
    /// Package type. If (Family[7:0] >= 10h), this field is valid.
    /// If (Family[7:0]<10h), this field is reserved
    pub fn pkg_type(&self) -> u32 {
        get_bits(self.ebx, 28, 31)
    }

    /// Returns brand ID on AMD.
    ///
    /// # Intel
    /// This field is not used (reserved).
    ///
    /// # AMD
    /// This field, in conjunction with CPUID
    /// LEAF=0x0000_0001_EBX[8BitBrandId], and used by firmware to generate the
    /// processor name string.
    pub fn brand_id(&self) -> u32 {
        get_bits(self.ebx, 0, 15)
    }

    /// Is LAHF/SAHF available in 64-bit mode?
    pub fn has_lahf_sahf(&self) -> bool {
        self.ecx.contains(ExtendedFunctionInfoEcx::LAHF_SAHF)
    }

    /// Check support for 64-bit mode.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_cmp_legacy(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::CMP_LEGACY)
    }

    /// Secure virtual machine supported.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_svm(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::SVM)
    }

    /// Extended APIC space. This bit indicates the presence of extended APIC register space starting at offset 400h from the “APIC Base Address Register,” as specified in the BKDG.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_ext_apic_space(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::EXT_APIC_SPACE)
    }

    /// LOCK MOV CR0 means MOV CR8. See “MOV(CRn)” in APM3.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_alt_mov_cr8(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::ALTMOVCR8)
    }

    /// Is LZCNT available?
    pub fn has_lzcnt(&self) -> bool {
        self.ecx.contains(ExtendedFunctionInfoEcx::LZCNT)
    }

    /// XTRQ, INSERTQ, MOVNTSS, and MOVNTSD instruction support.
    /// See “EXTRQ”, “INSERTQ”, “MOVNTSS”, and “MOVNTSD” in APM4.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_sse4a(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::SSE4A)
    }

    /// Misaligned SSE mode. See “Misaligned Access Support Added for SSE Instructions” in APM1.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_misaligned_sse_mode(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::MISALIGNSSE)
    }

    /// Is PREFETCHW available?
    ///
    /// # AMD
    /// PREFETCH and PREFETCHW instruction support.
    pub fn has_prefetchw(&self) -> bool {
        self.ecx.contains(ExtendedFunctionInfoEcx::PREFETCHW)
    }

    /// Indicates OS-visible workaround support
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_osvw(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::OSVW)
    }

    /// Instruction based sampling.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_ibs(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::IBS)
    }

    /// Extended operation support.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_xop(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::XOP)
    }

    /// SKINIT and STGI are supported.
    ///
    /// Indicates support for SKINIT and STGI, independent of
    /// the value of MSRC000_0080[SVME].
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_skinit(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::SKINIT)
    }

    /// Watchdog timer support.
    ///
    /// Indicates support for MSRC001_0074.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_wdt(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::WDT)
    }

    /// Lightweight profiling support
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_lwp(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::LWP)
    }

    /// Four-operand FMA instruction support.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_fma4(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::FMA4)
    }

    /// Trailing bit manipulation instruction support.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_tbm(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::TBM)
    }

    /// Topology extensions support.
    ///
    /// Indicates support for CPUID Fn8000_001D_EAX_x[N:0]-CPUID Fn8000_001E_EDX.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_topology_extensions(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::TOPEXT)
    }

    /// Processor performance counter extensions support.
    ///
    /// Indicates support for MSRC001_020[A,8,6,4,2,0] and MSRC001_020[B,9,7,5,3,1].
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_perf_cntr_extensions(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::PERFCTREXT)
    }

    /// NB performance counter extensions support.
    ///
    /// Indicates support for MSRC001_024[6,4,2,0] and MSRC001_024[7,5,3,1].
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_nb_perf_cntr_extensions(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::PERFCTREXTNB)
    }

    /// Data access breakpoint extension.
    ///
    /// Indicates support for MSRC001_1027 and MSRC001_101[B:9].
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_data_access_bkpt_extension(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::DATABRKPEXT)
    }

    /// Performance time-stamp counter.
    ///
    /// Indicates support for MSRC001_0280 [Performance Time Stamp Counter].
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_perf_tsc(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::PERFTSC)
    }

    /// Support for L3 performance counter extension.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_perf_cntr_llc_extensions(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::PERFCTREXTLLC)
    }

    /// Support for MWAITX and MONITORX instructions.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_monitorx_mwaitx(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::MONITORX)
    }

    /// Breakpoint Addressing masking extended to bit 31.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_addr_mask_extension(&self) -> bool {
        self.vendor == Vendor::Amd && self.ecx.contains(ExtendedFunctionInfoEcx::ADDRMASKEXT)
    }

    /// Are fast system calls available.
    pub fn has_syscall_sysret(&self) -> bool {
        self.edx.contains(ExtendedFunctionInfoEdx::SYSCALL_SYSRET)
    }

    /// Is there support for execute disable bit.
    pub fn has_execute_disable(&self) -> bool {
        self.edx.contains(ExtendedFunctionInfoEdx::EXECUTE_DISABLE)
    }

    /// AMD extensions to MMX instructions.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_mmx_extensions(&self) -> bool {
        self.vendor == Vendor::Amd && self.edx.contains(ExtendedFunctionInfoEdx::MMXEXT)
    }

    /// FXSAVE and FXRSTOR instruction optimizations.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_fast_fxsave_fxstor(&self) -> bool {
        self.vendor == Vendor::Amd && self.edx.contains(ExtendedFunctionInfoEdx::FFXSR)
    }

    /// Is there support for 1GiB pages.
    pub fn has_1gib_pages(&self) -> bool {
        self.edx.contains(ExtendedFunctionInfoEdx::GIB_PAGES)
    }

    /// Check support for rdtscp instruction.
    pub fn has_rdtscp(&self) -> bool {
        self.edx.contains(ExtendedFunctionInfoEdx::RDTSCP)
    }

    /// Check support for 64-bit mode.
    pub fn has_64bit_mode(&self) -> bool {
        self.edx.contains(ExtendedFunctionInfoEdx::I64BIT_MODE)
    }

    /// 3DNow AMD extensions.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_amd_3dnow_extensions(&self) -> bool {
        self.vendor == Vendor::Amd && self.edx.contains(ExtendedFunctionInfoEdx::THREEDNOWEXT)
    }

    /// 3DNow extensions.
    ///
    /// # Intel
    /// This feature is unavailable on Intel CPUs (will return false).
    pub fn has_3dnow(&self) -> bool {
        self.vendor == Vendor::Amd && self.edx.contains(ExtendedFunctionInfoEdx::THREEDNOW)
    }
}

impl Debug for ExtendedProcessorFeatureIdentifiers {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("ExtendedProcessorFeatureIdentifiers");
        ds.field("extended_signature", &self.extended_signature());

        if self.vendor == Vendor::Amd {
            ds.field("pkg_type", &self.pkg_type());
            ds.field("brand_id", &self.brand_id());
        }
        ds.field("ecx_features", &self.ecx);
        ds.field("edx_features", &self.edx);
        ds.finish()
    }
}

bitflags! {
    #[derive(Default)]
    #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
    struct ExtendedFunctionInfoEcx: u32 {
        const LAHF_SAHF = 1 << 0;
        const CMP_LEGACY =  1 << 1;
        const SVM = 1 << 2;
        const EXT_APIC_SPACE = 1 << 3;
        const ALTMOVCR8 = 1 << 4;
        const LZCNT = 1 << 5;
        const SSE4A = 1 << 6;
        const MISALIGNSSE = 1 << 7;
        const PREFETCHW = 1 << 8;
        const OSVW = 1 << 9;
        const IBS = 1 << 10;
        const XOP = 1 << 11;
        const SKINIT = 1 << 12;
        const WDT = 1 << 13;
        const LWP = 1 << 15;
        const FMA4 = 1 << 16;
        const TBM = 1 << 21;
        const TOPEXT = 1 << 22;
        const PERFCTREXT = 1 << 23;
        const PERFCTREXTNB = 1 << 24;
        const DATABRKPEXT = 1 << 26;
        const PERFTSC = 1 << 27;
        const PERFCTREXTLLC = 1 << 28;
        const MONITORX = 1 << 29;
        const ADDRMASKEXT = 1 << 30;
    }
}

bitflags! {
    #[derive(Default)]
    #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
    struct ExtendedFunctionInfoEdx: u32 {
        const SYSCALL_SYSRET = 1 << 11;
        const EXECUTE_DISABLE = 1 << 20;
        const MMXEXT = 1 << 22;
        const FFXSR = 1 << 24;
        const GIB_PAGES = 1 << 26;
        const RDTSCP = 1 << 27;
        const I64BIT_MODE = 1 << 29;
        const THREEDNOWEXT = 1 << 30;
        const THREEDNOW = 1 << 31;
    }
}

/// ASCII string up to 48 characters in length corresponding to the processor name.
/// (LEAF = 0x8000_0002..=0x8000_0004)
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
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
        // self.data contains 3 contiguous elements.
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

/// L1 Cache and TLB Information.
///
/// # Intel
/// This info is unavailable on Intel CPUs.
pub struct L1CacheTlbInfo {
    data: CpuIdResult,
}

impl L1CacheTlbInfo {
    pub(crate) fn new(data: CpuIdResult) -> Self {
        Self { data }
    }
}

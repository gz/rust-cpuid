use crate::uarch::Core::{Heterogeneous, Homogenous};
use crate::Vendor;
use crate::Vendor::{Intel, Amd};

#[non_exhaustive]
pub enum CoreArch {
    // Not including Intel micro-architecture without CPUID suport, for now.
    // Intel Micro-architectures (with CPUID support)
    i486,
    P5,
    P6,
    NetBurst,
    Prescott,
    PentiumM,
    ModifiedPentiumM,
    Core,
    Nehalem,
    SandyBridge,
    Haswell,
    // Client Skylake Core are pretty much minor refreshes (follow wikipedia)
    // However Server cores have actually undergone more evolution, so distinguish them
    Skylake,
    SkylakeServer,
    CascadeLake,
    CooperLake,
    PalmCove,
    SunnyCove,
    WillowCove,
    CypressCove,
    GoldenCove,
    // Intel Small Cores
    Bonnel,
    Saltwell,
    Silvermont,
    Airmont,
    Goldmont,
    GoldmontPlus,
    Tremont,
    Gracemont,
    Crestmont,
    // Other Intel
    LakeMont,
    // Many Core TODO

    // AMD Micro-architectures (with CPUID support)
    Am486,
    EnhancedAm486,
    Am586,
    K5,
    K6,
    K6_2,
    K6III,
    K7,
    K8, // Hammer
    // K9 was never finished.
    K10, // Llano
    Bobcat,
    Jaguar,
    // Enhanced Jaguar for XboX One X 'Scorpio". (CPUID currently unknown)
    Puma,
    Bulldozer,
    Piledriver,
    SteamRoller,
    Excavator,
    // Zen,
    Zen,
    ZenPlus, // PinnacleRidge, Picasso,
    Zen2,
    Zen3,
    Zen4,
}

#[non_exhaustive]
pub enum Core {
    Homogenous(CoreArch),
    Heterogeneous { P: CoreArch, E: CoreArch },
    // Note : This doesn't work for more complex set-up with three type of cores.
    // Such design do not exist in Intel (ISA) land, but exist in ARM land.
}

#[non_exhaustive]
pub enum UArch {
    // ---- Intel ----
    // i486
    i486, // Note, this may eventually be split,
    // Family 5
    // P5
    P5, // 80501 and P54C aka 80502
    P5MMX,  // 80503 (Pentium MMX)

    // NetBurst aka P68
    Willamette,
    Northwood,
    Prescott,
    CedarMill, // Prescott Die Shrink
    // P6
    P6PentiumPro,
    P6PentiumII,
    P6PentiumIII,
    // Pentium M
    Banias,
    Dothan,
    Tolapai,
    // Modified Pentium M
    Yonah,
    Merom,
    Penryn,
    Nehalem,
    Westmere,
    SandyBridge,
    IvyBridge,
    IvyBridgeE,
    Haswell,
    HaswellE,
    Broadwell,
    Skylake,
    SkylakeServer,
    KabyLake,
    CascadeLake,
    CoffeeLake,
    CooperLake,
    CannonLake,
    WhiskeyLake,
    AmberLake,
    CometLake,
    IceLake,
    IceLakeServer,
    TigerLake,
    RocketLake,
    AlderLake,
    SapphireRapids,
    RaptorLake,
    EmeraldRapids,
    MeteorLake,

    // Small Cores
    // No split for now.
    Bonnel,
    Saltwell,
    Silvermont,
    Airmont,
    Goldmont,
    GoldmontPlus,
    Tremont,
    Gracemont,
    Crestmont,

    // Lakemont
    Quark,
    // Many Core / Xeon Phi (TODO)
    // Polaris is not x86 ISA
    // Larrabee
    // RockCreek,
    KnightsFerry,
    KnightsCorner,
    KnightsLanding,
    KnightsMill,

    // ---- AMD ----
    Am486,
    EnhancedAm486,
    Am586,
    K5,
    K6,
    K6_2,
    K6III,
    K7,
    K8, // Hammer
    // K9 was never finished.
    K10, // Llano
    Bobcat,
    Jaguar,
    // Enhanced Jaguar for XboX One X 'Scorpio". (CPUID currently unknown)
    Puma,
    Bulldozer,
    Piledriver,
    SteamRoller,
    Excavator,
    // Zen,
    Zen,
    HygonDhyana,
    ZenPlus, // PinnacleRidge, Picasso,
    Zen2,
    Zen3,
    Zen4,

}

pub struct MicroArchitecture {
    pub vendor: Vendor,
    pub cores: Core,
    pub codename: UArch,
    // This currently leaves off the table the exact variant (e.g Amber Lake U).
}


// Source for the tables :
//
// Intel Software Developper Manual (and change document)
//
// https://en.wikichip.org/wiki/intel/cpuid
// https://en.wikichip.org/wiki/amd/cpuid
//
// https://en.wikipedia.org/wiki/List_of_Intel_CPU_microarchitectures
// https://en.wikipedia.org/wiki/List_of_AMD_CPU_microarchitectures


// ===================
// Micro-Architectures
// ===================
const INTEL_486: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::i486),
    codename: UArch::i486,
};
const INTEL_P5: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::P5),
    codename: UArch::P5,
};

const INTEL_P5MMX: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::P5),
    codename: UArch::P5MMX,
};

const INTEL_P6_PENTIUM_PRO: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::P6),
    codename: UArch::P6PentiumPro,
};

const INTEL_P6_PENTIUM_II: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::P6),
    codename: UArch::P6PentiumII,
};

const INTEL_P6_PENTIUM_III: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::P6),
    codename: UArch::P6PentiumIII,
};

const INTEL_WILLAMETTE: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::NetBurst),
    codename: UArch::Willamette,
};
const INTEL_NORTHWOOD: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::NetBurst),
    codename: UArch::Northwood,
};
const INTEL_PRESCOTT: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::Prescott),
    codename: UArch::Prescott,
};

const INTEL_CEDARMILL: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::Prescott),
    codename: UArch::CedarMill,
};


const INTEL_SKYLAKE: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Homogenous(CoreArch::Skylake),
    codename: UArch::Skylake,
};
const INTEL_ALDER_LAKE: MicroArchitecture = MicroArchitecture {
    vendor: Intel,
    cores: Heterogeneous { P: CoreArch::GoldenCove, E: CoreArch::Gracemont },
    codename: UArch::AlderLake,
};


const MICRO_ARCHITECTURE_LIST: [&'static MicroArchitecture; 12] = [
    &INTEL_486,
    &INTEL_P5,
    &INTEL_P5MMX,
    &INTEL_P6_PENTIUM_PRO,
    &INTEL_P6_PENTIUM_II,
    &INTEL_P6_PENTIUM_III,
    &INTEL_WILLAMETTE,
    &INTEL_NORTHWOOD,
    &INTEL_PRESCOTT,
    &INTEL_CEDARMILL,
    &INTEL_SKYLAKE,
    &INTEL_ALDER_LAKE
];

// ================
// PARSING
// ================

pub fn identify_micro_architecture(vendor: Vendor, family: u8, model: u8, stepping: u8) -> Option<&'static MicroArchitecture> {
    let family_model = (family as u16) << 8 + model;
    match vendor {
        Intel => match family_model {
            0x04_01 | 0x04_02 | 0x04_03 | 0x04_04 | 0x04_05 | 0x04_07 | 0x04_08 | 0x04_09 => Some(&INTEL_486),
            0x05_01 | 0x05_02 => Some(&INTEL_P5),
            0x05_04 | 0x05_07 => Some(&INTEL_P5MMX),
            0x05_09 => match stepping {
                0 => /* Quark X1000*/ None,
                _ => None
            },
            /* P6
            Pentium Pro : 0F619h

            Pentium II : Klamath: 80522 In Intel's "Family/Model/Stepping" scheme, Klamath CPUs are family 6, model 3.
                         Deschutes and Tonga: 80523 In Intel's "Family/Model/Stepping" scheme, Deschutes CPUs are family 6, model 5 and have the part number 80523.
                         Dixon: 80524 In Intel's "Family/Model/Stepping" scheme, Dixon CPUs are family 6, model 6 and their Intel product code is 80524. These identifiers are shared with the Mendocino Celeron processors.

            In Intel's "Family/Model/Stepping" scheme, the Pentium II OverDrive CPU is family 6, model 3. Though it was based on the Deschutes core, when queried by the CPUID command, it identified as a Klamath Pentium II.[5] As noted in the Pentium II Processor update documentation from Intel, "although this processor has a CPUID of 163xh, it uses a Pentium II processor CPUID 065xh processor core."[6]

            Pentium III
            	Katmai: 80525
            	Coppermine: 80526
            	Coppermine T: 80533
            	Tualatin: 80530

                Tualatin 	0 	0x6 	0x0 	0xB 	Family 6 Model 11
                Coppermine, Coppermine T 	0 	0x6 	0x0 	0x8 	Family 6 Model 8
                Katmai 	0 	0x6 	0x0 	0x7 	Family 6 Model 7

                06_7H, 06_08H, 06_0AH, 06_0BH Intel Pentium III Xeon processor, Intel Pentium III processor
            */
            0x06_01 => Some(&INTEL_P6_PENTIUM_PRO),
            0x06_03 | 0x06_05 | 0x06_06 => Some(&INTEL_P6_PENTIUM_II),
            0x06_07 | 0x06_08 | 0x06_0A | 0x06_0B => Some(&INTEL_P6_PENTIUM_III),
            /*
            https://en.wikipedia.org/wiki/List_of_Intel_Pentium_4_processors

            Willamette (180 nm) Intel Family 15 Model 1 Steppings: B2, C1, D0, E0
            Northwood (130 nm) Intel Family 15 Model 2 Steppings: B0, C1, D1, M0
            Prescott : 90nm Intel Family 15 Model 3 (C0, D0), Intel Family 15 Model 4 (E0, G1)
            Prescott 2M (90 nm) Intel Family 15 Model 4 (N0, R0)
            Cedar Mill (65 nm) Intel Family 15 Model 6 (B1, C1, D0)

            Pentium 4 Extreme Edition Gallatin (130 nm) M0 -> Northwood
            Pentium 4-M Northwood (130 nm) B0, B0 Shrink, C1, D1
            Mobile Pentium 4 Northwood (130 nm)
            Mobile Pentium 4 HT Northwood (130 nm)
            Mobile Pentium 4 HT  Prescott (90 nm) D0, E0


            https://en.wikipedia.org/wiki/List_of_Intel_Pentium_D_processors

            Pentium D "Smithfield" (90 nm) A0, B0
            Pentium D "Presler" (MCP, 65 nm) B1, C1, D0

            https://en.wikipedia.org/wiki/Pentium_D

            0F_04 (7) (Smithfield)
            0F_06 (5) (Presler)

            https://en.wikipedia.org/wiki/Xeon

            Gallatin (Northwood) 0F7x
            Paxville (90nm Prescott) 0F48
            Tulsa (Presler / Cedar Mill 65nm) 0F68
            */

            0x0F_00 | 0x0F_01 => Some(&INTEL_WILLAMETTE), /* Willamette (0F_01), Intel Xeon Processor, Intel Xeon processor MP, Intel Pentium 4 processors*/
            0x0F_02 => Some(&INTEL_NORTHWOOD), /* Northwood Intel Xeon Processor, Intel Xeon processor MP, Intel Pentium 4 processors*/
            0x0F_03 | 0x0F_04 => Some(&INTEL_PRESCOTT), /*Prescott Intel Xeon processor, Intel Xeon processor MP, Intel Pentium 4, Pentium D processors*/
            0x0F_06 => Some(&INTEL_CEDARMILL), /* CedarMill Netburst Intel Xeon processor 7100, 5000 Series, Intel Xeon Processor MP, Intel Pentium 4, Pentium D
processors*/
            _ => None,
        },
        Amd => match family_model {
            _ => None
        },
        _ => None,
    }
}
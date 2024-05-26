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
    Williamette,
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


const MICRO_ARCHITECTURE_LIST: [&'static MicroArchitecture] = *[
    &INTEL_486,
    &INTEL_P5,
    &INTEL_P5MMX,
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
            _ => None,
        },
        Amd => match family_model {
            _ => None
        },
        _ => None,
    }
}
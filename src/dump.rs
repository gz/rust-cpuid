use std::collections::HashMap;
use crate::{CpuIdResult, CpuIdReader, CpuIdWriter};

#[derive(Clone)]
enum LeafOrSubleaves {
    Leaf(CpuIdResult),
    Subleaf(HashMap<u32, CpuIdResult>),
}

// TODO: Clone is necessary because CpuIdReader wants it (for leaves with more complex subleaf
// structures, like the extended topology info leaf)
//
// This implies that there's a full clone of the dump held on for those leaf-specific views, which
// is unfortunate! It's also not yet really clear how to assemble those more complex leaves for
// writer purposes.
#[derive(Clone)]
pub struct CpuIdDump {
    leaves: HashMap<u32, LeafOrSubleaves>,
}

impl CpuIdDump {
    // TODO: probably should just take vendor in the initial constructor here
    // (that also lets this pick the right leaf/subleaf fallback behavior from the get-go)
    pub fn new() -> Self {
        Self {
            leaves: HashMap::new(),
        }
    }
}

const DEFAULT_LEAF: CpuIdResult = CpuIdResult {
    eax: 0,
    ebx: 0,
    ecx: 0,
    edx: 0,
};

impl CpuIdWriter for CpuIdDump {
    fn set_leaf(&mut self, leaf: u32, mut bits: Option<CpuIdResult>) {
        // Many bits in 8000_0001h EDX, if present, mirror leaf 1h EDX. Maintain that before
        // storing the bits.
        if let Some(bits) = bits.as_mut() {
            const MIRROR_MASK: u32 = 0b0000_0001_1000_0011_1111_0011_1111_1111;

            let update = if leaf == 0x0000_0001 {
                // We're updating leaf 1h, so go fix up leaf 8000_0001h (if present)
                match self.leaves.get_mut(&0x8000_0001) {
                    Some(LeafOrSubleaves::Leaf(ext_info)) => {
                        ext_info.edx &= !MIRROR_MASK;
                        ext_info.edx |= bits.edx & MIRROR_MASK;
                    }
                    Some(_) => {
                        panic!("extended feature information leaf (8000_0001h) had subleaves?");
                    }
                    None => {
                        // No leaf 8000_0001h to mirror to (yet?)
                    }
                }
            } else if leaf == 0x8000_0001 {
                match self.leaves.get(&0x0000_0001) {
                    Some(LeafOrSubleaves::Leaf(prior_bits)) => {
                        bits.edx &= !MIRROR_MASK;
                        bits.edx |= prior_bits.edx & MIRROR_MASK;
                    }
                    Some(_) => {
                        panic!("feature information leaf (01h) had subleaves?");
                    }
                    None => {
                        // No leaf 1h to shadow (yet?)
                    }
                }
            };
        }

        if let Some(bits) = bits {
            self.leaves.insert(leaf, LeafOrSubleaves::Leaf(bits));
        } else {
            self.leaves.remove(&leaf);
        }

        self.update_max_leaves();
    }
    fn set_subleaf(&mut self, leaf: u32, subleaf: u32, bits: Option<CpuIdResult>) {
        if let Some(bits) = bits {
            match self
                .leaves
                .entry(leaf)
                .or_insert(LeafOrSubleaves::Subleaf(HashMap::new()))
            {
                LeafOrSubleaves::Leaf(_) => {
                    panic!("adding a subleaf where there's a leaf. no");
                }
                LeafOrSubleaves::Subleaf(leaves) => {
                    leaves.insert(subleaf, bits);
                }
            }
        } else {
            self.leaves.get_mut(&leaf).map(|ent| {
                if let LeafOrSubleaves::Subleaf(leaves) = ent {
                    leaves.remove(&subleaf);
                } else {
                    panic!("removing a subleaf when there's a leaf. no");
                }
            });
        }

        self.update_max_leaves();
    }
}

impl CpuIdDump {
    fn update_max_leaves(&mut self) {
        let mut max_standard = None;
        let mut max_hv = None;
        let mut max_extended = None;

        for (idx, k) in self.leaves.keys().enumerate() {
            let k = *k;
            if k < 0x40000000 {
                max_standard = Some(match max_standard {
                    None => k,
                    Some(prev) => core::cmp::max(k, prev),
                });
            } else if k < 0x80000000 {
                max_hv = Some(match max_hv {
                    None => k,
                    Some(prev) => core::cmp::max(k, prev),
                });
            } else if k < 0xc0000000 {
                max_extended = Some(match max_extended {
                    None => k,
                    Some(prev) => core::cmp::max(k, prev),
                });
            }
        }

        if let Some(eax) = max_standard {
            match self
                .leaves
                .entry(0)
                .or_insert(LeafOrSubleaves::Leaf(CpuIdResult::empty()))
            {
                LeafOrSubleaves::Leaf(leaf) => {
                    leaf.eax = eax;
                }
                _ => {
                    panic!("cannot update leaf 1.EAX: leaf has subleaves?");
                }
            }
        }

        if let Some(eax) = max_hv {
            match self
                .leaves
                .entry(0x40000000)
                .or_insert(LeafOrSubleaves::Leaf(CpuIdResult::empty()))
            {
                LeafOrSubleaves::Leaf(leaf) => {
                    leaf.eax = eax;
                }
                _ => {
                    panic!("cannot update leaf 0x40000000.EAX: leaf has subleaves?");
                }
            }
        }

        if let Some(eax) = max_extended {
            match self
                .leaves
                .entry(0x80000000)
                .or_insert(LeafOrSubleaves::Leaf(CpuIdResult::empty()))
            {
                LeafOrSubleaves::Leaf(leaf) => {
                    leaf.eax = eax;
                }
                _ => {
                    panic!("cannot update leaf 0x80000000.EAX: leaf has subleaves?");
                }
            }
        }
    }
}

impl CpuIdReader for CpuIdDump {
    fn cpuid1(&self, leaf: u32) -> CpuIdResult {
        match self.leaves.get(&leaf) {
            Some(LeafOrSubleaves::Leaf(res)) => *res,
            Some(LeafOrSubleaves::Subleaf(subleaves)) => {
                *subleaves.get(&0).unwrap_or_else(|| {
                    // TODO: vendor-specific fallback behavior
                    &DEFAULT_LEAF
                })
            }
            None => {
                // TODO: more vendor-specific fallback behavior
                DEFAULT_LEAF
            }
        }
    }

    fn cpuid2(&self, leaf: u32, subleaf: u32) -> CpuIdResult {
        match self.leaves.get(&leaf) {
            Some(LeafOrSubleaves::Leaf(res)) => {
                // TODO: vendor-specific fallback behavior
                DEFAULT_LEAF
            }
            Some(LeafOrSubleaves::Subleaf(subleaves)) => {
                *subleaves.get(&subleaf).unwrap_or_else(|| {
                    // TODO: vendor-specific fallback behavior
                    &DEFAULT_LEAF
                })
            }
            None => {
                // TODO: more vendor-specific fallback behavior
                DEFAULT_LEAF
            }
        }
    }
}

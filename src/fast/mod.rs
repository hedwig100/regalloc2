use crate::cfg::CFGInfo;
use crate::ion;
use crate::ssa::validate_ssa;
use crate::{Allocation, Edit, Function, MachineEnv, Output, ProgPoint, RegAllocError, RegClass};
use lru_cache::LruCache;
use std::result::Result;
use std::vec::Vec;

struct SpillSlotData {
    pub class: RegClass,
    pub slots: u32, // offset from the base of the stack frame
}

/// Return the size of a register class in bytes.
fn regclass_size(regclass: RegClass) -> u32 {
    match regclass {
        RegClass::Int => 4,
        RegClass::Float => 8,
        RegClass::Vector => 8,
    }
}

#[derive(Clone, Debug)]
pub struct Edits {
    edits: Vec<(ProgPoint, Edit)>,
}

impl Edits {
    pub fn new() -> Self {
        Self { edits: Vec::new() }
    }

    pub fn push(&mut self, point: ProgPoint, edit: Edit) {
        self.edits.push((point, edit));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(ProgPoint, Edit)> {
        self.edits.iter()
    }

    pub fn to_vec(self) -> Vec<(ProgPoint, Edit)> {
        self.edits
    }
}

struct Env<'a, F: Function> {
    pub func: &'a F,
    pub env: &'a MachineEnv,
    pub cfginfo: CFGInfo,

    /// Current slot number for the stack frame.
    /// This is used to assign spill slots.
    pub current_slot: usize,

    /// Spill slots for each register.
    pub spillslots: Vec<SpillSlotData>,

    /// Cache for virtual register to spill slot mapping.
    /// This is used to select which variable to spill
    /// when we run out of registers.
    pub vreg_cache: LruCache<u32, u32>,

    // Output:
    pub allocs: Vec<Allocation>,
    pub inst_alloc_offsets: Vec<u32>,
    pub num_spillslots: u32,

    pub enable_annotations: bool,
}

impl<'a, F: Function> Env<'a, F> {
    pub(crate) fn new(
        func: &'a F,
        env: &'a MachineEnv,
        cfginfo: CFGInfo,
        enable_annotations: bool,
    ) -> Self {
        Self {
            func,
            env,
            cfginfo,

            current_slot: 0,
            spillslots: Vec::new(),
            vreg_cache: LruCache::new(128),

            allocs: Vec::new(),
            inst_alloc_offsets: Vec::new(),
            num_spillslots: 0,

            enable_annotations,
        }
    }

    pub(crate) fn run(&mut self) -> Result<Edits, RegAllocError> {
        // TODO: implement register allocation here
        Ok(Edits::new())
    }
}

pub fn run<F: Function>(
    func: &F,
    mach_env: &MachineEnv,
    enable_annotations: bool,
    enable_ssa_checker: bool,
) -> Result<Output, RegAllocError> {
    let cfginfo = CFGInfo::new(func)?;

    if enable_ssa_checker {
        validate_ssa(func, &cfginfo)?;
    }

    let mut env = Env::new(func, mach_env, cfginfo, enable_annotations);
    let edits = env.run()?;

    Ok(Output {
        num_spillslots: env.num_spillslots as usize,
        edits: edits.to_vec(),
        allocs: env.allocs,
        inst_alloc_offsets: env.inst_alloc_offsets,

        // TODO: implement the rest of the fields
        safepoint_slots: Vec::new(),
        debug_locations: Vec::new(),
        stats: ion::Stats::default(),
    })
}

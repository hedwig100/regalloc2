use crate::ion;
use crate::{Function, MachineEnv, Output, RegAllocError};
use std::vec;

pub fn run<F: Function>(
    func: &F,
    mach_env: &MachineEnv,
    enable_annotations: bool,
    enable_ssa_checker: bool,
) -> Result<Output, RegAllocError> {
    Ok(Output {
        num_spillslots: 0,
        edits: vec![],
        allocs: vec![],
        inst_alloc_offsets: vec![],
        safepoint_slots: vec![],
        debug_locations: vec![],
        stats: ion::Stats::default(),
    })
}

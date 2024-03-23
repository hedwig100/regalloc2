use super::regclass_size;
use crate::fast::{Env, SpillSlotData};
use crate::{
    Block, Function, Inst, Operand, OperandConstraint, OperandKind, RegAllocError, RegClass, VReg,
};

impl<'a, F: Function> Env<'a, F> {
    pub(crate) fn run(&mut self) -> Result<(), RegAllocError> {
        self.cfginfo
            .postorder
            .into_iter()
            .map(|block| self.process_block(block))
            .collect::<Result<(), RegAllocError>>()?;
        Ok(())
    }

    fn process_block(&mut self, block: Block) -> Result<(), RegAllocError> {
        let entry_progpoint = self.cfginfo.block_entry[block.index()];
        let exit_progpoint = self.cfginfo.block_exit[block.index()];
        let mut inst = entry_progpoint.inst();
        while inst != exit_progpoint.inst() {
            self.process_inst(inst)?;
            inst = inst.next();
        }
        Ok(())
    }

    fn process_inst(&mut self, inst: Inst) -> Result<(), RegAllocError> {
        for &op in self.func.inst_operands(inst) {
            // TODO: before iterating all operands,
            // we should check if the operand is a fixed register or late-use
            // or early-def operand because we cannot use these register
            // as register of other operands. We can handle then
            // by computing pre-determined registers
            match op.kind() {
                OperandKind::Use => self.use_op(op)?,
                OperandKind::Def => self.def_op(op)?,
            }
        }
        Ok(())
    }

    fn use_op(&mut self, op: Operand) -> Result<(), RegAllocError> {
        match op.constraint() {
            OperandConstraint::Any => {}
            OperandConstraint::Reg => {}
            OperandConstraint::Stack => {}
            OperandConstraint::FixedReg(preg) => {}
            OperandConstraint::Reuse(_) => unreachable!(),
        }
        Ok(())
    }

    fn def_op(&mut self, op: Operand) -> Result<(), RegAllocError> {
        match op.constraint() {
            OperandConstraint::Any => {}
            OperandConstraint::Reg => {}
            OperandConstraint::Stack => {}
            OperandConstraint::FixedReg(preg) => {}
            OperandConstraint::Reuse(reg_id) => {}
        }
        Ok(())
    }

    fn add_variable(&mut self, vreg: VReg, regclass: RegClass) {
        self.spillslots[vreg.vreg()] = SpillSlotData::new(regclass, self.current_slot);
        self.current_slot += regclass_size(regclass);
    }
}

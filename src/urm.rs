use std::{cmp::max, collections::HashMap};

type I = Instruction;
pub type Program = Vec<I>;
type RegisterIndex = usize;
type RegisterValue = usize;
type InstructionIndex = usize;
pub type Registers = HashMap<RegisterIndex, RegisterValue>;
pub const EXIT: usize = usize::MAX;

pub mod primitives;

pub fn register_history(urm: &mut URM) -> Vec<Registers> {
    urm.into_iter().collect()
}

pub fn execute(urm: &mut URM) -> Registers {
    urm.into_iter().last().unwrap_or_default()
}

// TODO
// Unit test
fn _max_register_value(program: &[I]) -> RegisterValue {
    program
        .iter()
        .map(|ins| {
            *(match ins {
                Instruction::Z(reg_idx) => reg_idx,
                Instruction::S(reg_idx) => reg_idx,
                Instruction::T(reg_idx_0, reg_idx_1) => max(reg_idx_0, reg_idx_1),
                Instruction::J(reg_idx_0, reg_idx_1, _) => max(reg_idx_0, reg_idx_1),
            })
        })
        .max()
        .unwrap_or_default()
}

#[derive(Debug)]
pub struct URM {
    pub registers: Registers,
    pub instructions: Program,
    pub program_counter: usize,
}

impl URM {
    // TODO
    // Make pub version of this.
    fn value_of_register(&mut self, index: usize) -> usize {
        *self.registers.entry(index).or_insert(0)
    }

    fn step(&mut self) -> Option<Registers> {
        //If there is an instruction at the index, execute it.
        if let Some(instruction) = self.next_instruction() {
            let mut next_pc = self.program_counter + 1;
            match instruction {
                Instruction::S(reg_idx) => *(self.registers.entry(reg_idx).or_insert(0)) += 1,
                Instruction::Z(reg_idx) => {
                    self.registers.insert(reg_idx, 0);
                }
                Instruction::T(reg_idx_0, reg_idx_1) => {
                    let reg_val_0 = self.value_of_register(reg_idx_0);
                    self.registers.insert(reg_idx_1, reg_val_0);
                }
                Instruction::J(reg_idx_0, reg_idx_1, ins_idx) => {
                    if self.value_of_register(reg_idx_0) == self.value_of_register(reg_idx_1) {
                        next_pc = ins_idx
                    };
                }
            };
            self.program_counter = next_pc;
            Some(self.registers.clone())
        } else {
            None
        }
    }

    fn next_instruction(&self) -> Option<Instruction> {
        self.instructions.get(self.program_counter).copied()
    }
}

impl Iterator for URM {
    type Item = Registers;
    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

impl Default for URM {
    fn default() -> Self {
        Self {
            registers: HashMap::new(),
            instructions: Vec::new(),
            program_counter: 0,
        }
    }
}

impl From<Program> for URM {
    fn from(program: Program) -> Self {
        Self {
            instructions: program,
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    S(RegisterIndex),
    Z(RegisterIndex),
    T(RegisterIndex, RegisterIndex),
    J(RegisterIndex, RegisterIndex, InstructionIndex),
}

#[cfg(test)]
mod urm_tests;

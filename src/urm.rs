
use std::collections::HashMap;

type I = Instruction;
pub type Program = Vec<I>;
type Index = usize;
type Value = usize;
pub type Registers = HashMap<Index, Value>;

pub fn execute(urm: URM) -> Option<usize> {
    urm.into_iter().last()
}

#[derive(Debug)]
pub struct URM {
    pub registers: Registers,
    pub instructions: Program,
    pub program_counter: usize,
}

impl URM {
    fn step(&mut self) -> Option<&Registers> {
        //If there is an instruction at the index, execute it.
        if let Some(instruction) = self.next_instruction() {
            let mut next_pc = self.program_counter + 1;
            match instruction {
                Instruction::S(reg) => *(self.registers.entry(reg).or_insert(0)) += 1,
                Instruction::Z(reg) => {
                    self.registers.insert(reg, 0);
                }
                Instruction::T(reg1, reg2) => {
                    let r1 = self.registers.entry(reg1).or_insert(0).clone();
                    self.registers.insert(reg2, r1);
                }
                Instruction::J(reg1, reg2, reg3) => {
                    if reg1 == reg2 {
                        next_pc = reg3
                    };
                }
            };
            self.program_counter = next_pc;
            Some(&self.registers)
        } else {
            None
        }
    }

    fn next_instruction(&self) -> Option<Instruction> {
        match self.program_counter {
            0 => None,
            _ => self
                .instructions
                .get(self.program_counter)
                .map(|ins| ins.clone()),
        }
    }
}

impl Iterator for URM {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.step()
            .map(|reg| reg.get(&1))
            .flatten()
            .map(|first| first.clone())
    }
}

impl Default for URM {
    fn default() -> Self {
        Self {
            registers: HashMap::new(),
            instructions: Vec::new(),
            program_counter: 1,
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

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    S(usize),
    Z(usize),
    T(usize, usize),
    J(usize, usize, usize),
}

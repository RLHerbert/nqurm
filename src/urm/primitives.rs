use super::{Instruction::*, EXIT, URM};

impl URM {
    pub fn add(lhs: usize, rhs: usize) -> Self {
        Self {
            instructions: vec![J(1, 2, EXIT), S(0), S(2), J(0, 0, 0)],
            registers: vec![(0, lhs), (1, rhs)].into_iter().collect(),
            ..URM::default()
        }
    }
}

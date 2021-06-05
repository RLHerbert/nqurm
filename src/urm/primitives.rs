use super::{EXIT, I, URM};

impl URM {
    pub fn add(lhs: usize, rhs: usize) -> Self {
        Self {
            instructions: vec![I::J(1, 2, EXIT), I::S(0), I::S(2), I::J(0, 0, 0)],
            registers: vec![(0, lhs), (1, rhs)].into_iter().collect(),
            ..URM::default()
        }
    }
}

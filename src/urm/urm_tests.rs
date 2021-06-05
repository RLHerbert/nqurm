
use super::{execute, Instruction::*, EXIT, URM};
#[test]
fn s_0_increments_register_0() {
    let mut urm = URM {
        instructions: vec![S(0)],
        ..URM::default()
    };

    assert_eq!(
        1usize,
        urm.next().unwrap_or_default().get(&0).unwrap().clone()
    );
    assert_eq!(None, urm.next());

    // STATUS: PASS
}

// Helper function
fn add_5_and_8() -> URM {
    URM {
        instructions: vec![J(1, 2, EXIT), S(0), S(2), J(0, 0, 0)],
        registers: vec![(0, 5), (1, 8)].into_iter().collect(),
        ..URM::default()
    }
}

#[test]
fn add_5_and_8_equals_13() {
    let mut urm = add_5_and_8();
    assert_eq!(13usize, execute(&mut urm).get(&0).unwrap().clone());

    println!("{:?}", urm);

    // STATUS: PASS
}

#[test]
fn add_5_and_8_second_instruction_is_1() {
    let mut urm = add_5_and_8();
    urm.next();
    assert_eq!(urm.program_counter, 1);
    urm.next();
    assert_eq!(urm.program_counter, 2);
    urm.next();
    assert_eq!(urm.program_counter, 3);
    urm.next();
    assert_eq!(urm.program_counter, 0);

    println!("{:?}", urm);

    // STATUS: PASS
}

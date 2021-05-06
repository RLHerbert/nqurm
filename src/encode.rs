// TODO
// Incorporate urm.rs

use super::urm::Instruction as Inst;
use std::cmp::min;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Inst {
//     S(usize),
//     Z(usize),
//     T(usize, usize),
//     J(usize, usize, usize),
// }

pub type Program = Vec<Inst>;

pub fn pi(x: usize, y: usize) -> usize {
    (2usize.pow(x as u32) * ((2 * y) + 1)) - 1
}

pub fn pi_inv(n: usize) -> (usize, usize) {
    let mut n = n + 1;
    let x: usize = (1..)
        .take_while(|pow| n % 2usize.pow(*pow as u32) == 0)
        .last()
        .unwrap_or_default();
    n /= 2usize.pow(x as u32);
    let y = (n - 1) / 2;

    (x, y)
}

pub fn xi(x: usize, y: usize, z: usize) -> usize {
    pi(pi(x, y), z)
}

pub fn xi_inv(n: usize) -> (usize, usize, usize) {
    let (temp, z) = pi_inv(n);
    let (x, y) = pi_inv(temp);
    (x, y, z)
}

pub fn tau(a_s: Vec<usize>) -> usize {
    a_s.iter()
        .fold((0, 0, 0), |(out, sup, k), x| {
            (out + 2usize.pow((sup + x) as u32 + k), sup + x, k + 1)
        })
        .0
        - 1
}

pub fn tau_inv(a: usize) -> Vec<usize> {
    let mut a2 = a + 1;
    // println!("{}", a2);
    let mut v: Vec<usize> = Vec::new();
    while a2 > 0 {
        let (exp, largest_pow_2) = (1..)
            .enumerate()
            .map(|(e, x)| (e + 1, 2usize.pow(x)))
            .take_while(|(_e, pot)| pot <= &a2)
            .last()
            .unwrap();
        // exp += 1;
        if let Some(x) = v.last_mut() {
            *x -= exp as usize;
        }

        // println!("{}", exp as usize);

        v.push(exp as usize);
        a2 -= largest_pow_2;
    }

    // println!("{:?}", &v);

    v.into_iter()
        .rev()
        .enumerate()
        .map(|(exp, out)| out - min(1, exp as usize))
        .collect()
}

pub fn beta(instruction: &Inst) -> usize {
    match instruction {
        Inst::Z(n) => 4 * (n - 1),
        Inst::S(n) => 4 * (n - 1) + 1,
        Inst::T(m, n) => 4 * pi(m - 1, n - 1) + 2,
        Inst::J(m, n, q) => 4 * xi(m - 1, n - 1, q - 1) + 3,
    }
}

pub fn beta_inv(a: usize) -> Inst {
    // println!("a: {}", a);
    match a % 4 {
        0 => Inst::Z((a / 4) + 1),
        1 => Inst::S((a - 1) / 4 + 1),
        2 => {
            let (x, y) = pi_inv((a - 2) / 4);
            Inst::T(x + 1, y + 1)
        }
        3 => {
            let (x, y, z) = xi_inv((a - 3) / 4);
            Inst::J(x + 1, y + 1, z + 1)
        }
        _ => unreachable!(),
    }
}

pub fn gamma(intcs: Vec<Inst>) -> usize {
    tau(intcs.iter().map(|i| beta(i)).collect())
}

pub fn gamma_inv(gn: usize) -> Vec<Inst> {
    tau_inv(gn).into_iter().map(beta_inv).collect()
}

#[test]
fn pi_works() {
    assert_eq!(pi(3, 8), 135)
}

#[test]
fn pi_inv_works() {
    assert_eq!(pi_inv(135), (3, 8));
}

#[test]
fn xi_works() {
    // TODO
}

#[test]
fn xi_inv_works() {
    assert_eq!((0, 1, 70), xi_inv(563));
}

#[test]
fn tau_works() {
    let v = vec![1, 2];
    assert_eq!(2usize.pow(1) + 2usize.pow(4) - 1, tau(v));
    let v = vec![5, 8, 4, 2, 4];
    assert_eq!(138952735, tau(v));
}

#[test]
fn tau_inv_works() {
    assert_eq!(vec![5, 6], tau_inv(4127));
    assert_eq!(vec![2, 0, 4, 1, 1], tau_inv(5387));
}

#[test]
fn beta_works() {
    assert_eq!(8, beta(&Inst::Z(3)));
    assert_eq!(13, beta(&Inst::S(4)));
    assert_eq!(10, beta(&Inst::T(1, 2)));
    assert_eq!(207, beta(&Inst::J(1, 2, 7)));
}

#[test]
fn beta_inv_works() {
    let z = Inst::Z(3);
    let s = Inst::S(4);
    let t = Inst::T(1, 2);
    let j = Inst::J(1, 2, 7);

    assert_eq!(z, beta_inv(beta(&z)));
    assert_eq!(s, beta_inv(beta(&s)));
    assert_eq!(t, beta_inv(beta(&t)));
    assert_eq!(j, beta_inv(beta(&j)));
}

#[test]
fn gamma_works() {
    assert_eq!(
        2usize.pow(18) + 2usize.pow(32) + 2usize.pow(53) - 1,
        gamma(vec![Inst::T(1, 3), Inst::S(4), Inst::Z(6)])
    );
}

#[test]
fn gamma_inv_works() {
    assert_eq!(
        gamma_inv(2usize.pow(18) + 2usize.pow(32) + 2usize.pow(53) - 1),
        vec![Inst::T(1, 3), Inst::S(4), Inst::Z(6)]
    )
}

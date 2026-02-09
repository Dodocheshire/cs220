//! Semiring

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use itertools::Itertools;

/// Semiring.
///
/// Consult <https://en.wikipedia.org/wiki/Semiring>.
pub trait Semiring: Debug + Clone + PartialEq {
    /// Additive identity.
    fn zero() -> Self;
    /// Multiplicative identity.
    fn one() -> Self;
    /// Addition operation.
    fn add(&self, rhs: &Self) -> Self;
    /// Multiplication operation.
    fn mul(&self, rhs: &Self) -> Self;
}

/// Converts integer to semiring value.
pub fn from_usize<T: Semiring>(value: usize) -> T {
    let mut result = T::zero();
    let one = T::one();

    for _ in 0..value {
        result = T::add(&result, &one);
    }

    result
}

impl Semiring for u64 {
    fn zero() -> Self {
        0u64
    }

    fn one() -> Self {
        1u64
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Semiring for i64 {
    fn zero() -> Self {
        0i64
    }

    fn one() -> Self {
        1i64
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Semiring for f64 {
    fn zero() -> Self {
        0f64
    }

    fn one() -> Self {
        1f64
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

/// Polynomials with coefficient in `C`.
///
/// For example, polynomial `x^2 + 5x + 6` is represented in `Polynomial<u64>` as follows:
///
/// ```ignore
/// Polynomial {
///     coefficients: {
///         2: 1,
///         1: 5,
///         0: 6,
///     },
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial<C: Semiring> {
    coefficients: HashMap<u64, C>,
}

impl<C: Semiring> Semiring for Polynomial<C> {
    fn zero() -> Self {
        Polynomial {
            coefficients: HashMap::new(),
        }
    }

    fn one() -> Self {
        let mut map = HashMap::new();
        let _unused = map.insert(0, C::one());
        Polynomial { coefficients: map }
    }

    fn add(&self, rhs: &Self) -> Self {
        let mut map = self.coefficients.clone();
        for e in rhs.coefficients.iter() {
            let (k, v) = e;
            let val = map
                .entry(*k)
                .and_modify(|lhs_e| {
                    *lhs_e = lhs_e.add(v);
                })
                .or_insert(v.clone());
        }

        map.retain(|_, v| *v != C::zero());
        Polynomial { coefficients: map }
    }

    fn mul(&self, rhs: &Self) -> Self {
        let mut map: HashMap<u64, C> = HashMap::new();
        for el in self.coefficients.iter() {
            let (kl, vl) = el;
            for er in rhs.coefficients.iter() {
                let (kr, vr) = er;
                let result_k = kl + kr;
                let result_v = vl.mul(vr);
                let _unused = map
                    .entry(result_k)
                    .and_modify(|dst_e| {
                        *dst_e = dst_e.add(&result_v);
                    })
                    .or_insert(result_v);
            }
        }

        // 原地过滤掉系数为 zero 的项
        map.retain(|_, v| *v != C::zero());
        Polynomial { coefficients: map }
    }
}

impl<C: Semiring> Polynomial<C> {
    /// Constructs polynomial `x`.
    pub fn x() -> Self {
        let mut map = HashMap::new();
        let _unused = map.insert(1, C::one());
        Polynomial { coefficients: map }
    }

    /// Evaluates the polynomial with the given value.
    pub fn eval(&self, value: C) -> C {
        let mut result = C::zero();
        for (exp, coeff) in &self.coefficients {
            let mut v_pow_n = C::one();
            for _ in 0..*exp {
                v_pow_n = v_pow_n.mul(&value);
            }

            let term_value = coeff.mul(&v_pow_n);
            result = result.add(&term_value);
        }

        result
    }

    /// Constructs polynomial `ax^n`.
    pub fn term(a: C, n: u64) -> Self {
        let mut map = HashMap::new();
        let _unused = map.insert(n, a);
        Polynomial { coefficients: map }
    }
}

impl<C: Semiring> From<C> for Polynomial<C> {
    fn from(value: C) -> Self {
        Polynomial::term(value, 0)
    }
}

/// Given a string `s`, parse it into a `Polynomial<C>`.
/// You may assume that `s` follows the criteria below.
/// Therefore, you do not have to return `Err`.
///
/// Assumptions:
/// - Each term is separated by ` + `.
/// - Each term is one of the following form: `a`, `x`, `ax`, `x^n`, and `ax^n`, where `a` is a
///   `usize` number and `n` is a `u64` number. This `a` should then be converted to a `C` type.
/// - In `a`, it is guaranteed that `a >= 1`.
/// - In `ax` and `ax^n`, it is guaranteed that `a >= 2`.
/// - In `x^n` and `ax^n`, it is guaranteed that `n >= 2`.
/// - All terms have unique degrees.
///
/// Consult `assignment06/grade.rs` for example valid strings.
///
/// Hint: `.split`, `.parse`, and `Polynomial::term`
impl<C: Semiring> std::str::FromStr for Polynomial<C> {
    type Err = (); // Ignore this for now...

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result_poly = Self::zero();
        for term_str in s.split(" + ") {
            let (a_usize, n_u64) = if let Some(x_pos) = term_str.find('x') {
                // x, ax, x^n, ax^n
                let a = if x_pos == 0 {
                    1
                } else {
                    term_str[..x_pos].parse::<usize>().unwrap() // 形如ax..
                };

                let n = if let Some(hat_pos) = term_str.find('^') {
                    term_str[hat_pos + 1..].parse::<u64>().unwrap()
                } else {
                    1
                };

                (a, n)
            } else {
                // 常数项 a
                (term_str.parse::<usize>().unwrap(), 0)
            };

            let a: C = from_usize::<C>(a_usize);
            let term_poly = Self::term(a, n_u64);
            result_poly = result_poly.add(&term_poly);
        }
        Ok(result_poly)
    }
}

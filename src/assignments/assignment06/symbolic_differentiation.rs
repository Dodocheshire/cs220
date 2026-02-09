//! Symbolic differentiation with rational coefficents.

use std::clone;
use std::fmt;

use std::ops::*;

use ntest::assert_false;

use crate::assignments::assignment08::church::exp;

/// Rational number represented by two isize, numerator and denominator.
///
/// Each Rational number should be normalized so that `demoninator` is nonnegative and `numerator`
/// and `demoninator` are coprime. See `normalize` for examples. As a corner case, 0 is represented
/// by `Rational { numerator: 0, demoninator: 0 }`.
///
/// For "natural use", it also overloads standard arithmetic operations, i.e, `+`, `-`, `*`, and
/// `/`.
///
/// See [here](https://doc.rust-lang.org/core/ops/index.html) for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    numerator: isize,
    denominator: isize,
}

// Some useful constants.

/// Zero
pub const ZERO: Rational = Rational::new(0, 0);
/// One
pub const ONE: Rational = Rational::new(1, 1);
/// Minus one
pub const MINUS_ONE: Rational = Rational::new(-1, 1);

impl Rational {
    /// Creates a new rational number.
    pub const fn new(numerator: isize, denominator: isize) -> Self {
        let sign = if denominator > 0 { 1 } else { -1 };
        assert_false!(denominator == 0 && numerator != 0);
        Self {
            numerator: numerator * sign,
            denominator: denominator * sign,
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs == ZERO {
            self
        } else if self == ZERO {
            rhs
        } else {
            let mut result = Rational::new(
                self.numerator * rhs.denominator + rhs.numerator * self.denominator,
                self.denominator * rhs.denominator,
            );
            gcd_approx(&mut result);
            result
        }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Rational::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        );
        gcd_approx(&mut result);
        result
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs.numerator = -rhs.numerator;
        self.add(rhs)
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(rhs != ZERO);
        let mut result = Rational::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        );
        gcd_approx(&mut result);
        result
    }
}

fn gcd_approx(r: &mut Rational) {
    let mut b = r.denominator.abs();
    let mut a = r.numerator.abs();
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    r.numerator /= a;
    r.denominator /= a;
}

/// Differentiable functions.
///
/// For simplicity, we only consider infinitely differentiable functions.
pub trait Differentiable: Clone {
    /// Differentiate.
    ///
    /// Since the return type is `Self`, this trait can only be implemented
    /// for types that are closed under differentiation.
    fn diff(&self) -> Self;
}

impl Differentiable for Rational {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Constant_term_rule>
    fn diff(&self) -> Self {
        ZERO
    }
}

/// Singleton polynomial.
///
/// Unlike regular polynomials, this type only represents a single term.
/// The `Const` variant is included to make `Polynomial` closed under differentiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingletonPolynomial {
    /// Constant polynomial.
    Const(Rational),
    /// Non-const polynomial.
    Polynomial {
        /// Coefficent of polynomial. Must be non-zero.
        coeff: Rational,
        /// Power of polynomial. Must be non-zero.
        power: Rational,
    },
}

impl SingletonPolynomial {
    /// Creates a new const polynomial.
    pub fn new_c(r: Rational) -> Self {
        SingletonPolynomial::Const(r)
    }

    /// Creates a new polynomial.
    pub fn new_poly(coeff: Rational, power: Rational) -> Self {
        SingletonPolynomial::Polynomial { coeff, power }
    }
}

impl Differentiable for SingletonPolynomial {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Power_rule>

    fn diff(&self) -> Self {
        match *self {
            SingletonPolynomial::Const(c) => SingletonPolynomial::new_c(ZERO),
            SingletonPolynomial::Polynomial { coeff: c, power: p } => {
                if p == ONE {
                    SingletonPolynomial::new_c(c)
                } else {
                    SingletonPolynomial::new_poly(c * p, p - ONE)
                }
            }
        }
    }
}

/// Expoential function.(`e^x`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exp;

impl Exp {
    /// Creates a new exponential function.
    pub fn new() -> Self {
        Exp {}
    }
}

impl Default for Exp {
    fn default() -> Self {
        Self::new()
    }
}

impl Differentiable for Exp {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Derivatives_of_exponential_and_logarithmic_functions>
    fn diff(&self) -> Self {
        *self
    }
}

/// Trigonometric functions.
///
/// The trig fucntions carry their coefficents to be closed under differntiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trignometric {
    /// Sine function.
    Sine {
        /// Coefficent
        coeff: Rational,
    },
    /// Cosine function.
    Cosine {
        /// Coefficent
        coeff: Rational,
    },
}

impl Trignometric {
    /// Creates a new sine function.
    pub fn new_sine(coeff: Rational) -> Self {
        Trignometric::Sine { coeff }
    }

    /// Creates a new cosine function.
    pub fn new_cosine(coeff: Rational) -> Self {
        Trignometric::Cosine { coeff }
    }
}

impl Differentiable for Trignometric {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Derivatives_of_trigonometric_functions>
    fn diff(&self) -> Self {
        match *self {
            Trignometric::Cosine { coeff } => Trignometric::Sine {
                coeff: ZERO - coeff,
            },
            Trignometric::Sine { coeff } => Trignometric::Cosine { coeff },
        }
    }
}

/// Basic functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseFuncs {
    /// Constant
    Const(Rational),
    /// Polynomial
    Poly(SingletonPolynomial),
    /// Exponential
    Exp(Exp),
    /// Trignometirc
    Trig(Trignometric),
}

impl Differentiable for BaseFuncs {
    fn diff(&self) -> Self {
        match *self {
            BaseFuncs::Const(r) => BaseFuncs::Const(r.diff()),
            BaseFuncs::Poly(sp) => BaseFuncs::Poly(sp.diff()),
            BaseFuncs::Exp(exp) => BaseFuncs::Exp(exp.diff()),
            BaseFuncs::Trig(trig) => BaseFuncs::Trig(trig.diff()),
        }
    }
}

/// Complex functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplexFuncs<F> {
    /// Basic functions
    Func(F),
    /// Addition
    Add(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Subtraction
    Sub(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Multipliciation
    Mul(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Division
    Div(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Composition
    Comp(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
}

impl<F: Differentiable> Differentiable for Box<F> {
    fn diff(&self) -> Self {
        Box::new((**self).diff())
    }
}

impl<F: Differentiable> Differentiable for ComplexFuncs<F> {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Elementary_rules_of_differentiation>
    fn diff(&self) -> Self {
        match self {
            ComplexFuncs::Func(f) => ComplexFuncs::Func(f.diff()),
            ComplexFuncs::Add(f1, f2) => ComplexFuncs::Add(f1.diff(), f2.diff()),
            ComplexFuncs::Sub(f1, f2) => ComplexFuncs::Sub(f1.diff(), f2.diff()),
            ComplexFuncs::Mul(f1, f2) => {
                let prev: ComplexFuncs<F> = ComplexFuncs::Mul(f1.diff(), f2.clone());
                let post: ComplexFuncs<F> = ComplexFuncs::Mul(f1.clone(), f2.diff());
                ComplexFuncs::Add(Box::new(prev), Box::new(post))
            }
            ComplexFuncs::Div(f1, f2) => {
                let prev: ComplexFuncs<F> = ComplexFuncs::Mul(f1.diff(), f2.clone());
                let post: ComplexFuncs<F> = ComplexFuncs::Mul(f1.clone(), f2.diff());
                let numerator: ComplexFuncs<F> = ComplexFuncs::Sub(Box::new(prev), Box::new(post));
                let denominator: ComplexFuncs<F> = ComplexFuncs::Mul(f2.clone(), f2.clone());
                ComplexFuncs::Div(Box::new(numerator), Box::new(denominator))
            }
            ComplexFuncs::Comp(f1, f2) => {
                let prev: ComplexFuncs<F> = ComplexFuncs::Comp(f1.diff(), f2.clone());
                let post: Box<ComplexFuncs<F>> = f2.diff();
                ComplexFuncs::Mul(Box::new(prev), post)
            }
        }
    }
}

/// Evaluate functions.
pub trait Evaluate {
    ///  Evaluate `self` at `x`.
    fn evaluate(&self, x: f64) -> f64;
}

impl Evaluate for Rational {
    fn evaluate(&self, x: f64) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl Evaluate for SingletonPolynomial {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            SingletonPolynomial::Const(c) => c.evaluate(x),
            SingletonPolynomial::Polynomial { coeff, power } => {
                coeff.evaluate(0.0) * x.powf(power.evaluate(0.0))
            }
        }
    }
}

impl Evaluate for Exp {
    fn evaluate(&self, x: f64) -> f64 {
        x.exp()
    }
}

impl Evaluate for Trignometric {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Trignometric::Cosine { coeff } => coeff.evaluate(0.0) * x.cos(),
            Trignometric::Sine { coeff } => coeff.evaluate(0.0) * x.sin(),
        }
    }
}

impl Evaluate for BaseFuncs {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            BaseFuncs::Const(r) => r.evaluate(x),
            BaseFuncs::Exp(e) => e.evaluate(x),
            BaseFuncs::Poly(p) => p.evaluate(x),
            BaseFuncs::Trig(trig) => trig.evaluate(x),
        }
    }
}

impl<F: Evaluate> Evaluate for ComplexFuncs<F> {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            ComplexFuncs::Func(f) => f.evaluate(x),
            ComplexFuncs::Add(f1, f2) => (*f1).evaluate(x) + (*f2).evaluate(x),
            ComplexFuncs::Sub(f1, f2) => (*f1).evaluate(x) - (*f2).evaluate(x),
            ComplexFuncs::Mul(f1, f2) => (*f1).evaluate(x) * (*f2).evaluate(x),
            ComplexFuncs::Div(f1, f2) => (*f1).evaluate(x) / (*f2).evaluate(x),
            ComplexFuncs::Comp(f1, f2) => {
                let inner: f64 = f2.evaluate(x);
                f1.evaluate(inner)
            }
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == ZERO {
            return write!(f, "0");
        } else if self.denominator == 1 {
            return write!(f, "{}", self.numerator);
        }
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl fmt::Display for SingletonPolynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(r) => write!(f, "{r}"),
            Self::Polynomial { coeff, power } => {
                // coeff or power is zero
                if *coeff == ZERO {
                    return write!(f, "0");
                } else if *power == ZERO {
                    return write!(f, "{coeff}");
                }

                // Standard form of px^q
                let coeff = if *coeff == ONE {
                    "".to_string()
                } else if *coeff == MINUS_ONE {
                    "-".to_string()
                } else {
                    format!("({coeff})")
                };
                let var = if *power == ONE {
                    "x".to_string()
                } else {
                    format!("x^({power})")
                };
                write!(f, "{coeff}{var}")
            }
        }
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exp(x)")
    }
}

impl fmt::Display for Trignometric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (func, coeff) = match self {
            Trignometric::Sine { coeff } => ("sin(x)", coeff),
            Trignometric::Cosine { coeff } => ("cos(x)", coeff),
        };

        if *coeff == ZERO {
            write!(f, "0")
        } else if *coeff == ONE {
            write!(f, "{func}")
        } else if *coeff == MINUS_ONE {
            write!(f, "-{func}")
        } else {
            write!(f, "({coeff}){func}")
        }
    }
}

impl fmt::Display for BaseFuncs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(r) => write!(f, "{r}"),
            Self::Poly(p) => write!(f, "{p}"),
            Self::Exp(e) => write!(f, "{e}"),
            Self::Trig(t) => write!(f, "{t}"),
        }
    }
}

impl<F: Differentiable + fmt::Display> fmt::Display for ComplexFuncs<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplexFuncs::Func(func) => write!(f, "{func}"),
            ComplexFuncs::Add(l, r) => write!(f, "({l} + {r})"),
            ComplexFuncs::Sub(l, r) => write!(f, "({l} - {r})"),
            ComplexFuncs::Mul(l, r) => write!(f, "({l} * {r})"),
            ComplexFuncs::Div(l, r) => write!(f, "({l} / {r})"),
            ComplexFuncs::Comp(l, r) => write!(f, "({l} ∘ {r})"),
        }
    }
}

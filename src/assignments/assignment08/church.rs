//! Church Numerals
//!
//! This exercise involves the use of "Church numerals", a
//! representation of natural numbers using lambda calculus, named after
//! Alonzo Church. Each Church numeral corresponds to a natural number `n`
//! and is represented as a higher-order function that applies a given function `f` `n` times.
//!
//! For more information, see:
//! - <https://en.wikipedia.org/wiki/Church_encoding>
//! - <https://opendsa-server.cs.vt.edu/OpenDSA/Books/PL/html/ChurchNumerals.html>

use std::cell::RefCell;
use std::rc::Rc;

use rayon::result;

use crate::assignments::assignment06::symbolic_differentiation::ZERO;

/// Church numerals are represented as higher-order functions that take a function `f`
pub type Church<T> = Rc<dyn Fn(Rc<dyn Fn(T) -> T>) -> Rc<dyn Fn(T) -> T>>;

/// This function returns a Church numeral equivalent of the natural number 1.
/// It takes a function `f` and applies it exactly once.
pub fn one<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(x)))
}

/// This function returns a Church numeral equivalent of the natural number 2.
/// It takes a function `f` and applies it twice.
pub fn two<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(f(x))))
}

/// This function represents the Church numeral for zero. As zero applications
/// of `f` should leave the argument unchanged, the function simply returns the input.
pub fn zero<T: 'static>() -> Church<T> {
    Rc::new(|_| Rc::new(|x| x))
}

/// Implement a function to add 1 to a given Church numeral.
pub fn succ<T: 'static>(n: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        // let f_clone = f.clone();
        let n_clone = n.clone(); // 保证外闭包捕获的值n不被移动到内闭包中，保持Fn特征
        Rc::new(move |x| f((n_clone(f.clone()))(x))) // 内闭包保证捕获的值f不被消耗(即移动到n_clone函数中), 保持Fn特征
    })
}

/// Implement a function to add two Church numerals.
pub fn add<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        let nf = n(f.clone());
        let mf = m(f.clone());
        Rc::new(move |x| {
            // 内闭包按值捕获nf、mf
            nf(mf(x))
        })
    })
}

/// Implement a function to multiply (mult) two Church numerals.
pub fn mult<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f| n(m(f))) // marvelous!
}

/// Implement a function to raise one Church numeral to the power of another.
/// This is the Church numeral equivalent of the natural number operation of exponentiation.
/// Given two natural numbers `n` and `m`, the function should return a Church numeral
/// that represents `n` to the power of `m`. The key is to convert `n` and `m` to Church numerals,
/// and then apply the Church numeral for `m` (the exponent) to the Church numeral for `n` (the
/// base). Note: This function should be implemented *WITHOUT* using the `to_usize` or any
/// `pow`-like method.
pub fn exp<T: 'static>(n: usize, m: usize) -> Church<T> {
    // ACTION ITEM: Uncomment the following lines and replace `todo!()` with your code.
    let n = from_usize(n);
    let m = from_usize(m);
    Rc::new(move |f| {
        let n_clone = n.clone();
        let m_clone = m.clone();
        let n_pow_m = m_clone(n_clone);
        n_pow_m(f)
    })
}

/// Implement a function to convert a Church numeral to a usize type.
pub fn to_usize<T: 'static + Default>(n: Church<T>) -> usize {
    // hard to think out
    // 1. 创建一个可以在闭包之间共享的可变计数器
    let count = Rc::new(RefCell::new(0usize));
    let count_for_f = count.clone();

    // 2. 定义动作 f：它接收一个 T，返回一个 T
    // 但在执行过程中，它会悄悄把计数器 +1
    let f = Rc::new(move |x: T| {
        *count_for_f.borrow_mut() += 1;
        x // 丘奇数要求 f 必须返回同类型的值，这里原样返回即可
    });

    // 3. 准备初始值 x（利用 T 的 Default 特性）
    let x = T::default();
    // 4. 让丘奇数 n 运行起来！
    // n(f) 返回的是一个“执行了 n 次 f 的函数”
    // 然后再作用于 x
    let _unused = (n(f))(x);

    // 5. 提取并返回计数器的最终值
    // 此时虽然count_for_f 这个指针被移动进入闭包f了，但count还没有
    let result = *count.borrow();
    result
}

/// Implement a function to convert a usize type to a Church numeral.
pub fn from_usize<T: 'static>(n: usize) -> Church<T> {
    // less effective method
    // let mut result = zero();
    // for _ in 0..n {
    //     result = add(result, one());
    // }
    // result

    // This one failed for n = 777777777777777
    // Rc::new(move |f| {
    //     Rc::new(move |mut x| {
    //         for _ in 0..n {
    //             x = f(x);
    //         }
    //         x
    //     })
    // })

    // another way
    let mut result: Church<T> = zero();
    let mut bit_stack: Vec<u64> = Vec::new();
    let mut n = n;
    while n > 0 {
        bit_stack.push(n as u64 % 2);
        n /= 2;
    }
    let two: Church<T> = add(one(), one());
    while let Some(v) = bit_stack.pop() {
        result = mult(result, two.clone());
        if v == 1 {
            result = add(result, one());
        }
    }
    result
}

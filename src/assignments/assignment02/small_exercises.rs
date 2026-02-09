//! Small problems.

use std::iter;

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Fahrenheit to Celsius temperature degree.
pub fn fahrenheit_to_celsius(degree: f64) -> f64 {
    (degree - FAHRENHEIT_OFFSET) * FAHRENHEIT_SCALE
}

/// Capitalizes English alphabets (leaving the other characters intact).
pub fn capitalize(input: String) -> String {
    input.to_ascii_uppercase()
}

/// Returns the sum of the given array. (We assume the absence of integer overflow.)
pub fn sum_array(input: &[u64]) -> u64 {
    input.iter().fold(0u64, |acc:u64, &x:&u64| -> u64 {acc + x})
}

/// Given a non-negative integer, say `n`, return the smallest integer of the form `3^m` that's
/// greater than or equal to `n`.
///
/// For instance, up3(6) = 9, up3(9) = 9, up3(10) = 27. (We assume the absence of integer overflow.)
pub fn up3(n: u64) -> u64 {
    std::iter::successors(Some(1u64), |&prev: &u64| -> Option<u64> {
        // 使用 checked_mul 代替 *
        // 如果 prev * 3 溢出，它会返回 None，迭代器就会在此处优雅地停止
        prev.checked_mul(3)
    }).find(|&v: &u64| -> bool {v >= n}).unwrap()
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence
/// of integer overflow.)
pub fn gcd(lhs: u64, rhs: u64) -> u64 {
    if rhs == 0 {lhs} else {gcd(rhs, lhs % rhs)}
}

/// Returns the array of nC0, nC1, nC2, ..., nCn, where nCk = n! / (k! * (n-k)!). (We assume the
/// absence of integer overflow.)
///
/// Consult <https://en.wikipedia.org/wiki/Pascal%27s_triangle> for computation of binomial
/// coefficients without integer overflow.
pub fn chooses(n: u64) -> Vec<u64> {
    /*
        scan 算子类似于 fold，但不同的是：fold 只返回最后的结果，而 scan 会返回一个迭代器，记录每一次计算的中间值。
        (0..n) : 是一个左闭右开的区间 
        (0..=n): 左闭右闭
        用fold中的初始值vec![1] 处理基准情况(n==0), 用迭代次数(0..n).fold()处理演进过程(n>0)
     */
    (0..n).fold(vec![1], |prev, _| {
        std::iter::once(1)
            .chain(prev.windows(2).map(|w| {w[0] + w[1]}))
            .chain(std::iter::once(1))
            .collect()
    })
}

/// Returns the "zip" of two vectors.
///
/// For instance, `zip(vec![1, 2, 3], vec![4, 5])` equals to `vec![(1, 4), (2, 5)]`. Here, `3` is
/// ignored because it doesn't have a partner.
pub fn zip(lhs: Vec<u64>, rhs: Vec<u64>) -> Vec<(u64, u64)> {
    lhs.into_iter().zip(rhs).collect::<Vec<(u64, u64)>>()
}

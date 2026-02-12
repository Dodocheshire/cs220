//! Big integer with infinite precision.

use std::cmp::max;
use std::fmt;
use std::iter::zip;
use std::ops::*;

use rayon::result;

/// An signed integer with infinite precision implemented with an "carrier" vector of `u32`s.
///
/// The vector is interpreted as a base 2^(32 * (len(carrier) - 1)) integer, where negative
/// integers are represented in their [2's complement form](https://en.wikipedia.org/wiki/Two%27s_complement).
///
/// For example, the vector `vec![44,345,3]` represents the integer
/// `44 * (2^32)^2 + 345 * (2^32) + 3`,
/// and the vector `vec![u32::MAX - 5, u32::MAX - 7]` represents the integer
/// `- (5 * 2^32 + 8)`
///
/// You will implement the `Add` and `Sub` trait for this type.
///
/// Unlike standard fix-sized intergers in Rust where overflow will panic, the carrier is extended
/// to save the overflowed bit. On the contrary, if the precision is too much (e.g, vec![0,0] is
/// used to represent 0, where `vec![0]` is sufficent), the carrier is truncated.
///
/// See [this section](https://en.wikipedia.org/wiki/Two%27s_complement#Arithmetic_operations) for a rouge guide on implementation,
/// while keeping in mind that the carrier should be extended to deal with overflow.
///
/// The `sign_extension()`, `two_complement()`, and `truncate()` are non-mandatory helper methods.
///
/// For testing and debugging purposes, the `Display` trait is implemented for you, which shows the
/// integer in hexadecimal form.
#[derive(Debug, Clone)]
pub struct BigInt {
    /// The carrier for `BigInt`.
    ///
    /// Note that the carrier should always be non-empty.
    pub carrier: Vec<u32>,
}

impl BigInt {
    /// Create a new `BigInt` from a `usize`.
    pub fn new(n: u32) -> Self {
        BigInt { carrier: vec![n] }
    }

    /// Creates a new `BigInt` from a `Vec<u32>`.
    ///
    /// # Panic
    ///
    /// Panics if `carrier` is empty.
    pub fn new_large(carrier: Vec<u32>) -> Self {
        assert!(!carrier.is_empty());
        BigInt { carrier }
    }
}

const SIGN_MASK: u32 = 1 << 31;

impl BigInt {
    /// Extend `self` to `len` bits.
    fn sign_extension(&self, len: usize) -> Self {
        if self.carrier.len() * 32 >= len {
            return BigInt {
                carrier: self.carrier.clone(),
            };
        }
        let n_word = (len + 31) / 32 - self.carrier.len();
        let word = if (self.carrier.first().unwrap() >> 31) & 1 == 1 {
            //rust中 & 优先级大于 < > 大于 != ==, 不像c++ == 优先于 &
            !0u32
        } else {
            0u32
        };
        let mut new_carrier = self.carrier.clone();
        // Splice 在销毁（Drop）时，会执行它的析构逻辑（移动数组元素并完成插入），随后释放它对 new_carrier 的可变借用。
        drop(new_carrier.splice(0..0, std::iter::repeat(word).take(n_word))); // drop Splice值，释放对new_carrier的可变借用
        return BigInt {
            carrier: new_carrier,
        };
    }

    /// Compute the two's complement of `self`.
    fn two_complement(&self) -> Self {
        let mut carry = 1;
        let mut new_carrier: Vec<u32> = self
            .carrier
            .iter()
            .rev() // 反转迭代器，从低位字开始
            .map(|&x| {
                let (sum, overflow) = (!x).overflowing_add(carry);
                carry = if overflow { 1 } else { 0 };
                sum
            })
            .collect();
        new_carrier.reverse(); // 转回大端序
                               // 处理溢出(如-1(32 bit) 取负数 (I32_Max+1))
        let old_sign_bit = (self.carrier.first().unwrap() >> 31) & 1;
        let new_sign_bit = (new_carrier.first().unwrap() >> 31) & 1;

        if old_sign_bit == 1 && new_sign_bit == 1 {
            // 溢出：负数取负为负数
            new_carrier.insert(0, 0);
        }
        BigInt {
            carrier: new_carrier,
        }
    }

    /// Truncate a `BigInt` to the minimum length.
    /// 对于正数，多余的 0 字可以移除，但要保证移除后，新的最高位字的最高位仍然是 0
    /// 对于负数，多余的 u32::MAX (即 0xFFFFFFFF) 字可以移除，但要保证移除后，新的最高位字的最高位仍然是 1
    fn truncate(&self) -> Self {
        let mut start = 0;
        while start + 1 < self.carrier.len() {
            let current = self.carrier[start];
            let next = self.carrier[start + 1];
            let next_msb = (next >> 31) == 1;

            if current == 0 && !next_msb {
                start += 1;
            } else if current == u32::MAX && next_msb {
                start += 1;
            } else {
                break;
            }
        }

        BigInt {
            carrier: self.carrier[start..].to_vec(),
        }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let tgt_len = max(self.carrier.len(), rhs.carrier.len()) + 1; // 预留可能的进位空间
        let extended_lhs = self.sign_extension(tgt_len * 32);
        let extended_rhs = rhs.sign_extension(tgt_len * 32);

        let mut carry = 0u32;
        let mut new_carrier = vec![0u32; tgt_len];
        for i in (0..tgt_len).rev() {
            let (sum1, overflow1) =
                extended_lhs.carrier[i].overflowing_add(extended_rhs.carrier[i]);
            let (sum2, overflow2) = sum1.overflowing_add(carry);

            new_carrier[i] = sum2;
            carry = if overflow1 || overflow2 { 1 } else { 0 };
        }

        let result = BigInt {
            carrier: new_carrier,
        };
        result.truncate()
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.two_complement()
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hex formatting so that each u32 can be formatted independently.
        for i in self.carrier.iter() {
            write!(f, "{:08x}", i)?;
        }
        Ok(())
    }
}

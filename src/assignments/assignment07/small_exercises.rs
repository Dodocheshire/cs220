//! Implement functions using `Iterator` trait

struct FindIter<'s, T: Eq> {
    query: &'s [T],
    base: &'s [T],
    curr: usize,
}

impl<T: Eq> Iterator for FindIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr + self.query.len() <= self.base.len() {
            let i = self.curr;
            if self.base[i..].starts_with(self.query) {
                self.curr += 1;
                return Some(i);
            }
            self.curr += 1;
        }
        None
    }
}

/// Returns an iterator over substring query indexes in the base.
// impl Iterator<Item = usize> (不透明返回类型) 表示返回一个实现了 Iterator trait 的类型
// + 用于连接多个约束。 在返回类型中，impl Trait + 'a 表示返回的这个东西既要实现那个 Trait，又要符合生命周期 'a 的约束
// 's 告诉编译器：“返回的这个迭代器，它内部携带了生命周期为 's 的引用。”
// 如果没有 's： 编译器可能会认为这个迭代器是“自包含”的（不依赖外部引用），从而允许它在原数据被销毁后继续存在，这会导致不安全
// 加上 's +： 你明确告诉编译器，这个返回值的有效期限被绑定到了输入参数的生命周期 's 上
pub fn find<'s, T: Eq>(query: &'s [T], base: &'s [T]) -> impl 's + Iterator<Item = usize> {
    FindIter {
        query,
        base,
        curr: 0,
    }
}

/// Implement generic fibonacci iterator
struct FibIter<T> {
    first: T,
    second: T,
}

impl<T: std::ops::Add<Output = T> + Copy> FibIter<T> {
    fn new(first: T, second: T) -> Self {
        FibIter { first, second }
    }
}

impl<T> Iterator for FibIter<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.first;
        (self.first, self.second) = (self.second, self.first.add(self.second));
        Some(ret)
    }
}

/// Returns and iterator over the generic fibonacci sequence starting from `first` and `second`.
/// This is a generic version of `fibonacci` function, which works for any types that implements
/// `std::ops::Add` trait.
pub fn fib<T>(first: T, second: T) -> impl Iterator<Item = T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    FibIter::new(first, second)
}

/// Endpoint of range, inclusive or exclusive.
#[derive(Debug)]
pub enum Endpoint {
    /// Inclusive endpoint
    Inclusive(isize),

    /// Exclusive endpoint
    Exclusive(isize),
}

struct RangeIter {
    left: isize,
    right: isize,
    step: isize,
    curr: isize,
}

impl RangeIter {
    fn new(endpoints: (Endpoint, Endpoint), step: isize) -> Self {
        let start_val = match endpoints.0 {
            Endpoint::Exclusive(v) => {
                if step > 0 {
                    v + 1
                } else {
                    v - 1
                }
            }
            Endpoint::Inclusive(v) => v,
        };
        let end_val = match endpoints.1 {
            Endpoint::Exclusive(v) => {
                if step > 0 {
                    v - 1
                } else {
                    v + 1
                }
            }
            Endpoint::Inclusive(v) => v,
        };
        RangeIter {
            left: start_val,
            right: end_val,
            curr: start_val,
            step,
        }
    }
}

impl Iterator for RangeIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        // 已经包含left > right 但step > 0 或 left < right 但step < 0的情形
        if self.curr > self.right && self.step >= 0 || self.curr < self.right && self.step <= 0 {
            return None;
        } else {
            let ret = self.curr;
            self.curr += self.step;
            Some(ret)
        }
    }
}

/// Returns an iterator over the range [left, right) with the given step.
pub fn range(left: Endpoint, right: Endpoint, step: isize) -> impl Iterator<Item = isize> {
    RangeIter::new((left, right), step)
}

/// Write an iterator that returns all divisors of n in increasing order.
/// Assume n > 0.
///
/// Hint: trying all candidates from 1 to n will most likely time out!
/// To optimize it, make use of the following fact:
/// if x is a divisor of n that is greater than sqrt(n),
/// then n/x is a divisor of n that is smaller than sqrt(n).
struct Divisors {
    n: u64,
    current_i: u64,
    /*使用 Vec 作为栈，
    因为先发现的配对约数（如 n/1）最大，后发现的（如 n/sqrt(n)）较小。 */
    large_divisors: Vec<u64>,
}
impl Divisors {
    fn new(n: u64) -> Self {
        Divisors {
            n,
            current_i: 1,
            large_divisors: Vec::new(),
        }
    }
}
impl Iterator for Divisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // 寻找 <= sqrt(n) 的约数, 使用 i * i <= n 来避免计算浮点根号
        while self.current_i * self.current_i <= self.n {
            let i = self.current_i;
            self.current_i += 1;

            if self.n % i == 0 {
                let counterpart = self.n / i;
                // 如果不是平方根，将大的存入栈中
                if counterpart != i {
                    self.large_divisors.push(counterpart);
                }
                return Some(i);
            }
        }

        // 从小到大返回之前存入栈large_divisors中的大约数(>sqrt(n))
        self.large_divisors.pop()
    }
}

/// Returns an iterator over the divisors of n.
pub fn divisors(n: u64) -> impl Iterator<Item = u64> {
    Divisors::new(n)
}

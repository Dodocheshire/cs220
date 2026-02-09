//! Implement your own minimal `itertools` crate.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Iterator that iterates over the given iterator and returns only unique elements.
#[derive(Debug)]
pub struct Unique<I: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I: Iterator> Iterator for Unique<I>
where
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iter.next() {
            if self.seen.insert(v.clone()) {
                return Some(v);
            }
        }
        None
    }
}

/// Iterator that chains two iterators together.
#[derive(Debug)]
pub struct Chain<I1: Iterator, I2: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    iter1: I1,
    iter2: I2,
    iter1_end: bool,
}

impl<T: Eq + Hash + Clone, I1: Iterator<Item = T>, I2: Iterator<Item = T>> Iterator
    for Chain<I1, I2>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter1_end == false {
            if let Some(v) = self.iter1.next() {
                Some(v)
            } else {
                self.iter1_end = true;
                self.iter2.next()
            }
        } else {
            self.iter2.next()
        }
    }
}

/// Iterator that iterates over given iterator and enumerates each element.
#[derive(Debug)]
pub struct Enumerate<I: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    idx: usize,
    iter: I,
}

impl<I: Iterator> Iterator for Enumerate<I> {
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(v) => {
                let ret: (usize, I::Item) = (self.idx, v);
                self.idx += 1;
                Some(ret)
            }
            None => None,
        }
    }
}

/// Iterator that zips two iterators together.
///
/// If one iterator is longer than the other one, the remaining elements for the longer element
/// should be ignored.
#[derive(Debug)]
pub struct Zip<I1: Iterator, I2: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    iter1: I1,
    iter2: I2,
    end: bool,
}

impl<I1: Iterator, I2: Iterator> Iterator for Zip<I1, I2> {
    type Item = (I1::Item, I2::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.end == true {
            return None;
        }
        match (self.iter1.next(), self.iter2.next()) {
            (Some(v1), Some(v2)) => Some((v1, v2)),
            (Some(_), None) | (None, Some(_)) => {
                self.end = true;
                None
            }
            (None, None) => {
                self.end = true;
                None
            }
        }
    }
}

/// My Itertools trait.
pub trait MyIterTools: Iterator {
    /// Returns an iterator that iterates over the `self` and returns only unique elements.
    fn my_unique(self) -> Unique<Self>
    where
        Self: Sized,
    {
        Unique {
            iter: self,
            seen: HashSet::new(),
        }
    }

    /// Returns an iterator that chains `self` and `other` together.
    fn my_chain<I: Iterator>(self, other: I) -> Chain<Self, I>
    where
        Self: Sized,
    {
        Chain {
            iter1: self,
            iter2: other,
            iter1_end: false,
        }
    }

    /// Returns an iterator that iterates over `self` and enumerates each element.
    fn my_enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate { idx: 0, iter: self }
    }

    /// Returns an iterator that zips `self` and `other` together.
    fn my_zip<I: Iterator>(self, other: I) -> Zip<Self, I>
    where
        Self: Sized,
    {
        Zip {
            iter1: self,
            iter2: other,
            end: false,
        }
    }

    /// Foldleft for `MyIterTools`
    fn my_fold<T, F>(mut self, init: T, mut f: F) -> T
    where
        Self: Sized,
        F: FnMut(Self::Item, T) -> T,
    {
        let mut state: T = init;
        for v in self {
            state = f(v, state);
        }
        state
    }
}

// Trait `Sized`: 含义：表示该类型的大小在编译时是已知的。
// Rust 的默认行为：在泛型定义 impl<T> ... 中，Rust 会默认给 T 加上 Sized 的约束，即 impl<T: Sized> ...
// T: ?Sized表示 为所有实现了 Iterator 接口的类型 T 自动实现 MyIterTools 接口，无论 T 在编译时的大小是否确定
impl<T: ?Sized> MyIterTools for T where T: Iterator {}

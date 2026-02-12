//! Labyrinth
//!
//! Look at `labyrinth_grade.rs` below before you start.
//! HINT: <https://en.wikipedia.org/wiki/100_prisoners_problem>
//!
//! NOTE: You will have to implement a probabilistic algorithm, which means, the algorithm can fail
//! even if you have implemented the solution. We recommend running multiple times (at least 5
//! times) to check your solution works well.

// 好玩的问题，置换环，在总共100个元素的置换中，长度超过一半（超过50）的环最多只能有一个。
// 这是由于任何一个置换都可以分解为若干个互不相交的循环/环（Cycles）
// 当囚犯 k 从抽屉 k开始， 并根据抽屉内的号码指向下一个抽屉时，他实际上是在遍历包含数k的那个特定的“环”
// 如果包含囚犯k编号的环长度 L <= 50, 那么他一定能在 50 步内找到自己的编号。
// 当且仅当该置换中所有的环长度都 <= 50, 所有囚犯都能成功
// 可以证明，在n阶置换中，出现长度为L(L > 50)的环的恰为1/L
// 由于出现长度大于50的环的事件是互斥的，囚犯全员失败的概率就是出现长度为51、52...100的环概率之和 ~= 0.7
use std::cell::RefCell;

/// Husband
#[derive(Debug)]
pub struct Husband {
    brain: RefCell<[usize; 100]>,
}

impl Husband {
    /// What might a husband, who is looking for his wife's ID my_wife, be thinking?
    pub fn seeking(my_wife: usize) -> Self {
        let mut init_brain = [0usize; 100];
        init_brain[0] = my_wife;
        Husband {
            brain: RefCell::new(init_brain),
        }
    }

    #[allow(missing_docs)]
    pub fn has_devised_a_strategy(&self) -> Strategy<'_> {
        Strategy { husband: self }
    }

    /// Based on the information about currently visited room number and someone's wife ID trapped
    /// inside, what the husband should do next?
    pub fn carefully_checks_whos_inside(&self, room: usize, wife: usize) {
        self.brain.borrow_mut()[0] = wife;
    }
}

/// Strategy of husband
#[derive(Debug)]
pub struct Strategy<'a> {
    husband: &'a Husband,
}

impl Iterator for Strategy<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_room = self.husband.brain.borrow()[0];
        Some(next_room)
    }
}

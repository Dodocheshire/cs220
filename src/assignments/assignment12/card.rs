//! Flipping card game.
//!
//! HINT: For this assignment, you have to see `play` function in `card_grade.rs` file.
//! Multiple threads will be created and they will run as enemy bots(`bot_threads`).
//! Strategy of the enemy bots is implemented in the closure of the `thread::spawn` function.
//! Your goal is to beat them so that there are more white cards than blue cards in the ground.
//! Write your strategy in the `flip_card_strategy` function of the `Player` struct.
//!
//! Have fun!

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::thread::current;

/// Color represents the color of the card.
/// The color of a card can be either Blue or White.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    /// blue
    Blue,

    /// white
    White,
}

/// Player struct represents a player in the card game.
/// Each player has a memory which is represented as a HashMap.
#[derive(Debug)]
pub struct Player {
    memory: HashMap<isize, isize>,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    /// Creates a new player with an empty memory.
    pub fn new() -> Self {
        let init_map: HashMap<isize, isize> = (0..25).map(|i| (i, i * 400)).collect();
        Self { memory: init_map }
    }

    /// This function should return the index of the card to flip and the color to change to.
    pub fn flip_card_strategy(&mut self) -> (usize, Color) {
        // a little hard
        // 敌人的弱点：只要牌是蓝色的，敌人就会执行 thread::sleep 10 微秒，是非常漫长的
        // 每个机器人负责 400 张牌,
        // memory[-1] = 当前段编号
        // 轮流选分段0, 1, 2, ... ,24, 0, 1, 2
        let current_seg = {
            let seg = self.memory.entry(-1).or_insert(0);
            *seg as usize % 25
        };
        // 当前分段的"当前指针"
        let base = self.memory.get_mut(&(current_seg as isize)).unwrap();
        let m = (*base).rem_euclid(5);
        *base = match m {
            0 => *base + 2,
            1 => *base + 2,
            2 => *base + 1,
            3 => *base + 1,
            4 => *base + 3,
            _ => unreachable!(),
        };
        // 复原指针到段开头
        if *base >= (current_seg as isize) * 400 + 400 {
            *base = (current_seg as isize) * 400;
        }
        let idx = *base as usize; // 翻哪张牌
        *self.memory.get_mut(&-1).unwrap() += 1; // memory[-1]设置为下一个分段

        (idx, Color::White)
    }
}

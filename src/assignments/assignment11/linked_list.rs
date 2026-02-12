//! Singly linked list.
//!
//! Consult <https://doc.rust-lang.org/book/ch15-01-box.html>.

use std::{boxed, collections::btree_map::Values, fmt::Debug, ops::Deref};

use itertools::{rev, Itertools};
use rayon::vec;

/// Node of the list.
#[derive(Debug)]
pub struct Node<T: Debug> {
    /// Value of current node.
    pub value: T,

    /// Pointer to the next node. If it is `None`, there is no next node.
    pub next: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    /// Creates a new node.
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

/// A singly-linked list.
#[derive(Debug)]
pub struct SinglyLinkedList<T: Debug> {
    /// Head node of the list. If it is `None`, the list is empty.
    head: Option<Node<T>>,
}

impl<T: Debug> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> SinglyLinkedList<T> {
    /// Creates a new list.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Adds the given node to the front of the list.
    pub fn push_front(&mut self, value: T) {
        let mut new_head: Node<T> = Node::new(value);
        new_head.next = self.head.take().map(Box::new);
        self.head = Some(new_head);
    }
    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, value: T) {
        match self.head.as_mut() {
            None => {
                self.head = Some(Node::new(value));
            }
            Some(mut v) => {
                while v.next.is_some() {
                    // as_mut(): 仅仅把Option<T> -> Option<&mut T>
                    // impl as_deref_mut() for Option<T: DerefMut> -> Option<&mut T::Target>
                    // e.g. Option<Box<Node<T>>>.as_deref_mut():
                    // 1. Box<Node<T>> implements DerefMut trait, whose Target = Node<T>
                    // 2. deref &mut Box<Node<T>> -> &mut Node<T>
                    // 3. wrap Option -> Option<&mut Node<T>>
                    v = v.next.as_deref_mut().unwrap();
                }
                v.next = Some(Box::new(Node::new(value)));
            }
        }
    }

    /// Removes and returns the node at the front of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        let Node { value, next } = self.head.take()?;
        self.head = next.map(|node_ptr| *node_ptr);
        Some(value)
    }

    /// Removes and returns the node at the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        // 空链表
        let _ = self.head.as_ref()?;
        // 一个节点
        if self.head.as_ref().unwrap().next.is_none() {
            return self.head.take().map(|n| n.value);
        }
        let mut current = self.head.as_mut()?;
        // 当前节点下一个的下一个不是None循环
        while current.next.as_deref().unwrap().next.is_some() {
            current = current.next.as_deref_mut().unwrap();
        }
        current.next.take().map(|boxed_node| boxed_node.value)
    }

    /// Create a new list from the given vector `vec`.
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut linked_list = SinglyLinkedList::new();
        if vec.is_empty() {
            return linked_list;
        }
        for value in rev(vec) {
            linked_list.push_front(value);
        }
        linked_list
    }

    /// Convert the current list into a vector.
    pub fn into_vec(mut self) -> Vec<T> {
        let mut values: Vec<T> = Vec::new();
        while let Some(node) = self.head.take() {
            values.push(node.value);
            self.head = node.next.map(|boxed_node| *boxed_node);
        }
        values
    }

    /// Return the length (i.e., number of nodes) of the list.
    pub fn length(&self) -> usize {
        if self.head.is_none() {
            return 0;
        }
        let mut current = self.head.as_ref().unwrap().next.as_ref();
        let mut cnt = 1;

        while let Some(boxed_node) = current {
            cnt += 1;
            current = boxed_node.next.as_ref();
        }
        cnt
    }

    /// Apply function `f` on every element of the list.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2]`, `f`: `|x| x + 1` ==> `[2, 3]`
    pub fn map<F: Fn(T) -> T>(mut self, f: F) -> Self {
        // hard
        let mut values = self.into_vec();
        let mut transformed_values = values.into_iter().map(f).collect::<Vec<T>>();
        SinglyLinkedList::from_vec(transformed_values)
    }

    /// Apply given function `f` for each adjacent pair of elements in the list.
    /// If `self.length() < 2`, do nothing.
    ///
    /// # Examples
    ///
    /// `self`: `[1, 2, 3, 4]`, `f`: `|x, y| x + y`
    /// // each adjacent pair of elements: `(1, 2)`, `(2, 3)`, `(3, 4)`
    /// // apply `f` to each pair: `f(1, 2) == 3`, `f(2, 3) == 5`, `f(3, 4) == 7`
    /// ==> `[3, 5, 7]`
    pub fn pair_map<F: Fn(T, T) -> T>(self, f: F) -> Self
    where
        T: Clone,
    {
        let mut values = self.into_vec();
        let transformed_vals = values
            .into_iter()
            .tuple_windows()
            .map(|(e1, e2)| f(e1, e2))
            .collect::<Vec<T>>();
        SinglyLinkedList::from_vec(transformed_vals)
    }
}

// A list of lists.
impl<T: Debug> SinglyLinkedList<SinglyLinkedList<T>> {
    /// Flatten the list of lists into a single list.
    ///
    /// # Examples
    /// `self`: `[[1, 2, 3], [4, 5, 6], [7, 8]]`
    /// ==> `[1, 2, 3, 4, 5, 6, 7, 8]`
    pub fn flatten(self) -> SinglyLinkedList<T> {
        let flattened_vec = self
            .into_vec()
            .into_iter()
            .flat_map(|list| list.into_vec())
            .collect::<Vec<T>>();
        SinglyLinkedList::from_vec(flattened_vec)
    }
}

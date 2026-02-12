//! Small exercises.

use std::collections::HashSet;

use itertools::*;
use ndarray::Data;
use rayon::result;

/// Returns the pairs of `(i, j)` where `i < j` and `inner[i] > inner[j]` in increasing order.
///
/// For example, the inversions of `[3, 5, 1, 2, 4]` is `[(0, 2), (0, 3), (1, 2), (1, 3), (1, 4)]`
/// because as follows:
///
/// - `0 < 2`, `inner[0] = 3 > 1 = inner[2]`
/// - `0 < 3`, `inner[0] = 3 > 2 = inner[3]`
/// - `1 < 2`, `inner[1] = 5 > 1 = inner[2]`
/// - `1 < 3`, `inner[1] = 5 > 2 = inner[3]`
/// - `1 < 4`, `inner[1] = 5 > 4 = inner[4]`
///
/// Consult <https://en.wikipedia.org/wiki/Inversion_(discrete_mathematics)> for more details of inversion.
pub fn inversion<T: Ord>(inner: Vec<T>) -> Vec<(usize, usize)> {
    let n = inner.len();
    (0..n)
        .cartesian_product(0..n)
        .filter(|(a, b)| a < b && inner[*a] > inner[*b])
        .collect()
}

/// Represents a node of tree data structure.
///
/// Consult <https://en.wikipedia.org/wiki/Tree_(data_structure)> for more details on tree data structure.
#[derive(Debug)]
pub enum Node<T> {
    /// Non-leaf node
    ///
    /// It contains `(the name of node, list of child nodes)`.
    NonLeaf((T, Vec<Node<T>>)),
    /// Leaf node
    ///
    /// It contains the name of node.
    Leaf(T),
}

/// Traverses the tree in preorder.
///
/// The algorithm for preorder traversal is as follows:
///
/// 1. Visit the root.
/// 2. If the root is a leaf node, end the traverse.
/// 3. If the root is a non-leaf node, traverse each subtree from the child nodes.
///
/// For example, the result of preorder traversal for the following tree
///
/// ```text
///     1
///    /|\
///   2 3 4
///  /|  /|\
/// 5 6 7 8 9
/// ```
///
/// which can be represented as
///
/// ```ignore
/// Node::NonLeaf((
///     1,
///     vec![
///         Node::NonLeaf((2, vec![Node::Leaf(5), Node::Leaf(6)])),
///         Node::Leaf(3),
///         Node::NonLeaf((4, vec![Node::Leaf(7), Node::Leaf(8), Node::Leaf(9)])),
///     ]
/// ))
/// ```
///
/// is `1 -> 2 -> 5 -> 6 -> 3 -> 4 -> 7 -> 8 -> 9`.
pub fn traverse_preorder<T>(root: Node<T>) -> Vec<T> {
    // idk
    // 结合递归与迭代器的 chain（链接）以及 flat_map（扁平映射）
    match root {
        Node::Leaf(f) => {
            vec![f]
        }
        Node::NonLeaf((x, children)) => std::iter::once(x)
            .chain(children.into_iter().flat_map(traverse_preorder))
            .collect(),
    }
}

/// File
#[derive(Debug)]
pub enum File {
    /// Directory
    ///
    /// It contains `(name of directory, list of files under the directory)`
    ///
    /// The size of a directory is the sum of the sizes of its sub-files.
    Directory(String, Vec<File>),

    /// Data
    ///
    /// It contains `(name of data, size of data)`
    Data(String, usize),
}

/// Given a file, summarize all subfiles and sizes in ascending order of size.
///
/// - Its behaviour is the same as the `du | sort -h` command on Linux.
/// - If the file size is the same, sort it by name.
/// - Assume that there are no duplicate file names.
///
/// # Example
///
/// Input:
///
/// ```txt
/// root (Directory)
/// |
/// |__a (Directory)
/// |  |__a1 (Data, size: 1)
/// |  |__a2 (Data, size: 3)
/// |
/// |__b (Directory)
/// |  |__b1 (Data, size: 3)
/// |  |__b2 (Data, size: 15)
/// |
/// |__c (Data, size: 8)
/// ```
///
/// Output: `[("a1", 1), ("a2", 3), ("b1", 3), ("a", 4), ("c", 8), ("b2", 15), ("b", 18), ("root",
/// 30)]`
pub fn du_sort(root: &File) -> Vec<(&str, usize)> {
    // hard
    fn collect_info(node: &File) -> (usize, Vec<(&str, usize)>) {
        match node {
            // 数据文件：大小就是自身，列表只包含自己
            File::Data(name, size) => (*size, vec![(name.as_str(), *size)]),

            // 目录：大小是直接子项的大小之和
            File::Directory(name, children) => {
                let mut dir_size = 0;
                let mut all_items = Vec::new();

                for child in children {
                    let (child_size, mut child_items) = collect_info(child);
                    dir_size += child_size; // 累加直接子项的大小
                    all_items.append(&mut child_items); // 收集子项及其子孙的所有记录
                }

                // 将目录本身的信息存入列表
                all_items.push((name.as_str(), dir_size));
                (dir_size, all_items)
            }
        }
    }

    let (_, mut result) = collect_info(root);

    result.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(b.0)));
    result
}

/// Remove all even numbers inside a vector using the given mutable reference.
/// That is, you must modify the vector using the given mutable reference instead
/// of returning a new vector.
///
/// # Example
/// ```ignore
/// let mut vec = vec![1, 2, 3, 4, 5];
/// remove_even(&mut vec);
/// assert_eq!(*vec, vec![1, 3, 5]);
/// ```
#[allow(clippy::ptr_arg)]
pub fn remove_even(inner: &mut Vec<i64>) {
    inner.retain(|&x| x % 2 != 0)
}

/// Remove all duplicate occurences of a number inside the array.
/// That is, if an integer appears more than once, remove some occurences
/// of it so that it only appears once. Note that you must modify the vector
/// using the given mutable reference instead of returning a new vector.
/// Also, note that the order does not matter.
///
/// # Example
/// ```ignore
/// let mut vec = vec![1, 2, 1, 1, 3, 7, 5, 7];
/// remove_duplicate(&mut vec);
/// assert_eq!(*vec, vec![1, 2, 3, 7, 5]);
/// ```
#[allow(clippy::ptr_arg)]
pub fn remove_duplicate(inner: &mut Vec<i64>) {
    // idk
    inner.sort_unstable();
    inner.dedup();
}

/// Returns the natural join of two tables using the first column as the join argument.
/// That is, for each pair of a row(`Vec<String>`) from table1 and a row(`Vec<String>`) from table2,
/// if the first element of them are equal, then add all elements of the row from table2
/// except its first element to the row from table1 and add it to the results.
/// Note that the order of results does not matter.
///
/// # Example
///
/// ```text
///        table1                     table2
/// ----------------------     ----------------------
///  20230001 |    Jack         20230001 |    CS
///  20231234 |    Mike         20230001 |    EE
///                             20231234 |    ME
///
///
///               result
/// -----------------------------------
///  20230001 |    Jack   |     CS
///  20230001 |    Jack   |     EE
///  20231234 |    Mike   |     ME
/// ```
pub fn natural_join(table1: Vec<Vec<String>>, table2: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let joint_pairs = table1.into_iter().cartesian_product(table2);
    let mut result: Vec<Vec<String>> = Vec::new();
    joint_pairs
        .filter(|(e1, e2)| e1[0] == e2[0])
        .map(|(mut e1, e2)| {
            e1.extend(e2.into_iter().skip(1));
            e1
        })
        .collect()
}

/// You can freely add more fields.
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Candidate {
    c: u64,
    m: u64,
    n: u64,
}
impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        // 反转比较逻辑，使 BinaryHeap 变成"最小堆"
        let ord = other.c.cmp(&self.c);
        if ord != Ordering::Equal {
            return ord;
        }

        // 如果c相等，比较a的值,要求勾股数(a, b, c) c相等时优先生成a小的数对,即a更小时在堆中应该优先
        let get_a = |m: u64, n: u64| {
            let leg1 = m * m - n * n;
            let leg2 = 2 * m * n;
            if leg1 < leg2 {
                leg1
            } else {
                leg2
            } // 生成的勾股数对应该a < b
        };
        let self_a = get_a(self.m, self.n);
        let other_a = get_a(other.m, other.n);
        other_a.cmp(&self_a)
    }
}
impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
struct Pythagorean {
    heap: BinaryHeap<Candidate>, // min heap(using our cmp logic)
}
// too hard
// a = m^2 - n^2
// b = 2mn
// c = m^2 + n^2
// && m > n > 0 gcd(m, n) = 1, m - n is odd
impl Pythagorean {
    fn new() -> Self {
        let mut heap = BinaryHeap::new();
        heap.push(Candidate { c: 5, m: 2, n: 1 });
        Pythagorean { heap }
    }
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}
impl Iterator for Pythagorean {
    type Item = (u64, u64, u64);

    // 难点：实现一个流式排序算法，把一个二维网格里的点，按照c = m^2 + n^2的顺序"拉成"一条线
    // 如何不重不漏地遍历整个无穷大的网格?
    fn next(&mut self) -> Option<Self::Item> {
        // 循环直到找到一个符合条件的本原勾股数
        while let Some(Candidate { c, m, n }) = self.heap.pop() {
            // 生成后续候选者放入堆中以保证序列不中断
            // 规则：1. 增加 n (如果 n+1 < m)
            // 即网格中的点(m, n) => (m, n+1)
            // 所以对所有n > 1的点(m, n)只能由规则一生成(网格中横向箭头)
            if n + 1 < m {
                let next_n = n + 1;
                self.heap.push(Candidate {
                    c: m * m + next_n * next_n,
                    m,
                    n: next_n,
                });
            }
            // 规则：2. 如果当前是 n=1，尝试增加 m（避免重复生成）
            // 即网格中的点(m, 1) => 生成 (m+1, 1)
            // 所以对所有n = 1的点(m, n)只能由规则二生成

            // 规则一/二保证了每个(m > n)节点有且仅被加入heap中一次，每从heap中弹出一个节点，就会push进0/1/2个节点

            if n == 1 {
                let next_m = m + 1;
                self.heap.push(Candidate {
                    c: next_m * next_m + 1,
                    m: next_m,
                    n: 1,
                });
            }

            // 检查当前的 (m, n) 是否能生成本原勾股数
            // 条件：1. 互质 2. 奇偶性不同
            if (m - n) % 2 == 1 && gcd(m, n) == 1 {
                let leg_a = m * m - n * n;
                let leg_b = 2 * m * n;
                // requires a < b
                let (a, b) = if leg_a < leg_b {
                    (leg_a, leg_b)
                } else {
                    (leg_b, leg_a)
                };
                return Some((a, b, c));
            }
        }
        None
    }
}

/// Generates sequence of unique [primitive Pythagorean triples](https://en.wikipedia.org/wiki/Pythagorean_triple),
/// i.e. (a,b,c) such that a² + b² = c², a and b are coprimes, and a < b. Generate in the increasing
/// order of c.
pub fn pythagorean() -> impl Iterator<Item = (u64, u64, u64)> {
    Pythagorean::new()
}

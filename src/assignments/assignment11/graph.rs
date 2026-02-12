//! A small graph library.
//!
//! A node has a i32 value and (directed) edges to other nodes. A node does not have multiple edges
//! to the same node. Nodes are not associated with a particular domain, and users can freely
//! create nodes however they like. However, after a node is created, it can be added to a
//! `SubGraph`, which form a subgraph of the graph of all nodes. A node can be added to multiple
//! subgraphs. `SubGraph` has a method to check if the it has a cycle.
//!
//! The goal of this assignment is to learn how to deal with inherently shared mutable data in
//! Rust. Design the types and fill in the `todo!()`s in methods. There are several possible
//! approaches to this problem and you may import anything from the std library accordingly.
//!
//! Refer `graph_grade.rs` for test cases.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

use rand::seq::index;

#[derive(PartialEq, Eq, Debug)]
enum VisitStatus {
    Unvisited,
    Visiting,
    Visited,
}

/// Handle to a graph node.
///
/// `NodeHandle` should implement `Clone`, which clones the handle without cloning the underlying
/// node. That is, there can be multiple handles to the same node.
/// The user can access the node through a handle if it does not violate Rust's aliasing rules.
///
/// You can freely add fields to this struct.
#[derive(Debug, Clone)]
pub struct NodeHandle {
    // Rc 让多个 handle 共享一个Node
    // RefCell 让我们可以通过 &self 修改 Node 内部的 edges
    inner: Rc<RefCell<Node>>,
}
impl PartialEq for NodeHandle {
    fn eq(&self, other: &Self) -> bool {
        // 比较两个 Rc 是否指向同一个内存地址
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}
impl Eq for NodeHandle {}
impl Hash for NodeHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let ptr = Rc::as_ptr(&self.inner);
        //使用 Rc 指向的内存地址作为哈希值
        ptr.hash(state);
    }
}
// Node持有对邻居(to)的强引用(Rc)，会导致循环引用，需要丢弃图之前手动断开所有边
#[derive(Debug)]
struct Node {
    value: i32,
    edges: Vec<NodeHandle>,
}

/// Error type for graph operations.
#[derive(Debug)]
pub struct GraphError;

/// Subgraph
///
/// You can freely add fields to this struct.
#[derive(Debug)]
pub struct SubGraph {
    // HashSet，方便快速查找和去重
    nodes: HashSet<NodeHandle>,
}

impl NodeHandle {
    /// Creates a node and returns the handle to it.
    pub fn new(value: i32) -> Self {
        let node = Node {
            value,
            edges: Vec::new(),
        };
        NodeHandle {
            inner: Rc::new(RefCell::new(node)),
        }
    }

    /// Adds an edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`. Returns `Ok(true)` if the edge is successfully added.
    /// Returns `Ok(false)` if an edge to `to` already exits.
    pub fn add_edge(&self, to: NodeHandle) -> Result<bool, GraphError> {
        let mut node_data = self.inner.try_borrow_mut().map_err(|_| GraphError)?;
        let position = node_data.edges.iter().position(|e| e.eq(&to));

        match position {
            Some(index) => Ok(false),
            None => {
                node_data.edges.push(to);
                Ok(true)
            }
        }
    }

    /// Removes the edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`. Returns `Ok(true)` if the edge is successfully removed.
    /// Returns `Ok(false)` if an edge to `to` does not exist.
    pub fn remove_edge(&self, to: &NodeHandle) -> Result<bool, GraphError> {
        // 如果该节点目前正被其他地方借用（例如正在遍历检查环），则返回 GraphError
        let mut node_data = self.inner.try_borrow_mut().map_err(|_| GraphError)?;

        // 在edges中寻找指向'to' 节点的索引,比较内存地址（指针），判断是否指向同一个对象
        let position = node_data.edges.iter().position(|e| e.eq(to));

        match position {
            Some(index) => {
                drop(node_data.edges.remove(index));
                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Removes all edges.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`.
    pub fn clear_edges(&self) -> Result<(), GraphError> {
        let mut node_data = self.inner.try_borrow_mut().map_err(|_| GraphError)?;
        node_data.edges.clear();
        Ok(())
    }
}

impl Default for SubGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SubGraph {
    /// Creates a new subgraph.
    pub fn new() -> Self {
        SubGraph {
            nodes: HashSet::new(),
        }
    }

    /// Adds a node to the subgraph. Returns true iff the node is newly added.
    pub fn add_node(&mut self, node: NodeHandle) -> bool {
        self.nodes.insert(node)
    }

    /// Removes a node from the subgraph. Returns true iff the node is successfully removed.
    pub fn remove_node(&mut self, node: &NodeHandle) -> bool {
        self.nodes.remove(node)
    }

    /// Returns true iff the subgraph contains a cycle. Nodes that do not belong to this subgraph
    /// are ignored. See <https://en.wikipedia.org/wiki/Cycle_(graph_theory)> for an algorithm.
    pub fn detect_cycle(&self) -> bool {
        // hard
        // 深度优先搜索（DFS），并配合三色标记法
        // 同一个节点可能属于多个子图，不要把访问状态放在Node中
        // 注意不要在遍历过程中产生RefCell的借用冲突(BorrowError)
        // 确保只检测属于当前 SubGraph 的节点
        let mut status: HashMap<NodeHandle, VisitStatus> = HashMap::new();
        // 初始化所有节点为Unvisited,存放到status这张子图访问状态表中
        for node in &self.nodes {
            let _ = status.insert(node.clone(), VisitStatus::Unvisited);
        }

        for node in &self.nodes {
            // 如果是新的连通分支，则进行dfs搜索cycle
            if let Some(VisitStatus::Unvisited) = status.get(node) {
                if self.has_cycle_dfs(node, &mut status) {
                    return true;
                }
            }
        }

        false
    }

    // DFS 辅助函数
    // 对初始点开始做dfs，改变访问状态state，并返回是否子图中存在环经过初始点
    fn has_cycle_dfs(
        &self,
        curr: &NodeHandle,
        status: &mut HashMap<NodeHandle, VisitStatus>,
    ) -> bool {
        let _ = status.insert(curr.clone(), VisitStatus::Visiting);
        let node_data = curr.inner.borrow();

        for neighbor in &node_data.edges {
            // 筛选出在SubGraph中的邻居
            if let Some(s) = status.get(neighbor) {
                match s {
                    VisitStatus::Visiting => return true,
                    VisitStatus::Unvisited => {
                        if self.has_cycle_dfs(neighbor, status) {
                            return true;
                        }
                        // 此时说明邻居没有环经过,则没有环同时包含current和neighbor
                    }
                    VisitStatus::Visited => {} //该邻居没有环经过, 则没有环同时包含current和neighbor
                }
            }
        }

        // 从current开始做完了dfs，且说明对任一邻居neighbor,没有环同时经过current和neighbor，说明没有环经过current
        // 将当前节点标记为Visited表示current已完成dfs且验证了没有环经过，返回false表示并且没有环经过current
        let _ = status.insert(curr.clone(), VisitStatus::Visited);
        false
    }
}

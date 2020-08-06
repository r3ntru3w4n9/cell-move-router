use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UnionFindNode {
    /// the head of current union
    head: usize,
    /// the height of current tree, used only if the node is root
    height: usize,
}

/// Stores the data needed in union-find algorithm
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct UnionFind {
    /// nodes used in union-find
    pub nodes: Vec<UnionFindNode>,
}

impl UnionFindNode {
    /// Create a new node in UnionFind data structure.
    pub fn new() -> Self {
        Self::default()
    }
}

impl UnionFind {
    /// Creates a new UnionFind data structure.
    pub fn new(size: usize) -> Self {
        Self {
            nodes: (0..size)
                .map(|idx| UnionFindNode {
                    head: idx,
                    height: 0,
                })
                .collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn done(&self) -> bool {
        (0..self.len())
            .map(|idx| self.find(idx))
            .collect::<HashSet<_>>()
            .len()
            == 1
    }

    /// Check if two values are in the same group.
    pub fn grouped(&self, a: usize, b: usize) -> Option<bool> {
        Some(self.find(a)? == self.find(b)?)
    }

    /// Wrapper function for `get`.
    pub fn get(&self, index: usize) -> Option<&UnionFindNode> {
        self.nodes.get(index)
    }

    /// Wrapper function for `get_mut`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut UnionFindNode> {
        self.nodes.get_mut(index)
    }

    /// Finds the root of the current bound tree.
    /// Does not apply path compression.
    pub fn find(&self, index: usize) -> Option<usize> {
        let head = self.get(index)?.head;

        if head == index {
            return Some(index);
        }

        self.find(head)
    }

    /// Finds the root of the current bound tree.
    /// Applies path compression.
    pub fn find_mut(&mut self, index: usize) -> Option<usize> {
        let ans = self.find(index);

        self.get_mut(index)?.head = ans.expect("Index out of bounds");

        ans
    }

    /// Joins two different unions.
    /// Returns `Some(())` if operation is successful.
    /// Returns `None` if an array's bounds check failed.
    pub fn join(&mut self, a: usize, b: usize) -> Option<()> {
        let heighta = self.get(a)?.height;
        let heightb = self.get(b)?.height;

        if heighta > heightb {
            self.get_mut(b)?.head = a;
        } else {
            self.get_mut(a)?.head = b;
            if heighta == heightb {
                self.get_mut(b)?.height += 1;
            }
        }

        Some(())
    }

    /// Unions two different disjoint sets.
    /// Returns true if a, b were unioned in this function.
    /// Returns false if a, b are already joined before this method is called.
    /// Panics if an Index out of bounds error is encountered
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        self.union_checked(a, b).expect("Index out of bounds")
    }

    /// Unions two different disjoint sets.
    /// Returns Option<bool> which requires manual unwrap.
    /// `union` wraps this function.
    pub fn union_checked(&mut self, a: usize, b: usize) -> Option<bool> {
        let heada = self.find_mut(a)?;
        let headb = self.find_mut(b)?;

        if heada == headb {
            return Some(false);
        }

        debug_assert!(!self.grouped(a, b)?);

        self.join(heada, headb)?;

        debug_assert!(self.grouped(a, b)?);

        Some(true)
    }
}

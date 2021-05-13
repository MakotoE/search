#![feature(drain_filter)]
#![feature(test)]

mod benchmark;

use std::fmt::Debug;
use std::hash::Hash;
use std::mem::replace;

trait Set<T>: Debug
where
    T: Hash,
{
    fn contains(&self, item: &T) -> bool;
    fn len(&self) -> usize;
    fn insert(&mut self, item: T);
}

#[derive(Debug, Default)]
pub struct SimpleArray<T> {
    arr: Vec<T>,
}

impl<T: Hash + Debug + PartialEq> Set<T> for SimpleArray<T> {
    fn contains(&self, item: &T) -> bool {
        self.arr.iter().any(|i| i == item)
    }

    fn len(&self) -> usize {
        self.arr.len()
    }

    fn insert(&mut self, item: T) {
        if !self.contains(&item) {
            self.arr.push(item);
        }
    }
}

#[derive(Debug, Default)]
pub struct OrderedArray<T> {
    arr: Vec<T>,
}

impl<T: Hash + Debug + PartialEq + PartialOrd> Set<T> for OrderedArray<T> {
    fn contains(&self, item: &T) -> bool {
        if self.arr.len() == 0 {
            return false;
        }

        let mut left = 0_usize;
        let mut right = self.arr.len() - 1;
        while left <= right {
            let middle = (left + right) / 2;
            if &self.arr[middle] < item {
                left = middle + 1;
            } else if &self.arr[middle] > item {
                if middle == 0 {
                    break;
                }
                right = middle - 1;
            } else {
                return true;
            }
        }

        return false;
    }

    fn len(&self) -> usize {
        self.arr.len()
    }

    fn insert(&mut self, item: T) {
        if self.arr.len() == 0 {
            self.arr.insert(0, item);
            return;
        }

        let mut left = 0_usize;
        let mut right = self.arr.len() - 1;
        while left <= right {
            let middle = (left + right) / 2;
            if self.arr[middle] < item {
                left = middle + 1;
            } else if self.arr[middle] > item {
                if middle == 0 {
                    break;
                }
                right = middle - 1;
            } else {
                return;
            }
        }

        self.arr.insert(left, item);
    }
}

#[derive(Debug, Default)]
pub struct LinkedBinaryTree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    item: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(item: i32) -> Self {
        Self {
            item,
            left: None,
            right: None,
        }
    }
}

impl LinkedBinaryTree {
    /// Adds item to a distant child of node and balances it
    fn node_insert(mut node: Box<Node>, item: i32) -> Box<Node> {
        if item == node.item {
            return node;
        }

        if item < node.item {
            if let Some(left_node) = node.left.take() {
                node.left = Some(LinkedBinaryTree::node_insert(left_node, item));
                if LinkedBinaryTree::balance_factor(&node) > 1 {
                    if LinkedBinaryTree::balance_factor(node.left.as_ref().unwrap()) > 0 {
                        // Left left
                        node = LinkedBinaryTree::right_rotate(node);
                    } else {
                        // Left right
                        node.left = Some(LinkedBinaryTree::left_rotate(node.left.unwrap()));
                        node = LinkedBinaryTree::right_rotate(node);
                    }
                }
            } else {
                node.left = Some(Box::new(Node::new(item)));
            }
        } else {
            if let Some(right_node) = node.right.take() {
                node.right = Some(LinkedBinaryTree::node_insert(right_node, item));
                if LinkedBinaryTree::balance_factor(&node) < 1 {
                    if LinkedBinaryTree::balance_factor(node.right.as_ref().unwrap()) > 0 {
                        // Right left
                        node.right = Some(LinkedBinaryTree::right_rotate(node.right.unwrap()));
                        node = LinkedBinaryTree::left_rotate(node);
                    } else {
                        // Right right
                        node = LinkedBinaryTree::left_rotate(node);
                    }
                }
            } else {
                node.right = Some(Box::new(Node::new(item)));
            }
        }

        return node;
    }

    fn balance_factor(node: &Node) -> isize {
        LinkedBinaryTree::node_height(&node.left) as isize
            - LinkedBinaryTree::node_height(&node.right) as isize
    }

    fn node_height(node: &Option<Box<Node>>) -> usize {
        if let Some(n) = node {
            usize::max(
                LinkedBinaryTree::node_height(&n.left),
                LinkedBinaryTree::node_height(&n.right),
            ) + 1
        } else {
            0
        }
    }

    fn right_rotate(mut node: Box<Node>) -> Box<Node> {
        let mut pivot = std::mem::take(&mut node.left).unwrap();
        node.left = pivot.right;
        pivot.right = Some(node);
        pivot
    }

    fn left_rotate(mut node: Box<Node>) -> Box<Node> {
        let mut pivot = std::mem::take(&mut node.right).unwrap();
        node.right = pivot.left;
        pivot.left = Some(node);
        pivot
    }

    fn count_nodes(node: &Node) -> usize {
        let mut total = 1_usize;
        if let Some(left) = &node.left {
            total += LinkedBinaryTree::count_nodes(left);
        }
        if let Some(right) = &node.right {
            total += LinkedBinaryTree::count_nodes(right);
        }
        total
    }

    fn get_min(node: &Node) -> i32 {
        let mut curr = node;
        while let Some(left) = &curr.left {
            curr = left;
        }
        return curr.item;
    }
}

impl Set<i32> for LinkedBinaryTree {
    fn contains(&self, item: &i32) -> bool {
        let mut current = &self.root;
        while let Some(curr) = current {
            if item == &curr.item {
                return true;
            } else if item < &curr.item {
                current = &curr.left;
            } else {
                current = &curr.right;
            }
        }

        return false;
    }

    fn len(&self) -> usize {
        if let Some(root) = &self.root {
            LinkedBinaryTree::count_nodes(root)
        } else {
            0
        }
    }

    fn insert(&mut self, item: i32) {
        if let Some(root) = self.root.take() {
            self.root = Some(LinkedBinaryTree::node_insert(root, item));
        } else {
            self.root = Some(Box::new(Node::new(item)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    macro_rules! set {
        ( $set_type:ty ) => {
            (|| Box::new(<$set_type>::default()), stringify!($set_type))
        };
    }

    lazy_static! {
        static ref SETS: [(fn() -> Box<dyn Set<i32>>, &'static str); 3] = [
            set!(SimpleArray<i32>),
            set!(OrderedArray<i32>),
            set!(LinkedBinaryTree),
        ];
    }

    #[test]
    fn test() {
        for (constructor, name) in SETS.iter() {
            let mut set = constructor();
            assert!(!set.contains(&0), "{}", name);
            set.insert(0);
            set.insert(0);
            assert_eq!(set.len(), 1, "{}", name);
            set.insert(2);
            assert_eq!(set.len(), 2, "{}", name);
            set.insert(1);
            assert_eq!(set.len(), 3, "{}", name);
            assert!(set.contains(&0), "{}", name);
            assert!(set.contains(&1), "{}", name);
            assert!(set.contains(&2), "{}", name);
        }
    }
}

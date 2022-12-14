#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    // BOX smart pointer stores data on the heap instead of the stack
    // Stack Last in First Out Format
    // Data on the stack must have a defined fixed size
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>, 
        pub right: Option<Box<TreeNode<T>>>, 
        pub key: T, 
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode { left: None, right: None, key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) ->
        Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) ->
        Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));

}

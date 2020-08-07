use super::*;
use std::mem;

impl<T> IntervalNode<T>
    where T: PartialEq + Clone{

    fn node_height(&self) -> isize{
        let left_height = match &self.left{
            Some(node) => node.node_height(),
            None => 0,
        };

        let right_height = match &self.right{
            Some(node) => node.node_height(),
            None => 0,
        };

        1 + std::cmp::max(left_height, right_height)
    }

    fn node_height_diff(&self) -> isize{
        let left_height = match &self.left{
            Some(node) => node.node_height(),
            None => 0,
        };

        let right_height = match &self.right{
            Some(node) => node.node_height(),
            None => 0,
        };

        left_height - right_height
    }

    /// Returns None if it has no right node.
    /// If a right node returns None, this will deconstruct
    /// said node and return it up.
    fn right_most_mut(&mut self) -> Option<Box<IntervalNode<T>>>{
        let r_node = match &mut self.right{
            Some(node) => self.right_most_mut(),
            None => return None,
        };

        match r_node{
            Some(node) => return Some(node),
            None => mem::replace(&mut self.right, None),
        }
    }

    /// Returns None if it has no left node.
    /// If a left node returns None, this will deconstruct
    /// said node and return it up.
    fn left_most_mut(&mut self) -> Option<Box<IntervalNode<T>>>{
        let l_node = match &mut self.left{
            Some(node) => self.left_most_mut(),
            None => return None,
        };

        match l_node{
            Some(node) => return Some(node),
            None => mem::replace(&mut self.left, None),
        }
    }
}
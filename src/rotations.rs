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
    fn right_most(&mut self) -> Option<Box<IntervalNode<T>>>{
        let r_node = match &mut self.right{
            Some(node) => self.right_most(),
            None => return None,
        };

        match r_node{
            Some(node) => return Some(node),
            None => {
                let mut ret_node = mem::replace(&mut self.right, None);
                mem::swap(&mut self.right, &mut ret_node.as_mut().unwrap().left);
                ret_node
            },
        }
    }

    /// Returns None if it has no left node.
    /// If a left node returns None, this will deconstruct
    /// said node and return it up.
    fn left_most(&mut self) -> Option<Box<IntervalNode<T>>>{
        let l_node = match &mut self.left{
            Some(node) => self.left_most(),
            None => return None,
        };

        match l_node{
            Some(node) => return Some(node),
            None =>{
                let mut ret_node = mem::replace(&mut self.left, None);
                mem::swap(&mut self.left, &mut ret_node.as_mut().unwrap().right);
                ret_node
            },
        }
    }

    pub fn rotate(&mut self){
        let diff = self.node_height_diff();

        // Remove the correct node from the tree
        let mut new_self = match diff{
            left if left > 1 => match &mut self.left{
                Some(node) => {
                    match node.right_most(){
                        Some(node) => node,
                        None => return,
                    }
                },
                None => return,
            }
            right if right < -1 => match &mut self.right{
                Some(node) =>{
                    match node.left_most(){
                        Some(node) => node,
                        None => return,
                    }
                },
                None => return,
            }
            _ => return,
        };

        // Swap all the values of the node
        mem::swap(&mut self.element, &mut new_self.element);
        mem::swap(&mut self.count, &mut new_self.count);
        mem::swap(&mut self.start, &mut new_self.start);

        match diff{
            left if left > 1 =>{
                mem::swap(&mut new_self.right, &mut self.right);
                self.right = Some(new_self);
            },
            right if right < -1 =>{
                mem::swap(&mut new_self.left, &mut self.left);
                self.left = Some(new_self);
            }
            _ => return,
        }
    }
}
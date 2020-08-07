use super::*;

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
}
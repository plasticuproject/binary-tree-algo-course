#[allow(unused_imports)]
use crate::node::{Node, NodeRef};
use crate::zero::Zero;
use std::ops::{Add, AddAssign};

#[allow(dead_code)]
trait Dfs<T> {
    fn depth_first_recursive(&self) -> T;
}

impl<T> Dfs<T> for NodeRef<T>
where
    T: Clone + PartialOrd + AddAssign<T> + Add<Output = T> + Zero,
{
    fn depth_first_recursive(&self) -> T {
        let node = self.borrow();
        let mut max_sum = node.val.clone();
        let left_sum = node
            .left
            .as_ref()
            .map_or_else(T::zero, Dfs::depth_first_recursive);
        let right_sum = node
            .right
            .as_ref()
            .map_or_else(T::zero, Dfs::depth_first_recursive);

        max_sum += if left_sum > right_sum {
            left_sum
        } else {
            right_sum
        };

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //      5
    //     / \
    //   11   3
    //   / \   \
    //  4   2   1

    fn create_tree() -> Vec<NodeRef<i32>> {
        let a = Node::new(5);
        let b = Node::new(11);
        let c = Node::new(3);
        let d = Node::new(4);
        let e = Node::new(2);
        let f = Node::new(1);

        a.borrow_mut().insert_left(b.to_owned());
        a.borrow_mut().insert_right(c.to_owned());
        b.borrow_mut().insert_left(d.to_owned());
        b.borrow_mut().insert_right(e.to_owned());
        c.borrow_mut().insert_right(f.to_owned());

        vec![a, b, c, d, e, f]
    }

    #[test]
    fn depth_first_recursive_test() {
        let tree = create_tree();

        assert_eq!(tree[0].depth_first_recursive(), 20);
        assert_eq!(tree[1].depth_first_recursive(), 15);
        assert_eq!(tree[2].depth_first_recursive(), 4);
        assert_eq!(tree[3].depth_first_recursive(), 4);
    }
}

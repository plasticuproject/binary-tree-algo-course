#[allow(unused_imports)]
use crate::node::{Node, NodeRef};
use crate::zero::Zero;
use std::collections::VecDeque;
use std::ops::{Add, AddAssign};

#[allow(dead_code)]
trait Dfs<T> {
    fn depth_first_sum_iterative(&self) -> T;
    fn depth_first_sum_recursive(&self) -> T;
}

#[allow(dead_code)]
trait Bfs<T> {
    fn breadth_first_sum(&self) -> T;
}

impl<T> Dfs<T> for NodeRef<T>
where
    T: Clone + AddAssign<T> + Add<Output = T> + Zero,
{
    fn depth_first_sum_iterative(&self) -> T {
        let mut value = T::zero();
        let mut stack = VecDeque::new();

        stack.push_back(self.clone());

        while let Some(current) = stack.pop_back() {
            let current_borrowed = current.borrow();

            value += current_borrowed.val.clone();

            if let Some(left) = &current_borrowed.left {
                stack.push_back(left.to_owned());
            }
            if let Some(right) = &current_borrowed.right {
                stack.push_back(right.to_owned());
            }
        }

        value
    }

    fn depth_first_sum_recursive(&self) -> T {
        let mut value = T::zero();

        value += self.borrow().val.clone();

        if let Some(right) = &self.borrow().right {
            value += right.depth_first_sum_recursive();
        }
        if let Some(left) = &self.borrow().left {
            value += left.depth_first_sum_recursive();
        }

        value
    }
}

impl<T> Bfs<T> for NodeRef<T>
where
    T: Clone + AddAssign<T> + Add<Output = T> + Zero,
{
    fn breadth_first_sum(&self) -> T {
        let mut value = T::zero();
        let mut queue = VecDeque::new();

        queue.push_back(self.clone());

        while let Some(current) = queue.pop_front() {
            let current_borrowed = current.borrow();
            value += current_borrowed.val.clone();

            if let Some(left) = &current_borrowed.left {
                queue.push_back(left.to_owned());
            }
            if let Some(right) = &current_borrowed.right {
                queue.push_back(right.to_owned());
            }
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //      3
    //     / \
    //   11   4
    //   / \   \
    //  4   2   1

    fn create_tree() -> Vec<NodeRef<i32>> {
        let a = Node::new(3);
        let b = Node::new(11);
        let c = Node::new(4);
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
    fn depth_first_sum_iterative_test() {
        let tree = create_tree();

        assert_eq!(tree[0].depth_first_sum_iterative(), 25);
        assert_eq!(tree[1].depth_first_sum_iterative(), 17);
        assert_eq!(tree[2].depth_first_sum_iterative(), 5);
        assert_eq!(tree[3].depth_first_sum_iterative(), 4);
    }

    #[test]
    fn depth_first_sum_recursive_test() {
        let tree = create_tree();

        assert_eq!(tree[0].depth_first_sum_recursive(), 25);
        assert_eq!(tree[1].depth_first_sum_recursive(), 17);
        assert_eq!(tree[2].depth_first_sum_recursive(), 5);
        assert_eq!(tree[3].depth_first_sum_recursive(), 4);
    }

    #[test]
    fn breadth_first_sum_test() {
        let tree = create_tree();

        assert_eq!(tree[0].breadth_first_sum(), 25);
        assert_eq!(tree[1].breadth_first_sum(), 17);
        assert_eq!(tree[2].breadth_first_sum(), 5);
        assert_eq!(tree[3].breadth_first_sum(), 4);
    }
}

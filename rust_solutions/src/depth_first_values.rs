#[allow(unused_imports)]
use crate::node::{Node, NodeRef};
use std::collections::VecDeque;

#[allow(dead_code)]
trait Dfs {
    fn depth_first_traversal_iterative(&self) -> Vec<String>;
    fn depth_first_traversal_recursive(&self) -> Vec<String>;
}

impl Dfs for NodeRef {
    fn depth_first_traversal_iterative(&self) -> Vec<String> {
        let mut values = Vec::new();
        let mut stack = VecDeque::new();

        stack.push_back(self.clone());

        while let Some(current) = stack.pop_back() {
            let current_borrowed = current.borrow();
            values.push(current_borrowed.val.clone());

            if let Some(left) = &current_borrowed.left {
                stack.push_back(left.to_owned());
            }
            if let Some(right) = &current_borrowed.right {
                stack.push_back(right.to_owned());
            }
        }

        values
    }

    fn depth_first_traversal_recursive(&self) -> Vec<String> {
        let mut values = Vec::new();
        values.push(self.borrow().val.clone());
        if let Some(right) = &self.borrow().right {
            values.extend(right.depth_first_traversal_recursive());
        }
        if let Some(left) = &self.borrow().left {
            values.extend(left.depth_first_traversal_recursive());
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //      a
    //     / \
    //    b   c
    //   / \   \
    //  d   e   f

    fn create_tree() -> NodeRef {
        let a = Node::new("a");
        let b = Node::new("b");
        let c = Node::new("c");
        let d = Node::new("d");
        let e = Node::new("e");
        let f = Node::new("f");

        a.borrow_mut().insert_left(b.to_owned());
        a.borrow_mut().insert_right(c.to_owned());
        b.borrow_mut().insert_left(d.to_owned());
        b.borrow_mut().insert_right(e.to_owned());
        c.borrow_mut().insert_right(f.to_owned());

        a
    }

    #[test]
    fn depth_first_traversal_iterative_test() {
        let tree = create_tree();
        let result = tree.depth_first_traversal_iterative();
        assert_eq!(result, vec!["a", "c", "f", "b", "e", "d"]);
    }

    #[test]
    fn depth_first_traversal_recursive_test() {
        let tree = create_tree();
        let result = tree.depth_first_traversal_recursive();
        assert_eq!(result, vec!["a", "c", "f", "b", "e", "d"]);
    }
}

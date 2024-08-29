#[allow(unused_imports)]
use crate::node::{Node, NodeRef};
use std::collections::VecDeque;

#[allow(dead_code)]
trait Dfs<T> {
    fn depth_first_traversal_iterative(&self) -> Vec<T>;
    fn depth_first_traversal_recursive(&self) -> Vec<T>;
}

impl<T: Clone> Dfs<T> for NodeRef<T> {
    fn depth_first_traversal_iterative(&self) -> Vec<T> {
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

    fn depth_first_traversal_recursive(&self) -> Vec<T> {
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

    fn create_tree() -> NodeRef<String> {
        let a = Node::new("a".to_string());
        let b = Node::new("b".to_string());
        let c = Node::new("c".to_string());
        let d = Node::new("d".to_string());
        let e = Node::new("e".to_string());
        let f = Node::new("f".to_string());

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

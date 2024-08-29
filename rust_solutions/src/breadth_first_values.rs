#[allow(unused_imports)]
use crate::node::{Node, NodeRef};
use std::collections::VecDeque;

#[allow(dead_code)]
trait Bfs<T> {
    fn breadth_first_traversal_iterative(&self) -> Vec<T>;
}

impl<T: Clone> Bfs<T> for NodeRef<T> {
    fn breadth_first_traversal_iterative(&self) -> Vec<T> {
        let mut values = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(self.clone());

        while let Some(current) = queue.pop_front() {
            let current_borrowed = current.borrow();
            values.push(current_borrowed.val.clone());

            if let Some(left) = &current_borrowed.left {
                queue.push_back(left.to_owned());
            }
            if let Some(right) = &current_borrowed.right {
                queue.push_back(right.to_owned());
            }
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
    fn breadth_first_traversal_iterative_test() {
        let tree = create_tree();
        let result = tree.breadth_first_traversal_iterative();
        assert_eq!(result, vec!["a", "b", "c", "d", "e", "f"]);
    }
}

#[allow(unused_imports)]
use crate::node::{Node, NodeRef};
use std::collections::VecDeque;

#[allow(dead_code)]
trait Bfs {
    fn breadth_first_traversal_iterative(&self) -> Vec<String>;
}

impl Bfs for NodeRef {
    fn breadth_first_traversal_iterative(&self) -> Vec<String> {
        let mut values = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(self.clone());

        while let Some(current) = queue.pop_front() {
            let current_borrowed = current.borrow();
            values.push(current_borrowed.val.clone());

            if let Some(left) = &current_borrowed.left {
                queue.push_back(left.clone());
            }
            if let Some(right) = &current_borrowed.right {
                queue.push_back(right.clone());
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

    fn create_tree() -> NodeRef {
        let a = Node::new("a");
        let b = Node::new("b");
        let c = Node::new("c");
        let d = Node::new("d");
        let e = Node::new("e");
        let f = Node::new("f");

        a.borrow_mut().insert_left(b.clone());
        a.borrow_mut().insert_right(c.clone());
        b.borrow_mut().insert_left(d.clone());
        b.borrow_mut().insert_right(e.clone());
        c.borrow_mut().insert_right(f.clone());

        a
    }

    #[test]
    fn breadth_first_traversal_iterative_test() {
        let tree = create_tree();
        let result = tree.breadth_first_traversal_iterative();
        assert_eq!(result, vec!["a", "b", "c", "d", "e", "f"]);
    }
}

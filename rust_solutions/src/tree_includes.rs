#[allow(unused_imports)]
use crate::node::{Node, NodeRef};
use std::collections::VecDeque;

#[allow(dead_code)]
trait Dfs<T> {
    fn depth_first_includes_iterative(&self, target: T) -> bool;
    fn depth_first_includes_recursive(&self, target: T) -> bool;
}

#[allow(dead_code)]
trait Bfs<T> {
    fn breadth_first_includes(&self, target: T) -> bool;
}

impl<T: Clone + PartialEq> Dfs<T> for NodeRef<T> {
    fn depth_first_includes_iterative(&self, target: T) -> bool {
        let mut stack = VecDeque::new();

        stack.push_back(self.clone());

        while let Some(current) = stack.pop_back() {
            let current_borrowed = current.borrow();

            if current_borrowed.val == target {
                return true;
            }
            if let Some(left) = &current_borrowed.left {
                stack.push_back(left.to_owned());
            }
            if let Some(right) = &current_borrowed.right {
                stack.push_back(right.to_owned());
            }
        }

        false
    }

    fn depth_first_includes_recursive(&self, target: T) -> bool {
        if self.borrow().val == target {
            return true;
        }
        if let Some(right) = &self.borrow().right {
            if right.depth_first_includes_recursive(target.clone()) {
                return true;
            }
        }
        if let Some(left) = &self.borrow().left {
            if left.depth_first_includes_recursive(target) {
                return true;
            }
        }

        false
    }
}

impl<T: Clone + PartialEq> Bfs<T> for NodeRef<T> {
    fn breadth_first_includes(&self, target: T) -> bool {
        let mut queue = VecDeque::new();

        queue.push_back(self.clone());

        while let Some(current) = queue.pop_front() {
            let current_borrowed = current.borrow();

            if current_borrowed.val == target {
                return true;
            }
            if let Some(left) = &current_borrowed.left {
                queue.push_back(left.to_owned());
            }
            if let Some(right) = &current_borrowed.right {
                queue.push_back(right.to_owned());
            }
        }

        false
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
    fn depth_first_includes_iterative_test() {
        let tree = create_tree();

        assert!(tree.depth_first_includes_iterative("e".to_string()));
        assert!(!tree.depth_first_includes_iterative("g".to_string()));
    }

    #[test]
    fn depth_first_includes_recursive_test() {
        let tree = create_tree();

        assert!(tree.depth_first_includes_recursive("e".to_string()));
        assert!(!tree.depth_first_includes_recursive("g".to_string()));
    }

    #[test]
    fn breadth_first_includes_test() {
        let tree = create_tree();

        assert!(tree.breadth_first_includes("e".to_string()));
        assert!(!tree.breadth_first_includes("g".to_string()));
    }
}

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub val: T,
    pub left: Option<NodeRef<T>>,
    pub right: Option<NodeRef<T>>,
}

#[allow(clippy::module_name_repetitions)]
pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

impl<T> Node<T> {
    #[must_use]
    pub fn new(val: T) -> NodeRef<T> {
        Rc::new(RefCell::new(Self {
            val,
            left: None,
            right: None,
        }))
    }

    pub fn insert_left(&mut self, child: NodeRef<T>) {
        self.left = Some(child);
    }

    pub fn insert_right(&mut self, child: NodeRef<T>) {
        self.right = Some(child);
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

    fn create_tree() -> Vec<NodeRef<String>> {
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

        vec![a, b, c, d, e, f]
    }

    #[test]
    fn node_test() {
        let tree = create_tree();
        assert_eq!(tree.len(), 6);
    }
}

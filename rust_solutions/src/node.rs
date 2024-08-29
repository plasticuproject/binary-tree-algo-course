use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node {
    pub val: String,
    pub left: Option<NodeRef>,
    pub right: Option<NodeRef>,
}

#[allow(clippy::module_name_repetitions)]
pub type NodeRef = Rc<RefCell<Node>>;

impl Node {
    #[must_use]
    pub fn new(val: &str) -> NodeRef {
        Rc::new(RefCell::new(Self {
            val: val.to_string(),
            left: None,
            right: None,
        }))
    }

    pub fn insert_left(&mut self, child: NodeRef) {
        self.left = Some(child);
    }

    pub fn insert_right(&mut self, child: NodeRef) {
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

    fn create_tree() -> Vec<NodeRef> {
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

        vec![a, b, c, d, e, f]
    }

    #[test]
    fn node_test() {
        let tree = create_tree();
        assert_eq!(tree.len(), 6);
    }
}

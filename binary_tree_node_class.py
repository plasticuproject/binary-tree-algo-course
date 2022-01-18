"""binary_tree_node_class.py"""
from typing import Optional


class Node:  # pylint: disable=too-few-public-methods
    """Node class for binary tree structure."""

    def __init__(self, val: Optional[str]) -> None:
        self.val = val
        self.left: Optional[Node] = None
        self.right: Optional[Node] = None


#     a
#    / \
#   b   c
#  / \   \
# d   e   f

a = Node("a")
b = Node("b")
c = Node("c")
d = Node("d")
e = Node("e")
f = Node("f")

a.left = b
a.right = c
b.left = d
b.right = e
c.right = f

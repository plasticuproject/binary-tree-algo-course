"""depth_first_values.py"""
from collections import deque
from typing import Optional, List, Deque


class Node:  # pylint: disable=too-few-public-methods
    """Node class for binary tree structure."""

    def __init__(self, val: Optional[str]) -> None:
        self.val = val
        self.left: Optional[Node] = None
        self.right: Optional[Node] = None


def breadth_first_traversal_iterative(
        root: Optional[Node]) -> List[Optional[str]]:
    """Breadth-first iterative binary tree traversal. Returns
    list of nodes in order traversed."""
    values: List[Optional[str]] = []
    if root is None:
        return values
    queue: Deque[Node] = deque([root])
    while queue:
        current: Node = queue.popleft()
        values.append(current.val)
        if current.left:
            queue.append(current.left)
        if current.right:
            queue.append(current.right)
    return values


# Binary Tree Structure

#     a
#    / \
#   b   c
#  / \   \
# d   e   f

a = Node('a')
b = Node('b')
c = Node('c')
d = Node('d')
e = Node('e')
f = Node('f')

a.left = b
a.right = c
b.left = d
b.right = e
c.right = f

# TESTS
assert breadth_first_traversal_iterative(a) == ["a", "b", "c", "d", "e", "f"]
assert not breadth_first_traversal_iterative(None)

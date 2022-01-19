"""depth_first_values.py"""
from timeit import timeit
from collections import deque
from typing import Optional, List, Deque


class Node:  # pylint: disable=too-few-public-methods
    """Node class for binary tree structure."""

    def __init__(self, val: Optional[str]) -> None:
        self.val = val
        self.left: Optional[Node] = None
        self.right: Optional[Node] = None


def depth_first_traversal_iterative(
        root: Optional[Node]) -> List[Optional[str]]:
    """Depth-first iterative binary tree traversal. Returns
    list of nodes in order traversed."""
    values: List[Optional[str]] = []
    if not root:
        return values
    stack: Deque[Node] = deque([root])
    while stack:
        current: Node = stack.pop()
        values.append(current.val)
        if current.right:
            stack.append(current.right)
        if current.left:
            stack.append(current.left)
    return values


def depth_first_traversal_recursive(
        root: Optional[Node]) -> List[Optional[str]]:
    """Depth-first recursive binary tree traversal. Returns
    list of nodes in order traversed."""
    values: List[Optional[str]] = []
    if not root:
        return values
    left_values = depth_first_traversal_recursive(root.left)
    right_values = depth_first_traversal_recursive(root.right)
    values = [root.val] + left_values + right_values
    return values


# Binary Tree Structure

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

# TESTS
assert depth_first_traversal_iterative(a) == ["a", "b", "d", "e", "c", "f"]
assert not depth_first_traversal_iterative(None)

assert depth_first_traversal_recursive(a) == ["a", "b", "d", "e", "c", "f"]
assert not depth_first_traversal_recursive(None)

# TIME TESTS
print("\ndepth_first_traversal_iterative:")
print(timeit(lambda: depth_first_traversal_iterative(a), number=1000000))
print("\ndepth_first_traversal_recursive:")
print(timeit(lambda: depth_first_traversal_recursive(a), number=1000000))

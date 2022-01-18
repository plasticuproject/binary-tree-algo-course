"""tree_includes.py"""
from timeit import timeit
from collections import deque
from typing import Optional, Deque


class Node:  # pylint: disable=too-few-public-methods
    """Node class for binary tree structure."""

    def __init__(self, val: Optional[str]) -> None:
        self.val = val
        self.left: Optional[Node] = None
        self.right: Optional[Node] = None


def breadth_first_includes(root: Optional[Node], target: str) -> bool:
    """Breadth-first iterative binary tree traversal. Returns
    True if target is found in Node, False otherwise."""
    if root is None:
        return False
    queue: Deque[Node] = deque([root])
    while queue:
        current: Node = queue.popleft()
        if current.val == target:
            return True
        if current.left:
            queue.append(current.left)
        if current.right:
            queue.append(current.right)
    return False


def depth_first_includes_iterative(root: Optional[Node], target: str) -> bool:
    """Depth-first iterative binary tree traversal. Returns
    True if target is found in Node, False otherwise."""
    if root is None:
        return False
    stack: Deque[Node] = deque([root])
    while stack:
        current: Node = stack.pop()
        if current.val == target:
            return True
        if current.right:
            stack.append(current.right)
        if current.left:
            stack.append(current.left)
    return False


def depth_first_includes_recursive(root: Optional[Node], target: str) -> bool:
    """Depth-first recursive binary tree traversal. Returns
    True if target is found in Node, False otherwise."""
    if root is None:
        return False
    if root.val == target:
        return True
    return (depth_first_includes_recursive(root.left, target)
            or depth_first_includes_recursive(root.right, target))


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
assert breadth_first_includes(a, "e")
assert not breadth_first_includes(a, "g")
assert not breadth_first_includes(None, "e")
assert not breadth_first_includes(None, "g")

assert depth_first_includes_iterative(a, "e")
assert not depth_first_includes_iterative(a, "g")
assert not depth_first_includes_iterative(None, "e")
assert not depth_first_includes_iterative(None, "g")

assert depth_first_includes_recursive(a, "e")
assert not depth_first_includes_recursive(a, "g")
assert not depth_first_includes_recursive(None, "e")
assert not depth_first_includes_recursive(None, "g")

# TIME TESTS
print("\nbreadth_first_includes:")
print(timeit(lambda: breadth_first_includes(a, "e"), number=1000000))
print("\ndepth_first_includes_iterative:")
print(timeit(lambda: depth_first_includes_iterative(a, "e"), number=1000000))
print("\ndepth_first_includes_recursive:")
print(timeit(lambda: depth_first_includes_recursive(a, "e"), number=1000000))

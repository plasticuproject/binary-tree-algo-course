"""tree_min_value.py"""
from timeit import timeit
from collections import deque
from typing import Optional, Deque


class Node:  # pylint: disable=too-few-public-methods
    """Node class for binary tree structure."""

    def __init__(self, val: Optional[int]) -> None:
        self.val = val
        self.left: Optional[Node] = None
        self.right: Optional[Node] = None


def breadth_first_min(root: Optional[Node]) -> Optional[int]:
    """Breadth-first iterative binary tree traversal. Returns
    Node of minimum value."""
    value: Optional[int] = None
    if root is None:
        return value
    value = 0
    if root.val:
        value += root.val
    queue: Deque[Node] = deque([root])
    while queue:
        current: Node = queue.popleft()
        if current.val and current.val < value:
            value = current.val
        if current.left:
            queue.append(current.left)
        if current.right:
            queue.append(current.right)
    return value


def depth_first_min_iterative(root: Optional[Node]) -> Optional[int]:
    """Depth-first iterative binary tree traversal. Returns
    Node of minimum value."""
    value: Optional[int] = None
    if root is None:
        return None
    value = 0
    if root.val:
        value += root.val
    stack: Deque[Node] = deque([root])
    while stack:
        current: Node = stack.pop()
        if current.val and current.val < value:
            value = current.val
        if current.right:
            stack.append(current.right)
        if current.left:
            stack.append(current.left)
    return value


def depth_first_min_recursive(root: Optional[Node]) -> Optional[int]:
    """Depth-first recursive binary tree traversal. Returns
    Node of minimum value."""
    value: Optional[int] = None
    if root is None:
        return value
    value = 0
    if root.val:
        value += root.val
    left_values = depth_first_min_recursive(root.left)
    if left_values and left_values < value:
        value = left_values
    right_values = depth_first_min_recursive(root.right)
    if right_values and right_values < value:
        value = right_values
    return value


# Binary Tree Structure

#     5
#    / \
#   11  3
#  /  \  \
# 4    15 12

a = Node(5)
b = Node(11)
c = Node(3)
d = Node(4)
e = Node(15)
f = Node(12)

a.left = b
a.right = c
b.left = d
b.right = e
c.right = f

# TESTS
assert breadth_first_min(a) == 3
assert breadth_first_min(b) == 4
assert breadth_first_min(c) == 3
assert breadth_first_min(d) == 4
assert not breadth_first_min(None)

assert depth_first_min_iterative(a) == 3
assert depth_first_min_iterative(b) == 4
assert depth_first_min_iterative(c) == 3
assert depth_first_min_iterative(d) == 4
assert not depth_first_min_iterative(None)

assert depth_first_min_recursive(a) == 3
assert depth_first_min_recursive(b) == 4
assert depth_first_min_recursive(c) == 3
assert depth_first_min_recursive(d) == 4
assert not depth_first_min_recursive(None)

# TIME TESTS
print("\nbreadth_first_min:")
print(timeit(lambda: breadth_first_min(a), number=1000000))
print("\ndepth_first_min_iterative:")
print(timeit(lambda: depth_first_min_iterative(a), number=1000000))
print("\ndepth_first_min_recursive:")
print(timeit(lambda: depth_first_min_recursive(a), number=1000000))

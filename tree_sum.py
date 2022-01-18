"""tree_sum.py"""
from collections import deque
from typing import Optional, Deque


class Node:  # pylint: disable=too-few-public-methods
    """Node class for binary tree structure."""

    def __init__(self, val: Optional[int]) -> None:
        self.val = val
        self.left: Optional[Node] = None
        self.right: Optional[Node] = None


def breadth_first_sum(root: Optional[Node]) -> Optional[int]:
    """Breadth-first iterative binary tree traversal. Returns
    sum of Nodes."""
    value: Optional[int] = None
    if root is None:
        return value
    value = 0
    queue: Deque[Node] = deque([root])
    while queue:
        current: Node = queue.popleft()
        if current.val:
            value += current.val
        if current.left:
            queue.append(current.left)
        if current.right:
            queue.append(current.right)
    return value


def depth_first_sum_iterative(root: Optional[Node]) -> Optional[int]:
    """Depth-first iterative binary tree traversal. Returns
    sum of Nodes."""
    value: Optional[int] = None
    if root is None:
        return value
    value = 0
    stack: Deque[Node] = deque([root])
    while stack:
        current: Node = stack.pop()
        if current.val:
            value += current.val
        if current.right:
            stack.append(current.right)
        if current.left:
            stack.append(current.left)
    return value


def depth_first_sum_recursive(root: Optional[Node]) -> Optional[int]:
    """Depth-first recursive binary tree traversal. Returns
    sum of Nodes."""
    value: Optional[int] = None
    if root is None:
        return value
    value = 0
    if root.val:
        value += root.val
    left_values = depth_first_sum_recursive(root.left)
    if left_values:
        value += left_values
    right_values = depth_first_sum_recursive(root.right)
    if right_values:
        value += right_values
    return value


# Binary Tree Structure

#     3
#    / \
#   11   4
#  /  \   \
# 4    2   1

a = Node(3)
b = Node(11)
c = Node(4)
d = Node(4)
e = Node(2)
f = Node(1)

a.left = b
a.right = c
b.left = d
b.right = e
c.right = f

# TESTS
assert breadth_first_sum(a) == 25
assert breadth_first_sum(b) == 17
assert breadth_first_sum(c) == 5
assert breadth_first_sum(d) == 4
assert not breadth_first_sum(None)

assert depth_first_sum_iterative(a) == 25
assert depth_first_sum_iterative(b) == 17
assert depth_first_sum_iterative(c) == 5
assert depth_first_sum_iterative(d) == 4
assert not depth_first_sum_iterative(None)

assert depth_first_sum_recursive(a) == 25
assert depth_first_sum_recursive(b) == 17
assert depth_first_sum_recursive(c) == 5
assert depth_first_sum_recursive(d) == 4
assert not depth_first_sum_recursive(None)

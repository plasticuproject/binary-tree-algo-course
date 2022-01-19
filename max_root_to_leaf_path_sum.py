"""max_root_to_leaf_path_sum.py"""
from timeit import timeit
from typing import Optional


class Node():  # pylint: disable=too-few-public-methods
    """Node class for binary tree structure."""

    def __init__(self, val: Optional[int]) -> None:
        self.val = val
        self.left: Optional[Node] = None
        self.right: Optional[Node] = None


def depth_first_recursive(root: Optional[Node]) -> Optional[int]:
    """Depth-first recursive binary tree traversal. Returns
    Node of maximum root to leaf sumation."""
    value: Optional[int] = None
    left_value: int = 0
    right_value: int = 0
    if not root:
        return value
    value = 0
    if root.val:
        value = root.val
    if not root.left and not root.right:
        return value
    if root.left:
        left_node = depth_first_recursive(root.left)
        if left_node:
            left_value = left_node
    if root.right:
        right_node = depth_first_recursive(root.right)
        if right_node:
            right_value = right_node
    if left_value > right_value:
        value += left_value
    else:
        value += right_value
    return value


# Binary Tree Structure

#     5
#    / \
#   11  3
#  /  \  \
# 4    2  1

a = Node(5)
b = Node(11)
c = Node(3)
d = Node(4)
e = Node(2)
f = Node(1)

a.left = b
a.right = c
b.left = d
b.right = e
c.right = f

# TESTS
assert depth_first_recursive(a) == 20
assert depth_first_recursive(b) == 15
assert depth_first_recursive(c) == 4
assert depth_first_recursive(d) == 4
assert not depth_first_recursive(None)

# TIME TESTS
print("\ndepth_first_recursive:")
print(timeit(lambda: depth_first_recursive(a), number=1000000))

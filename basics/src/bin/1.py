from typing import List

class Node:
    def __init__(self, value):
        self.value = value
        self.l = None
        self.r = None

def insert(root:Node, value:int) -> Node:

    if root == None:
        return Node(value)

    if root.value > value:
        root.l = insert(root.l, value)
    else:
        root.r = insert(root.r, value)

    return root




def make_tree(arr:List[int]) -> Node:
    if not arr:
        return f"Err: array was empty. Nothing to build tree from"
    
    root = Node(arr[0])
    for i in range(1, len(arr)):
        insert(root, arr[i])    

    return root

def lnr(root:Node):
    if root:
        lnr(root.l)
        print(root.value)
        lnr(root.r)

arr = list(map(int, input().split())) 
root = make_tree(arr)
lnr(root)
'''
MY OWN NEW WAY TO EASILYYYYYYY DETECT ALL ERRORS
AND TRACE BACK HOW THEY ORIGINATE


'''






from typing import List

error = [None]*100
def ERROR(n:int):

    error[0]  = "Err: array was empty. Nothing to build tree from"
    error[1]  = "Err: input invalid. Expected int"
    error[2]  = "Err: List index out of Bounds. from"
    error[3]  = "Err: array was empty. Nothing to build tree from"
    error[4]  = "Err: array was empty. Nothing to build tree from"
    error[5]  = "Err: array was empty. Nothing to build tree from"
    error[6]  = "Err: array was empty. Nothing to build tree from"
    error[7]  = "Err: array was empty. Nothing to build tree from"
    error[8]  = "Err: array was empty. Nothing to build tree from"
    error[9]  = "Err: array was empty. Nothing to build tree from"
    error[10] = "Err: array was empty. Nothing to build tree from"
    error[11] = "Err: array was empty. Nothing to build tree from"

    return error[n]






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
        return ERROR(1)
    
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
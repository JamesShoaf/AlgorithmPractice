class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
    

    def __lt__(self, other): return self
    def __le__(self, other): return self
from typing import List
import heapq
from context import ListNode

class Solution:
    def merge_sorted_lists(self, lists:List[ListNode]) -> ListNode:
        try:
            if not lists: return None
            # add all lists to a minheap sorted by the starting node's value
            heap = []
            for index, list in enumerate(lists):
                if list:
                    item = (list.val, index, list)
                    heapq.heappush(heap, item)
            # store a reference to the lowest list to return after merging
            # store the index of the list to give heapq a unique value to compare lists with in case of
            # a tie between node values (not all implementations of linked lists have __lt__ methods)
            val, index, list = heapq.heappop(heap)
            current = list
            # while there are lists to merge
            while len(heap):
                # if the current node has a next value, swap it with a lower one from another list
                next_node = current.next
                if next_node:
                    if next_node.val != val and next_node.val > heap[0][0]:
                        val, index, next_node = heapq.heapreplace(heap, (next_node.val, index, next_node))
                        current.next = next_node
                    else: val = next_node.val
                # if the current node has no next value, merge it with the lowest one from another list
                else:
                    val, index, next_node = heapq.heappop(heap)
                    current.next = next_node
                # if the current node has a next value which is equal to the current value,
                # simply update the current reference
                current = next_node
            return list
        except: return None
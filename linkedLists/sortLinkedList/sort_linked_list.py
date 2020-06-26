from context import ListNode

class Solution:
    def sort_linked_list(self, list:ListNode) -> ListNode:
        try:
            def merge_sort(sub_head:ListNode, length:int) -> ListNode:
                if length == 1:
                    # break the linked list into individual nodes
                    sub_head.next = None
                    return sub_head
                midpoint = int(length / 2)
                mid_head = sub_head
                for i in range(midpoint): mid_head = mid_head.next
                start_head = merge_sort(sub_head, midpoint)
                mid_head = merge_sort(mid_head, length - midpoint)
                # grab the least element from each of the sorted sublists
                if mid_head.val < start_head.val:
                    sorted_list_head = mid_head
                    mid_head = mid_head.next
                else:
                    sorted_list_head = start_head
                    start_head = start_head.next
                current_sorted_list_node = sorted_list_head
                # while there are nodes left in either list, append the least one to the list
                while start_head and mid_head:
                    if mid_head.val < start_head.val:
                        current_sorted_list_node.next = mid_head
                        mid_head = mid_head.next
                    else: 
                        current_sorted_list_node.next = start_head
                        start_head = start_head.next
                    current_sorted_list_node = current_sorted_list_node.next
                # once one list has been exhausted, append the remaining nodes from the other list
                # to the current list
                if start_head: current_sorted_list_node.next = start_head
                else: current_sorted_list_node.next = mid_head
                return sorted_list_head
            node_count = 0
            count_pointer = list
            while count_pointer:
                node_count += 1
                count_pointer = count_pointer.next
            return merge_sort(list, node_count)
        except: return None
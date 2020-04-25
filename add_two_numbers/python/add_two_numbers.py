class ListNode:
    def __init__(self, x, next=None):
        self.val = x
        self.next = next


def add_two_numbers(l1: ListNode, l2: ListNode) -> ListNode:
    """
    Returns a `ListNode` that is the result of adding two positive
    integers that are stored as a `ListNode` in reverse order
    
    Arguments
    
    * `l1` - A `ListNode` contain positive integers in reverse order
    * `l2` - A `ListNode` contain positive integers in reverse order
    
    Approach
    
    This solution adds each node value of both lists starting with the
    least significant digits. If an addition operation results in overflow,
    use that as the carry for the next operation.
    """

    result_head = ListNode(-1)
    result_current = result_head
    l1_current, l2_current = l1, l2
    carry = 0
    while l1_current or l2_current:
        x = l1_current.val if l1_current else 0
        y = l2_current.val if l2_current else 0
        sum = x + y + carry
        carry = sum // 10

        result_current.next = ListNode(sum % 10)
        result_current = result_current.next

        l1_current = l1_current.next if l1_current else None
        l2_current = l2_current.next if l2_current else None

    if carry == 1:
        result_current.next = ListNode(carry)
    
    return result_head.next

def test_add_two_numbers():
    l1 = ListNode(2, ListNode(4, ListNode(3)))
    l2 = ListNode(5, ListNode(6, ListNode(4)))
    result = ListNode(7, ListNode(0, ListNode(8)))
    output = add_two_numbers(l1, l2)

    while result and output:
        assert result.val == output.val
        result = result.next
        output = output.next
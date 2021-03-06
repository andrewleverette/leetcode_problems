#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

/// Returns a linked list that is the result adding two numbers together,
/// numbers are in reverse order.
///
/// # Arguments
///
/// * `l1` - A linked list that represents a number in reverse order.
/// * `l2` - A linked list that represents a number in reverse order.
///
/// # Approach
///
/// This solution uses an iterative approach to add the value of two nodes together
/// in each iteration. The `carry` is used to add the overflow from one operation
/// to the next. If one linked list reaches the end, then it's reference is ignored
/// until the other linked list is fully processed.
pub fn add_two_numbers_iteratively(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1_current = l1;
    let mut l2_current = l2;
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = head.as_mut();
    let mut carry = 0;

    while l1_current.is_some() || l2_current.is_some() {
        let mut sum = carry;

        if let Some(node) = l1_current {
            sum += node.val;
            l1_current = node.next;
        }

        if let Some(node) = l2_current {
            sum += node.val;
            l2_current = node.next;
        }

        carry = sum / 10;

        if let Some(node) = current {
            node.next = Some(Box::new(ListNode::new(sum % 10)));
            current = node.next.as_mut();
        }
    }

    if carry > 0 {
        current.unwrap().next = Some(Box::new(ListNode::new(carry)));
    }

    head.unwrap().next
}

/// Returns a linked list that is the result adding two numbers together,
/// numbers are in reverse order.
///
/// # Arguments
///
/// * `l1` - A linked list that represents a number in reverse order.
/// * `l2` - A linked list that represents a number in reverse order.
///
/// # Approach
///
/// This solution uses recursive approach to add the value to two nodes together.
pub fn add_two_numbers_recursively(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add(l1, l2, 0)
}

/// Recursively builds the linked list
///
/// # Arguments
///
/// * `l1` - A linked list that represents a number in reverse order.
/// * `l2` - A linked list that represents a number in reverse order.
/// * `carry` - A number that is the overflow from the addition operation
///
/// # Approach
///
/// This method uses the same logic as the iterative approach except it recursively
/// adds the next digit to the linked list. The benefit is that a dummy node is not
/// needed. The root node can simply be returned.
fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    let mut sum = carry;

    let l1_current = if let Some(node) = l1 {
        sum += node.val;
        node.next
    } else {
        None
    };

    let l2_current = if let Some(node) = l2 {
        sum += node.val;
        node.next
    } else {
        None
    };

    let mut result = ListNode::new(sum % 10);

    if l1_current.is_some() || l2_current.is_some() || sum >= 10 {
        result.next = add(l1_current, l2_current, sum / 10);
    }

    Some(Box::new(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers_iteratively() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let result = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));

        let output = add_two_numbers_iteratively(l1, l2);

        assert_eq!(result, output);
    }

    #[test]
    fn test_add_two_numbers_recursively() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let result = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));

        let output = add_two_numbers_recursively(l1, l2);

        assert_eq!(result, output);
    }
}

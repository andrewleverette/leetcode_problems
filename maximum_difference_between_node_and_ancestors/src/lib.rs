use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, max: i32, min: i32) -> i32 {
            if let Some(node) = node {
                let n = node.borrow();
                let current_max = n.val.max(max);
                let current_min = n.val.min(min);
    
                let left = helper(&n.left, current_max, current_min);
                let right = helper(&n.right, current_max, current_min);
    
                left.max(right)
            } else {
                max - min
            }
        }
        
        let current_max = node.borrow().val;
        let current_min = node.borrow().val;

        helper(&Some(node), current_max, current_min)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let root = TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 14,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                    right: None,
                }))),
            }))),
        };

        assert_eq!(max_ancestor_diff(Some(Rc::new(RefCell::new(root)))), 7);
    }

    #[test]
    fn test_example_2() {
        let root = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None,
                })))
            })))
        };

        assert_eq!(max_ancestor_diff(Some(Rc::new(RefCell::new(root)))), 3);
    }
}

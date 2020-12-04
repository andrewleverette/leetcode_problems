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

pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let mut values = traverse(&Some(node));
        values.reverse();
        tree_builder(values)
    } else {
        None
    }
}

fn tree_builder(mut values: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(val) = values.pop() {
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.borrow_mut().right = tree_builder(values);
        Some(node)
    } else {
        None
    }
}

fn traverse(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut values = Vec::new();

    if let Some(node) = root {
        let n = node.borrow();
        values.extend(traverse(&n.left));
        values.push(n.val);
        values.extend(traverse(&n.right));
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 6,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode {
                                    val: 7,
                                    left: None,
                                    right: Some(Rc::new(RefCell::new(TreeNode {
                                        val: 8,
                                        left: None,
                                        right: Some(Rc::new(RefCell::new(TreeNode {
                                            val: 9,
                                            left: None,
                                            right: None
                                        }))),
                                    }))),
                                }))),
                            }))),
                        }))),
                    }))),
                }))),
            }))),
        })));

        assert_eq!(increasing_bst(root), expected);
    }
}

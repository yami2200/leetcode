use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

// ################# BRUTE FORCE
pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    match root {
        Some(n) => {
            let t_n = n.borrow();
            let mut d_x = get_value_depth(&t_n.left, x, 1, t_n.val);
            if d_x.0 == -1 { d_x = get_value_depth(&t_n.right, x, 1, t_n.val); }
            let mut d_y = get_value_depth(&t_n.left, y, 1, t_n.val);
            if d_y.0 == -1 { d_y = get_value_depth(&t_n.right, y, 1, t_n.val); }
            d_x.0 != -1 && d_x.0 == d_y.0 && d_x.1 != d_y.1
        }
        None => { false }
    }
}

pub fn get_value_depth(root: &Option<Rc<RefCell<TreeNode>>>, v: i32, d: i32, p_v: i32) -> (i32, i32) {
    match root {
        Some(n) => {
            let t_n = n.borrow();
            if t_n.val == v { return (d, p_v) }
            let left = get_value_depth(&t_n.left, v, d+1, t_n.val);
            let right = get_value_depth(&t_n.right, v, d+1, t_n.val);
            if left.0 > right.0 { return left }
            right
        }
        None => { (-1, -1) }
    }
}
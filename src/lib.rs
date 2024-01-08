#![allow(dead_code, unused_variables)]
mod p0020_valid_parentheses;
mod p1535_find_the_winner_of_an_array_game;
mod p0001_two_sum;
mod p0021_merge_two_sorted_lists;
mod p1662_check_if_two_string_arrays_are_equivalent;
mod p1155_number_of_dice_rolls_with_target_sum;
mod p0938_range_sum_of_bst;
mod p0121_best_time_to_buy_and_sell_stock;

use std::rc::Rc;
use std::cell::RefCell;

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

mod p0125_valid_palindrome;

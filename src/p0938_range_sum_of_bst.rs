// 
// [938] Range Sum of BST
// 
// 
// Given the root node of a binary search tree and two integers low and high, return the sum of values of all nodes with a value in the inclusive range [low, high].
// 
//  
// Example 1:
// 
// Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
// Output: 32
// Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.
// 
// 
// Example 2:
// 
// Input: root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
// Output: 23
// Explanation: Nodes 6, 7, and 10 are in the range [6, 10]. 6 + 7 + 10 = 23.
// 
// 
//  
// Constraints:
// 
// 
// 	The number of nodes in the tree is in the range [1, 2 * 10⁴].
// 	1 <= Node.val <= 10⁵
// 	1 <= low <= high <= 10⁵
// 	All Node.val are unique.
// 
// 
struct Solution;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::nodes::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let mut sum = 0;
                let mut node = node.borrow_mut();
                if low <= node.val && node.val <= high {
                    sum += node.val;
                }
                if low < node.val {
                    sum += Self::range_sum_bst(node.left.take(), low, high);
                }
                if node.val < high {
                    sum += Self::range_sum_bst(node.right.take(), low, high);
                }
                sum
            }
        }
        
    }
}

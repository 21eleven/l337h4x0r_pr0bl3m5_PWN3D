// 
// [206] Reverse Linked List
// 
// 
// Given the head of a singly linked list, reverse the list, and return the reversed list.
// 
//  
// Example 1:
// 
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]
// 
// 
// Example 2:
// 
// Input: head = [1,2]
// Output: [2,1]
// 
// 
// Example 3:
// 
// Input: head = []
// Output: []
// 
// 
//  
// Constraints:
// 
// 
// 	The number of nodes in the list is the range [0, 5000].
// 	-5000 <= Node.val <= 5000
// 
// 
//  
// Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?
// 
struct Solution;
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use crate::nodes::ListNode;
use crate::nodes::test::{mk_ll, print_ll};
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!();
    }
}
#[test]
fn test_linked_list_del() {
    // need to make a util to create LL nodes
    let a = mk_ll(vec![1]);
    let b = Solution::reverse_list(a);
    print_ll(b);
    let a = mk_ll(vec![1, 2]);
    let b = Solution::reverse_list(a);
    print_ll(b);
    let a = mk_ll(vec![1,2,3]);
    let b = Solution::reverse_list(a);
    print_ll(b);
    let a = mk_ll(vec![1,2,3,4]);
    let b = Solution::reverse_list(a);
    print_ll(b);
    let a = mk_ll(vec![1,2,3,4,5]);
    let b = Solution::reverse_list(a);
    print_ll(b);
}

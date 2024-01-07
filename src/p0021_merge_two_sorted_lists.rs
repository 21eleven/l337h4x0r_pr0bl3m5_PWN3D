#![allow(dead_code, unused_variables)]
//
// [21] Merge Two Sorted Lists
//
//
// You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two li
// sts.
//
// Return the head of the merged linked list.
//
//  
// Example 1:
//
// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]
//
//
// Example 2:
//
// Input: list1 = [], list2 = []
// Output: []
//
//
// Example 3:
//
// Input: list1 = [], list2 = [0]
// Output: [0]
//
//
//  
// Constraints:
//
//
//         The number of nodes in both lists is in the range [0, 50].
//         -100 <= Node.val <= 100
//         Both list1 and list2 are sorted in non-decreasing order.
//
// error: error decoding response body: expected value at line 1 column 1, please try again
struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        // let x = &mut head;
        // let mut a = list1;
        // let mut b = list2;

        fn recurse(ptr: &mut ListNode, mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) {
            match (a, b) {
                (Some(x), Some(y)) => {
                    if x.val <= y.val {
                        let mut new_node = ListNode::new(x.val);
                        recurse(&mut new_node, x.next, Some(y));
                        ptr.next = Some(Box::new(new_node));
                    } else {
                        let mut new_node = ListNode::new(y.val);
                        recurse(&mut new_node, Some(x), y.next);
                        ptr.next = Some(Box::new(new_node));
                    }
                }
                (Some(x), None) => {
                    ptr.next = Some(x);
                }
                (None, Some(y)) => {
                    ptr.next = Some(y);
                }
                (None, None) => {
                    // nothing to od
                }
            }

        }
        recurse(&mut head, list1, list2);
        head.next

    }
}

#[test]
fn test_linked_list_merge() {
    // need to make a util to create LL nodes
    fn mk_ll(nodes: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let vals = nodes.into_iter().rev().collect::<Vec<i32>>();
        fn recurse(node: &mut ListNode, mut vals: Vec<i32>) {
            if let Some(x) = vals.pop() {
                let mut new = ListNode::new(x);
                recurse(&mut new, vals);
                node.next = Some(Box::new(new));
            }
        }
        recurse(&mut head, vals);
        head.next
    }
    let a = mk_ll(vec![1,3,5]);
    dbg!(&a);
    let b = mk_ll(vec![2,4,6]);
    dbg!(&b);
    dbg!(Solution::merge_two_lists(a, b));
}

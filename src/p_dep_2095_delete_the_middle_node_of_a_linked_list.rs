// 
// [2095] Delete the Middle Node of a Linked List
// 
// 
// You are given the head of a linked list. Delete the middle node, and return the head of the modified linked list.
// 
// The middle node of a linked list of size n is the ⌊n / 2⌋th node from the start using 0-based indexing, where ⌊x⌋ denotes the largest integer less than or equal to x.
// 
// 
// 	For n = 1, 2, 3, 4, and 5, the middle nodes are 0, 1, 1, 2, and 2, respectively.
// 
// 
//  
// Example 1:
// 
// Input: head = [1,3,4,7,1,2,6]
// Output: [1,3,4,1,2,6]
// Explanation:
// The above figure represents the given linked list. The indices of the nodes are written below.
// Since n = 7, node 3 with value 7 is the middle node, which is marked in red.
// We return the new list after removing this node. 
// 
// 
// Example 2:
// 
// Input: head = [1,2,3,4]
// Output: [1,2,4]
// Explanation:
// The above figure represents the given linked list.
// For n = 4, node 2 with value 3 is the middle node, which is marked in red.
// 
// 
// Example 3:
// 
// Input: head = [2,1]
// Output: [2]
// Explanation:
// The above figure represents the given linked list.
// For n = 2, node 1 with value 1 is the middle node, which is marked in red.
// Node 0 with value 2 is the only node remaining after removing node 1.
// 
//  
// Constraints:
// 
// 
// 	The number of nodes in the list is in the range [1, 10⁵].
// 	1 <= Node.val <= 10⁵
// 
// 
use crate::nodes::ListNode;
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
impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // match head {
        //     None => 0,
        //     Some(Node)
        // }
        // head
        fn recurse(prev: &mut Box<ListNode>, curr: &mut Option<Box<ListNode>>, len: usize) -> usize {
            match curr {
                None => len / 2,
                Some(ref mut node) => {
                    let idx = len;
                    let mut nxt = node.next.take();
                    let middle_idx = recurse(node, &mut nxt, idx+1); 
                    dbg!(middle_idx, idx, &node, &prev);
                    if middle_idx == idx {
                        prev.next = nxt;
                    } else {
                        node.next = nxt;
                    }
                    middle_idx

                }
            }

        }
        let mut head = head.unwrap();
        if head.next.is_some() {
            let mut curr = head.next.take();
            recurse(&mut head, &mut curr, 1);
            dbg!(&curr);
            head.next = curr;
            return Some(head)
        }
        None
        
    }
}
#[test]
fn test_linked_list_del() {
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
    fn print_ll(head: Option<Box<ListNode>>) {
        let mut nums: Vec<i32> = vec![];
        fn recurse(node: Option<Box<ListNode>>) -> Vec<i32> {
            match node {
                None => vec![],
                Some(n) => {
                    let mut nums = vec![n.val];
                    nums.append(&mut recurse(n.next));
                    nums
                }
            }
        }
        println!("{:?}", recurse(head));
    }
    // let a = mk_ll(vec![1]);
    // let b = Solution::delete_middle(a);
    // print_ll(b);
    // let a = mk_ll(vec![1, 2]);
    // let b = Solution::delete_middle(a);
    // print_ll(b);
    // let a = mk_ll(vec![1,2,3]);
    // let b = Solution::delete_middle(a);
    // print_ll(b);
    let a = mk_ll(vec![1,2,3,4]);
    let b = Solution::delete_middle(a);
    print_ll(b);
}

// #[test]
fn rand_test() {
    let x = 8;
    let y = &x;

    let x = 9;
    dbg!(x, y);
}

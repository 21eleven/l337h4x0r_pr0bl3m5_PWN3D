use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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


pub mod test {
    use super::ListNode;
    pub fn mk_ll(nodes: Vec<i32>) -> Option<Box<ListNode>> {
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
    pub fn print_ll(head: Option<Box<ListNode>>) {
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
}

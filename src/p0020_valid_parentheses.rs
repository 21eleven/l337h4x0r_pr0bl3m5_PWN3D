//
// [20] Valid Parentheses is on the run...
//
//
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
//
// 	Open brackets must be closed by the same type of brackets.
// 	Open brackets must be closed in the correct order.
// 	Every close bracket has a corresponding open bracket of the same type.
//
//
//
// Example 1:
//
// Input: s = "()"
// Output: true
//
//
// Example 2:
//
// Input: s = "()[]{}"
// Output: true
//
//
// Example 3:
//
// Input: s = "(]"
// Output: false
//
//
//
// Constraints:
//
//
// 	1 <= s.length <= 10⁴
// 	s consists of parentheses only '()[]{}'.
//
//

#[derive(Debug, PartialEq, Eq)]
enum Bracket {
    A,
    Aa,
    B,
    Bb,
    C,
    Cc,
}

use Bracket::*;

impl From<char> for Bracket {
    fn from(c: char) -> Self {
        match c.to_ascii_lowercase() {
            '(' => A,
            ')' => Aa,
            '[' => B,
            ']' => Bb,
            '{' => C,
            '}' => Cc,
            _ => panic!("ahhh: {c}"),
        }
    }
}

enum BracketType {
    Open(Bracket),
    Close(Bracket),
}
use BracketType::*;
impl From<Bracket> for BracketType {
    fn from(bra: Bracket) -> Self {
        match bra {
            A => Open(A),
            B => Open(B),
            C => Open(C),
            Aa => Close(Aa),
            Bb => Close(Bb),
            Cc => Close(Cc),
        }
    }
}

enum Err {
    NoMatch,
    OpenDelimiterLeftover,
}
impl Bracket {
    fn matches(self, other: Bracket) -> Result<(), Err> {
        match (self, other) {
            (A, Aa) | (B, Bb) | (C, Cc) => Ok(()),
            _ => Err(Err::NoMatch),
        }
    }
}

impl From<char> for BracketType {
    fn from(c: char) -> Self {
        BracketType::from(Bracket::from(c))
    }
}
struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn valid(s: String) -> Result<(), Err> {
            let stack =
                s.chars()
                    .map(|c| BracketType::from(c))
                    .try_fold(vec![], |mut stack, paren| match paren {
                        Open(bracket) => {
                            stack.push(bracket);
                            Ok(stack)
                        }
                        Close(right) => {
                            if let Some(left) = stack.pop() {
                                left.matches(right).map(|_| stack)
                            } else {
                                Err(Err::NoMatch)
                            }
                        }
                    })?;
            if stack.len() != 0 {
                return Err(Err::OpenDelimiterLeftover);
            }
            Ok(())
        }

        valid(s).is_ok()
    }
}

#[test]
fn t1() {
    assert!(true);
    assert_eq!(<char as Into<Bracket>>::into('('), A);
    // good
    assert_eq!(Solution::is_valid("({})".to_string()), true);
    // bad
    assert_ne!(Solution::is_valid("(})".to_string()), true);
    assert_ne!(Solution::is_valid("(".to_string()), true);
}

/*
Success

Runtime: 1 ms, faster than 39% of Rust online submissions for Valid Parentheses.

Memory Usage: 2.1 MB, less than 72% of Rust Valid Parentheses.
*/

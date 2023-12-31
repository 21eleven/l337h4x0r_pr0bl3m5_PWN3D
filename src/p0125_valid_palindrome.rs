// 
// [125] Valid Palindrome
// 
// 
// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
// 
// Given a string s, return true if it is a palindrome, or false otherwise.
// 
//  
// Example 1:
// 
// Input: s = "A man, a plan, a canal: Panama"
// Output: true
// Explanation: "amanaplanacanalpanama" is a palindrome.
// 
// 
// Example 2:
// 
// Input: s = "race a car"
// Output: false
// Explanation: "raceacar" is not a palindrome.
// 
// 
// Example 3:
// 
// Input: s = " "
// Output: true
// Explanation: s is an empty string "" after removing non-alphanumeric characters.
// Since an empty string reads the same forward and backward, it is a palindrome.
// 
// 
//  
// Constraints:
// 
// 
// 	1 <= s.length <= 2 * 10⁵
// 	s consists only of printable ASCII characters.
// 
// 
struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase());
        while let (Some(a), Some(b)) = (chars.next(), chars.next_back()) {
            if a != b {
                return false
            }
        }
        true
    }
}

#[test]
fn chk() {
    assert!(Solution::is_palindrome("OP".to_string()) == false);
    assert!(Solution::is_palindrome("0P".to_string()) == false);
    assert!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()) == true);
}

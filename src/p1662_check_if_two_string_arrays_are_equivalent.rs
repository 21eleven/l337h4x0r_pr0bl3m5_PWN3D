/*
[1662] Check If Two String Arrays are Equivalent

Given two string arrays word1 and word2, return true if the two arrays represent the same string, and false otherwise.

A string is represented by an array if the array elements concatenated in order forms the string.


Example 1:
Input: word1 = ["ab", "c"], word2 = ["a", "bc"]
Output: true
Explanation:
word1 represents string "ab" + "c" -> "abc"
word2 represents string "a" + "bc" -> "abc"
The strings are the same, so return true.

Example 2:
Input: word1 = ["a", "cb"], word2 = ["ab", "c"]
Output: false


Example 3:
Input: word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
Output: true


Constraints:
	1 <= word1.length, word2.length <= 10³
	1 <= word1[i].length, word2[i].length <= 10³
	1 <= sum(word1[i].length), sum(word2[i].length) <= 10³
	word1[i] and word2[i] consist of lowercase letters.
*/

struct Solution;
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1
            .into_iter()
            .flat_map(|w| w.chars().collect::<Vec<_>>())
            .eq(word2
                .into_iter()
                .flat_map(|w| w.chars().collect::<Vec<_>>()))
    }
}

#[test]
fn strs_eq() {
    let word1 = vec!["abc".to_string(), "d".to_string(), "defg".to_string()];
    let word2 = vec!["abcddefg".to_string()];

    assert!(Solution::array_strings_are_equal(word1, word2));
}

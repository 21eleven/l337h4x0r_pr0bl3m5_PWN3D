// 
// [648] Replace Words
// 
// 
// In English, we have a concept called root, which can be followed by some other word to form another longer word - let's call this word derivative. For example, when the root "help" is followed by the word "ful", we can form a derivative "helpful".
// 
// Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces, replace all the derivatives in the sentence with the root forming it. If a derivative can be replaced by more than one root, replace it with the root that has the shortest length.
// 
// Return the sentence after the replacement.
// 
//  
// Example 1:
// 
// Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled by the battery"
// Output: "the cat was rat by the bat"
// 
// 
// Example 2:
// 
// Input: dictionary = ["a","b","c"], sentence = "aadsfasf absbs bbab cadsfafs"
// Output: "a a b c"
// 
// 
//  
// Constraints:
// 
// 
// 	1 <= dictionary.length <= 1000
// 	1 <= dictionary[i].length <= 100
// 	dictionary[i] consists of only lower-case letters.
// 	1 <= sentence.length <= 10⁶
// 	sentence consists of only lower-case letters and spaces.
// 	The number of words in sentence is in the range [1, 1000]
// 	The length of each word in sentence is in the range [1, 1000]
// 	Every two consecutive words in sentence will be separated by exactly one space.
// 	sentence does not have leading or trailing spaces.
// 
// 
#![allow(unused_imports)]
struct Solution;
use std::boxed::Box;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    terminal: bool,
    childs: [Option<Box<Node>>; 26]
}

impl Node {
    fn new() -> Self {
        // let arr: [Option<Box<Node>>; 26] = [None; 26];
        Node { terminal: false, childs: std::array::from_fn(|_| None)}

    }
    fn insert(&mut self, mut string: Vec<char>) {
        if let Some(c) = string.pop() {
            match self.childs[ascii(c)] {
                // Some(ref mut child) => {
                Some(ref mut child) => {
                    child.insert(string)
                }
                None => {
                    let mut child = Node::new();
                    child.insert(string);
                    self.childs[ascii(c)] = Some(Box::new(child))
                }
            }
        } else {
            // end of word
            self.terminal = true;
        }
    }

    fn find_prefix(&self, mut root: Vec<char>, mut search: Vec<char>) -> Option<String> {
        if self.terminal {
            return Some(root.into_iter().collect::<String>())
        } else {
            match search.pop() {
                None => None,
                Some(c) => {
                    if let Some(node) = &self.childs[ascii(c)] {
                        root.push(c);
                        node.find_prefix(root, search)
                    } else {
                        None
                    }
                }
            }
        } 
    }
}

#[test]
fn node_insert() {
    let mut subject = Node::new();
    subject.insert(vec!['c', 'b', 'a']);
    let mut root = Node::new();
    let mut a = Box::new(Node::new());
    let mut b = Box::new(Node::new());
    let mut c = Box::new(Node::new());
    c.terminal = true;
    b.childs[2] = Some(c);
    a.childs[1] = Some(b);
    root.childs[0] = Some(a);
    assert_eq!(subject, root);
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Trie {
    head: Node,
}

impl Trie {
    fn new() -> Self {
        let root = Node::new();
        Trie { head: root }
    }
    fn insert(&mut self, string: &str) {
        self.head.insert(string.chars().rev().collect::<Vec<char>>())
    }
    fn get_root(&self, word: String) -> String {
        let chars = word.chars().rev().collect::<Vec<char>>();
        match self.head.find_prefix(vec![], chars) {
            None => word,
            Some(prefix) => prefix,
        }
    }
    
}

#[test]
fn test_search() {
    let mut trie = Trie::new();
    let expected = "abc";
    trie.insert(expected);
    assert_eq!(trie.get_root("abcd".to_string()), expected);
    assert_eq!(trie.get_root("a".to_string()), "a");
    assert_eq!(trie.get_root("z".to_string()), "z");
}

fn ascii(c: char) -> usize {
    c as usize - 97
}

#[test]
fn test_ascii() {
    assert!(ascii('a') == 0);
    assert!(ascii('z') == 25);
}

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        dictionary.into_iter().for_each(|w| trie.insert(&w));
        sentence.split_whitespace().map(|w| trie.get_root(w.to_string())).collect::<Vec<String>>().join(" ")
    }
}
